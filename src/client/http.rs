use crate::context::{GetRef, GitlabUrl};
use crate::query::get_project_list::GetProjectListQueryView;
use crate::url_util::UrlExt;
use crate::Project;
use bytes::Bytes;
use core::task::{Context, Poll};
use futures::future::BoxFuture;
use http::{Method, Request, Response};
use http_body::Body;
use http_body_util::{BodyExt, Empty};
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
  #[error("unexpected error: {0}")]
  Other(String),
}

impl<'req, Cx, TyInner, TyBody> Service<GetProjectListQueryView<'req, Cx>> for HttpGitlabClient<TyInner>
where
  Cx: GetRef<GitlabUrl>,
  TyInner: Service<Request<Empty<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = Vec<Project>;
  type Error = HttpGitlabClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpGitlabClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: GetProjectListQueryView<'req, Cx>) -> Self::Future {
    let req = Request::builder()
      .method(Method::GET)
      .uri(req.context.get_ref().url_join(["projects"]).as_str())
      .body(Empty::new())
      .unwrap();
    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpGitlabClientError::Send(format!("{e:?}")))?;
      dbg!(res.status());
      dbg!(res.headers());
      let body = res
        .into_body()
        .collect()
        .await
        .map_err(|e| HttpGitlabClientError::Receive(format!("{e:?}")))?;
      let body: Bytes = body.to_bytes();

      let body: Vec<Project> =
        serde_json::from_slice(&body).map_err(|e| HttpGitlabClientError::ResponseFormat(format!("{e:?}"), body))?;
      Ok(body)
    })
  }
}
