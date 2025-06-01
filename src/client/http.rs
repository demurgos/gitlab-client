use crate::command::create_release::CreateReleaseCommand;
use crate::command::create_release_link::CreateReleaseLinkCommand;
use crate::command::publish_package_file::PublishPackageFileCommand;
use crate::common::package::GenericPackageFile;
use crate::common::release::{InputReleaseAssetsView, Release, ReleaseLink};
use crate::common::tree::TreeRecord;
use crate::common::Page;
use crate::context::{GetRef, GitlabUrl};
use crate::query::get_package_file::GetPackageFileQuery;
use crate::query::get_project::GetProjectQuery;
use crate::query::get_project_list::GetProjectListQuery;
use crate::query::get_project_list_page::GetProjectListPageQuery;
use crate::query::get_project_release::GetProjectReleaseQuery;
use crate::query::get_tree_record_list::GetTreeRecordListQuery;
use crate::url_util::UrlExt;
use crate::{GitlabAuth, GitlabAuthView, InputPackageStatus, Project};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use compact_str::CompactString;
use core::task::{Context, Poll};
use demurgos_headers::link::{Link, RelationType};
use demurgos_headers::UserAgent;
use futures::future::BoxFuture;
use http::header::CONTENT_TYPE;
use http::{HeaderMap, Method, Request, Response, StatusCode};
use http_body::Body;
use http_body_util::{BodyExt, Full};
use std::error::Error as StdError;
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
  #[error("resource already exists")]
  Conflict,
  #[error("resource not found")]
  NotFound,
  #[error("unexpected error: {0}")]
  Other(String),
}

impl<'req, Cx, TyInner, TyBody> Service<&'req GetProjectListQuery<Cx>> for HttpGitlabClient<TyInner>
where
  Cx: GetRef<GitlabUrl> + GetRef<UserAgent>,
  TyInner: Service<Request<Full<Bytes>>, Response = Response<TyBody>> + 'req,
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
    let mut url = GetRef::<GitlabUrl>::get_ref(&req.context).url_join(["projects"]);

    {
      let mut query = url.query_pairs_mut();
      if let Some(owned) = req.owned {
        query.append_pair("owned", owned.as_str());
      }
    }

    let req = Request::builder()
      .method(Method::GET)
      .uri(url.as_str())
      .user_agent(GetRef::<UserAgent>::get_ref(&req.context))
      .gitlab_auth(req.auth.as_ref().map(GitlabAuth::as_view))
      .body(Full::new(Bytes::new()))
      .unwrap();
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
        last: cursors.last,
        items: body,
      })
    })
  }
}

impl<'req, Cx, TyInner, TyBody> Service<&'req GetProjectListPageQuery<Cx>> for HttpGitlabClient<TyInner>
where
  TyInner: Service<Request<Full<Bytes>>, Response = Response<TyBody>> + 'req,
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
    let url = &req.cursor;

    let req = Request::builder()
      .method(Method::GET)
      .uri(url.as_str())
      .gitlab_auth(req.auth.as_ref().map(GitlabAuth::as_view))
      .body(Full::new(Bytes::new()))
      .unwrap();

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
        last: cursors.last,
        items: body,
      })
    })
  }
}

impl<'req, Cx, TyInner, TyBody> Service<&'req GetTreeRecordListQuery<Cx>> for HttpGitlabClient<TyInner>
where
  Cx: GetRef<GitlabUrl>,
  TyInner: Service<Request<Full<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = Page<TreeRecord>;
  type Error = HttpGitlabClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpGitlabClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: &'req GetTreeRecordListQuery<Cx>) -> Self::Future {
    let mut url = req.project.with_str(|project| {
      req
        .context
        .get_ref()
        .url_join(["projects", project, "repository", "tree"])
    });
    {
      let mut query = url.query_pairs_mut();
      if let Some(recursive) = req.recursive {
        query.append_pair("recursive", recursive.as_str());
      }
      if let Some(git_ref) = req.r#ref.as_deref() {
        query.append_pair("ref", git_ref);
      }
    }

    let req = Request::builder()
      .method(Method::GET)
      .uri(url.as_str())
      .gitlab_auth(req.auth.as_ref().map(GitlabAuth::as_view))
      .body(Full::new(Bytes::new()))
      .unwrap();

    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpGitlabClientError::Send(format!("{e:?}")))?;
      let cursors = get_cursors(dbg!(res.headers()));
      let body = res
        .into_body()
        .collect()
        .await
        .map_err(|e| HttpGitlabClientError::Receive(format!("{e:?}")))?;
      let body: Bytes = body.to_bytes();

      // dbg!(std::str::from_utf8(&body));

      let body: Vec<TreeRecord> =
        serde_json::from_slice(&body).map_err(|e| HttpGitlabClientError::ResponseFormat(format!("{e:?}"), body))?;
      Ok(Page {
        first: cursors.first,
        next: cursors.next,
        last: cursors.last,
        items: body,
      })
    })
  }
}

