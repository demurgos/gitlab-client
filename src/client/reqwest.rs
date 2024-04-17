use crate::query::get_project_list::GetProjectListQueryView;
use crate::url_util::UrlExt;
use crate::{GetGitlabUrl, Project};
use core::task::{Context, Poll};
use futures::future::BoxFuture;
use reqwest::{Method, Request, Response};
use tower_service::Service;

pub struct ReqwestGitlabClient<TyInner> {
  inner: TyInner,
}

impl<TyInner> ReqwestGitlabClient<TyInner> {
  pub fn new(inner: TyInner) -> Self {
    Self { inner }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum ReqwestGitlabClientError {
  #[error("failed to poll ready status: {0}")]
  PollReady(String),
  #[error("failed to send request: {0}")]
  Send(String),
  #[error("failed to receive response: {0}")]
  Receive(String),
  #[error("failed to parse response with body = {1}: {0}")]
  ResponseFormat(String, String),
  #[error("operation is forbidden for provided auth")]
  Forbidden,
  #[error("unexpected error: {0}")]
  Other(String),
}

impl<'req, ExtraInput, TyInner> Service<GetProjectListQueryView<'req, ExtraInput>> for ReqwestGitlabClient<TyInner>
where
  ExtraInput: GetGitlabUrl,
  TyInner: Service<Request, Response = Response, Error = reqwest::Error> + 'req,
  TyInner::Future: Send,
{
  type Response = Vec<Project>;
  type Error = ReqwestGitlabClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| ReqwestGitlabClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: GetProjectListQueryView<'req, ExtraInput>) -> Self::Future {
    let req = Request::new(Method::GET, req.extra_input.gitlab_url().url_join(["projects"]));
    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response = res
        .await
        .map_err(|e| ReqwestGitlabClientError::Send(format!("{e:?}")))?;
      let body = res
        .text()
        .await
        .map_err(|e| ReqwestGitlabClientError::Receive(format!("{e:?}")))?;

      let body: Vec<Project> =
        serde_json::from_str(&body).map_err(|e| ReqwestGitlabClientError::ResponseFormat(format!("{e:?}"), body))?;
      Ok(body)
    })
  }
}

// impl<TyInner> Service<PublishPackageFileRequest> for ReqwestGitlabClient<TyInner>
//   where
//     TyInner: Service<Request, Response=Response, Error=reqwest::Error>,
//     TyInner::Future: Send,
// {
//   type Response = GenericPackageFile;
//   type Error = ReqwestGitlabClientError;
//   type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
//
//   fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//     self.inner.poll_ready(cx).map_err(|e| ReqwestGitlabClientError::PollReady(format!("{e:?}")))
//   }
//
//   fn call(&mut self, req: PublishPackageFileRequest) -> Self::Future {
//     todo!()
//   }
// }
