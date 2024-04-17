use crate::query::get_project_list::{GetProjectListError, GetProjectListQueryView};
use crate::Project;
use bytes::Bytes;
use core::task::{Context, Poll};
use futures::future::BoxFuture;
use futures::TryFutureExt;
use http::{Method, Request, Response};
use http_body::Body;
use http_body_util::{BodyExt, Empty, Full};
use tower_service::Service;

pub struct HttpGitlabClient<TyInner> {
  inner: TyInner,
}

impl<TyInner> HttpGitlabClient<TyInner> {
  pub fn new(inner: TyInner) -> Self {
    Self { inner }
  }
}

impl<'req, TyInner, TyBody> Service<GetProjectListQueryView<'req>> for HttpGitlabClient<TyInner>
where
  TyInner: Service<Request<Empty<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
{
  type Response = Vec<Project>;
  type Error = GetProjectListError<TyInner::Error>;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.inner.poll_ready(cx).map_err(Self::Error::Inner)
  }

  fn call(&mut self, req: GetProjectListQueryView<'req>) -> Self::Future {
    let req = Request::builder()
      .method(Method::GET)
      .uri(req.base.api_url(["projects"]).as_str())
      .body(Empty::new())
      .unwrap();
    let res = self.inner.call(req);
    Box::pin(async move {
      // let res: Response<TyBody> = match res.await {
      //   Ok(b) => b,
      //   Err(e) => panic!("failure"),
      // };
      let res: Response<TyBody> = res.await.map_err(GetProjectListError::Inner)?;
      dbg!(res.status());
      dbg!(res.headers());
      let body = match res.into_body().collect().await {
        Ok(c) => c,
        Err(e) => todo!(),
      };
      let body: Bytes = body.to_bytes();

      let body: Vec<Project> =
        serde_json::from_slice(&body).map_err(|e| GetProjectListError::ResponseFormat(format!("{e:?}")))?;
      Ok(body)
    })
  }
}