impl<'req, Cx, TyInner, TyBody> Service<&'req GetProjectQuery<Cx>> for HttpGitlabClient<TyInner>
where
  Cx: GetRef<GitlabUrl> + GetRef<UserAgent>,
  TyInner: Service<Request<Full<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = Project;
  type Error = HttpGitlabClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpGitlabClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: &'req GetProjectQuery<Cx>) -> Self::Future {
    let mut url = req
      .id
      .with_str(|id| GetRef::<GitlabUrl>::get_ref(&req.context).url_join(["projects", id]));

    {
      let mut query = url.query_pairs_mut();
      if let Some(license) = req.license {
        query.append_pair("license", license.as_str());
      }
      if let Some(statistics) = req.statistics {
        query.append_pair("statistics", statistics.as_str());
      }
      if let Some(with_custom_attributes) = req.with_custom_attributes {
        query.append_pair("with_custom_attributes", with_custom_attributes.as_str());
      }
    }

    let req = Request::builder()
      .method(Method::GET)
      .uri(url.as_str())
      .user_agent(GetRef::<UserAgent>::get_ref(&req.context))
      .gitlab_auth(req.auth.as_ref().map(GitlabAuth::as_view))
      .body(Full::new(Bytes::new()))
      .unwrap();

    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpGitlabClientError::Send(format!("{e:?}")))?;
      let body = res
        .into_body()
        .collect()
        .await
        .map_err(|e| HttpGitlabClientError::Receive(format!("{e:?}")))?;
      let body: Bytes = body.to_bytes();

      let body: Project =
        serde_json::from_slice(&body).map_err(|e| HttpGitlabClientError::ResponseFormat(format!("{e:?}"), body))?;
      Ok(body)
    })
  }
}

impl<'req, Cx, TyInner, TyBody> Service<&'req GetPackageFileQuery<Cx>> for HttpGitlabClient<TyInner>
where
  Cx: GetRef<GitlabUrl>,
  TyInner: Service<Request<Full<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = Bytes;
  type Error = HttpGitlabClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpGitlabClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: &'req GetPackageFileQuery<Cx>) -> Self::Future {
    let url = req.project.with_str(|project| {
      req.context.get_ref().url_join([
        "projects",
        project,
        "packages",
        "generic",
        &req.package_name,
        &req.package_version,
        &req.filename,
      ])
    });
    let req = Request::builder()
      .method(Method::GET)
      .uri(url.as_str())
      .gitlab_auth(req.auth.as_ref().map(GitlabAuth::as_view))
      .body(Full::new(Bytes::new()))
      .unwrap();

    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpGitlabClientError::Send(format!("{e:?}")))?;
      let body = res
        .into_body()
        .collect()
        .await
        .map_err(|e| HttpGitlabClientError::Receive(format!("{e:?}")))?;
      let body: Bytes = body.to_bytes();
      Ok(body)
    })
  }
}

impl<'req, Cx, TyInner, TyBody> Service<&'req GetProjectReleaseQuery<Cx>> for HttpGitlabClient<TyInner>
where
  Cx: GetRef<GitlabUrl>,
  TyInner: Service<Request<Full<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = Release;
  type Error = HttpGitlabClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpGitlabClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: &'req GetProjectReleaseQuery<Cx>) -> Self::Future {
    let url = req.project.with_str(|project| {
      req
        .context
        .get_ref()
        .url_join(["projects", project, "releases", &req.tag_name])
    });
    let req = Request::builder()
      .method(Method::GET)
      .uri(url.as_str())
      .gitlab_auth(req.auth.as_ref().map(GitlabAuth::as_view))
      .body(Full::new(Bytes::new()))
      .unwrap();

    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpGitlabClientError::Send(format!("{e:?}")))?;
      match res.status() {
        StatusCode::OK => {
          let body = res
            .into_body()
            .collect()
            .await
            .map_err(|e| HttpGitlabClientError::Receive(format!("{e:?}")))?;
          let body: Bytes = body.to_bytes();

          let body: Release =
            serde_json::from_slice(&body).map_err(|e| HttpGitlabClientError::ResponseFormat(format!("{e:?}"), body))?;

          Ok(body)
        }
        StatusCode::NOT_FOUND => Err(HttpGitlabClientError::NotFound),
        code => Err(HttpGitlabClientError::Receive(format!(
          "unexpected status code: {}",
          code
        ))),
      }
    })
  }
}

