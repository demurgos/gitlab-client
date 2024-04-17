use crate::command::publish_package_file::{PublishPackageFileError, PublishPackageFileRequest};
use crate::query::get_project_list::{GetProjectListError, GetProjectListQueryView};
use crate::{GenericPackageFile, Project};
use core::task::{Context, Poll};
use futures::future::BoxFuture;
use futures::TryFutureExt;
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

impl<'req, TyInner> Service<GetProjectListQueryView<'req>> for ReqwestGitlabClient<TyInner>
where
  TyInner: Service<Request, Response = Response, Error = reqwest::Error> + 'req,
  TyInner::Future: Send,
{
  type Response = Vec<Project>;
  type Error = GetProjectListError<TyInner::Error>;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.inner.poll_ready(cx).map_err(Self::Error::Inner)
  }

  fn call(&mut self, req: GetProjectListQueryView<'req>) -> Self::Future {
    let req = Request::new(Method::GET, req.base.api_url(["projects"]));
    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response = res.await.map_err(GetProjectListError::Inner)?;
      let body = res.text().await.map_err(GetProjectListError::Inner)?;

      let body: Vec<Project> =
        serde_json::from_str(&body).map_err(|e| GetProjectListError::ResponseFormat(format!("{e:?}")))?;
      Ok(body)
    })
  }
}

impl<TyInner> Service<PublishPackageFileRequest> for ReqwestGitlabClient<TyInner>
where
  TyInner: Service<Request>,
{
  type Response = GenericPackageFile;
  type Error = PublishPackageFileError<TyInner::Error>;
  type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.inner.poll_ready(cx).map_err(Self::Error::Inner)
  }

  fn call(&mut self, req: PublishPackageFileRequest) -> Self::Future {
    todo!()
  }
}
