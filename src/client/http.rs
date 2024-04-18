use crate::common::Page;
use crate::context::{GetRef, GitlabUrl};
use crate::query::get_project_list::GetProjectListQuery;
use crate::query::get_project_list_page::GetProjectListPageQuery;
use crate::url_util::UrlExt;
use crate::Project;
use bytes::Bytes;
use compact_str::CompactString;
use core::task::{Context, Poll};
use futures::future::BoxFuture;
use headers_link::link::{Link, RelationType};
use headers_link::Header;
use http::{HeaderMap, Method, Request, Response};
use http_body::Body;
use http_body_util::{BodyExt, Empty};
use std::error::Error as StdError;
use std::str::FromStr;
use tower_service::Service;

pub struct HttpGitlabClient<TyInner> {
  inner: TyInner,
}

impl<TyInner> HttpGitlabClient<TyInner> {
  pub fn new(inner: TyInner) -> Self {
    Self { inner }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum HttpGitlabClientError {
  #[error("failed to poll ready status: {0}")]
  PollReady(String),
  #[error("failed to send request: {0}")]
  Send(String),
  #[error("failed to receive response: {0}")]
  Receive(String),
  #[error("failed to parse response: {0}")]
  ResponseFormat(String, Bytes),
  #[error("operation is forbidden for provided auth")]
  Forbidden,
  #[error("unexpected error: {0}")]
  Other(String),
}

impl<'req, Cx, TyInner, TyBody> Service<&'req GetProjectListQuery<Cx>> for HttpGitlabClient<TyInner>
where
  Cx: GetRef<GitlabUrl>,
  TyInner: Service<Request<Empty<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = Page<Project>;
  type Error = HttpGitlabClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpGitlabClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: &'req GetProjectListQuery<Cx>) -> Self::Future {
    let mut url = req.context.get_ref().url_join(["projects"]);

    {
      let mut query = url.query_pairs_mut();
      if let Some(owned) = req.owned {
        query.append_pair("owned", if owned { "true" } else { "false" });
      }
    }

    let mut http_req = Request::builder().method(Method::GET).uri(url.as_str());

    if let Some(auth) = req.auth.as_ref() {
      let (key, value) = auth.http_header();
      http_req = http_req.header(key, value);
    }
    let req = http_req.body(Empty::new()).unwrap();
    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpGitlabClientError::Send(format!("{e:?}")))?;
      let cursors = get_cursors(res.headers());
      let body = res
        .into_body()
        .collect()
        .await
        .map_err(|e| HttpGitlabClientError::Receive(format!("{e:?}")))?;
      let body: Bytes = body.to_bytes();

      let body: Vec<Project> =
        serde_json::from_slice(&body).map_err(|e| HttpGitlabClientError::ResponseFormat(format!("{e:?}"), body))?;
      Ok(Page {
        first: cursors.first,
        next: cursors.next,
        items: body,
      })
    })
  }
}

impl<'req, Cx, TyInner, TyBody> Service<&'req GetProjectListPageQuery<Cx>> for HttpGitlabClient<TyInner>
where
  TyInner: Service<Request<Empty<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = Page<Project>;
  type Error = HttpGitlabClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpGitlabClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: &'req GetProjectListPageQuery<Cx>) -> Self::Future {
    let mut url = &req.cursor;

    let mut http_req = Request::builder().method(Method::GET).uri(url.as_str());
    if let Some(auth) = req.auth.as_ref() {
      let (key, value) = auth.http_header();
      http_req = http_req.header(key, value);
    }
    let req = http_req.body(Empty::new()).unwrap();

    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpGitlabClientError::Send(format!("{e:?}")))?;
      let cursors = get_cursors(res.headers());
      let body = res
        .into_body()
        .collect()
        .await
        .map_err(|e| HttpGitlabClientError::Receive(format!("{e:?}")))?;
      let body: Bytes = body.to_bytes();

      let body: Vec<Project> =
        serde_json::from_slice(&body).map_err(|e| HttpGitlabClientError::ResponseFormat(format!("{e:?}"), body))?;
      Ok(Page {
        first: cursors.first,
        next: cursors.next,
        items: body,
      })
    })
  }
}

struct Cursors<Str> {
  first: Option<Str>,
  next: Option<Str>,
}

fn get_cursors(headers: &HeaderMap) -> Cursors<CompactString> {
  let mut next: Option<CompactString> = None;
  let mut first: Option<CompactString> = None;
  for link in headers.get_all(Link::name().as_str()) {
    let link = match link.to_str() {
      Ok(l) => l,
      Err(_) => continue,
    };
    let link = match Link::from_str(link) {
      Ok(l) => l,
      Err(_) => continue,
    };
    for value in link.iter() {
      let rel = match value.rel() {
        Some(rel) => rel,
        None => continue,
      };
      for r in rel {
        // todo: detect when there are multiple different links for the same rel type
        if *r == RelationType::FIRST {
          first = Some(CompactString::new(value.link()));
        }
        if *r == RelationType::NEXT {
          next = Some(CompactString::new(value.link()));
        }
      }
    }
  }
  Cursors { first, next }
}