impl<'req, Cx, TyInner, TyBody> Service<&'req PublishPackageFileCommand<Cx>> for HttpGitlabClient<TyInner>
where
  Cx: GetRef<GitlabUrl>,
  TyInner: Service<Request<Full<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = GenericPackageFile;
  type Error = HttpGitlabClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpGitlabClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: &'req PublishPackageFileCommand<Cx>) -> Self::Future {
    let mut url = req.project.with_str(|project| {
      req.context.get_ref().url_join([
        "projects",
        project,
        "packages",
        "generic",
        &req.package_name,
        &req.package_version,
        &req.filename,
      ])
    });

    {
      let mut q = url.query_pairs_mut();
      q.append_pair(
        "status",
        match req.status {
          InputPackageStatus::Default => "default",
          InputPackageStatus::Hidden => "hidden",
        },
      );
      q.append_pair("select", "package_file");
    }

    let req = Request::builder()
      .method(Method::GET)
      .uri(url.as_str())
      .gitlab_auth(req.auth.as_ref().map(GitlabAuth::as_view))
      .body(Full::new(Bytes::from(req.data.clone())))
      .unwrap();

    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpGitlabClientError::Send(format!("{e:?}")))?;
      match res.status() {
        StatusCode::OK | StatusCode::CREATED => {
          let body = res
            .into_body()
            .collect()
            .await
            .map_err(|e| HttpGitlabClientError::Receive(format!("{e:?}")))?;
          let body: Bytes = body.to_bytes();
          let body: GenericPackageFile =
            serde_json::from_slice(&body).map_err(|e| HttpGitlabClientError::ResponseFormat(format!("{e:?}"), body))?;
          Ok(body)
        }
        StatusCode::FORBIDDEN => Err(HttpGitlabClientError::Forbidden),
        code => Err(HttpGitlabClientError::Receive(format!(
          "unexpected status code: {}",
          code
        ))),
      }
    })
  }
}

impl<'req, Cx, TyInner, TyBody> Service<&'req CreateReleaseCommand<Cx>> for HttpGitlabClient<TyInner>
where
  Cx: GetRef<GitlabUrl>,
  TyInner: Service<Request<Full<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = Release;
  type Error = HttpGitlabClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpGitlabClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: &'req CreateReleaseCommand<Cx>) -> Self::Future {
    #[derive(Debug, serde::Serialize)]
    struct Body<'r> {
      name: Option<&'r str>,
      tag_name: &'r str,
      tag_message: Option<&'r str>,
      description: Option<&'r str>,
      r#ref: Option<&'r str>,
      assets: InputReleaseAssetsView<'r>,
      released_at: Option<DateTime<Utc>>,
    }

    let url = req
      .project
      .with_str(|project| req.context.get_ref().url_join(["projects", project, "releases"]));

    let body = serde_json::to_vec(&Body {
      name: req.name.as_deref(),
      tag_name: req.tag_name.as_str(),
      tag_message: req.tag_message.as_deref(),
      description: req.description.as_deref(),
      r#ref: req.r#ref.as_deref(),
      assets: req.assets.as_view(),
      released_at: req.released_at,
    })
    .unwrap();

    let req = Request::builder()
      .method(Method::GET)
      .uri(url.as_str())
      .gitlab_auth(req.auth.as_ref().map(GitlabAuth::as_view))
      .header(CONTENT_TYPE, "application/json")
      .body(Full::new(Bytes::from(body)))
      .unwrap();

    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpGitlabClientError::Send(format!("{e:?}")))?;
      match res.status() {
        StatusCode::OK | StatusCode::CREATED => {
          let body = res
            .into_body()
            .collect()
            .await
            .map_err(|e| HttpGitlabClientError::Receive(format!("{e:?}")))?;
          let body: Bytes = body.to_bytes();
          let body: Release =
            serde_json::from_slice(&body).map_err(|e| HttpGitlabClientError::ResponseFormat(format!("{e:?}"), body))?;
          Ok(body)
        }
        StatusCode::CONFLICT => Err(HttpGitlabClientError::Conflict),
        StatusCode::FORBIDDEN => Err(HttpGitlabClientError::Forbidden),
        code => Err(HttpGitlabClientError::Receive(format!(
          "unexpected status code: {}",
          code
        ))),
      }
    })
  }
}

impl<'req, Cx, TyInner, TyBody> Service<&'req CreateReleaseLinkCommand<Cx>> for HttpGitlabClient<TyInner>
where
  Cx: GetRef<GitlabUrl>,
  TyInner: Service<Request<Full<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = ReleaseLink;
  type Error = HttpGitlabClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpGitlabClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: &'req CreateReleaseLinkCommand<Cx>) -> Self::Future {
    #[derive(Debug, serde::Serialize)]
    struct Body<'r> {
      name: &'r str,
      url: &'r str,
      direct_asset_path: Option<&'r str>,
      link_type: &'r str,
    }

    let url = req.project.with_str(|project| {
      req.context.get_ref().url_join([
        "projects",
        project,
        "releases",
        req.tag_name.as_str(),
        "assets",
        "links",
      ])
    });

    let body = serde_json::to_vec(&Body {
      name: req.name.as_str(),
      url: req.url.as_str(),
      direct_asset_path: req.direct_asset_path.as_deref(),
      link_type: req.link_type.as_str(),
    })
    .unwrap();

    let req = Request::builder()
      .method(Method::GET)
      .uri(url.as_str())
      .gitlab_auth(req.auth.as_ref().map(GitlabAuth::as_view))
      .header(CONTENT_TYPE, "application/json")
      .body(Full::new(Bytes::from(body)))
      .unwrap();

    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpGitlabClientError::Send(format!("{e:?}")))?;
      match res.status() {
        StatusCode::OK | StatusCode::CREATED => {
          let body = res
            .into_body()
            .collect()
            .await
            .map_err(|e| HttpGitlabClientError::Receive(format!("{e:?}")))?;
          let body: Bytes = body.to_bytes();
          let body: ReleaseLink =
            serde_json::from_slice(&body).map_err(|e| HttpGitlabClientError::ResponseFormat(format!("{e:?}"), body))?;
          Ok(body)
        }
        StatusCode::FORBIDDEN => Err(HttpGitlabClientError::Forbidden),
        code => Err(HttpGitlabClientError::Receive(format!(
          "unexpected status code: {}",
          code
        ))),
      }
    })
  }
}

struct Cursors<Str> {
  first: Option<Str>,
  next: Option<Str>,
  last: Option<Str>,
}

fn get_cursors(headers: &HeaderMap) -> Cursors<CompactString> {
  use demurgos_headers::HeaderMapExt;

  let mut next: Option<CompactString> = None;
  let mut first: Option<CompactString> = None;
  let mut last: Option<CompactString> = None;

  let links: Option<Link> = headers.typed_get::<Link>();

  if let Some(links) = links {
    for value in links.values() {
      let rel = match value.rel() {
        Some(rel) => rel,
        None => continue,
      };
      for r in rel {
        // todo: detect when there are multiple different links for the same rel type
        if *r == RelationType::NEXT {
          next = Some(CompactString::new(value.link()));
        }
        if *r == RelationType::FIRST {
          first = Some(CompactString::new(value.link()));
        }
        if *r == RelationType::LAST {
          last = Some(CompactString::new(value.link()));
        }
      }
    }
  }

  Cursors { first, next, last }
}

trait RequestBuilderExt {
  fn user_agent(self, user_agent: &UserAgent) -> Self;

  fn gitlab_auth(self, gitlab_auth: Option<GitlabAuthView<'_>>) -> Self;
}

impl RequestBuilderExt for http::request::Builder {
  fn user_agent(mut self, user_agent: &UserAgent) -> Self {
    use demurgos_headers::HeaderMapExt;

    if let Some(headers) = self.headers_mut() {
      HeaderMapExt::typed_insert(headers, user_agent.clone());
    }
    self
  }

  fn gitlab_auth(self, gitlab_auth: Option<GitlabAuthView<'_>>) -> Self {
    if let Some(auth) = gitlab_auth {
      let (key, value) = auth.http_header();
      self.header(key, value)
    } else {
      self
    }
  }
}

trait BoolExt {
  fn as_str(&self) -> &'static str;
}

impl BoolExt for bool {
  fn as_str(&self) -> &'static str {
    if *self {
      "true"
    } else {
      "false"
    }
  }
}
