use gitlab_client::client::http::HttpGitlabClient;
use gitlab_client::common::project::{ProjectRef, ProjectSlug};
use gitlab_client::compact_str::CompactString;
use gitlab_client::context::{Context, GitlabUrl};
use gitlab_client::query::get_project::GetProjectQuery;
use gitlab_client::query::get_project_list::GetProjectListQuery;
use gitlab_client::query::get_project_list_page::GetProjectListPageQuery;
use gitlab_client::tower_service::Service;
use gitlab_client::url::Url;
use gitlab_client::{GitlabAuth, GitlabClient};
use hyper_tls::HttpsConnector;

const TOKEN: &str = "glpat-zw-cpaX998ohRcPLdZ7v";

#[tokio::main]
async fn main() {
  let connector = HttpsConnector::new();
  let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new()).build(connector);
  let mut client = HttpGitlabClient::new(client);
  let context = Context::new().set_gitlab_url(GitlabUrl(Url::parse("https://gitlab.com/").unwrap()));
  // let mut query = GetProjectListQuery::<_>::new().set_context(context);
  // // query.auth = Some(GitlabAuth::PrivateToken("...".parse().unwrap()));
  // // query.owned = Some(true);
  // let res = client.get_project_list(&query).await.unwrap();
  // for p in &res.items {
  //   dbg!(&p.path_with_namespace);
  // }
  // if let Some(next) = res.next {
  //   let context = Context::new().set_gitlab_url(GitlabUrl(Url::parse("https://gitlab.com/").unwrap()));
  //   let query = GetProjectListPageQuery::<_>::new(next).set_context(context);
  //   let res = client.call(&query).await.unwrap();
  //   for s in &res.items {
  //     dbg!(&s.path_with_namespace);
  //   }
  // }
  let mut query = GetProjectQuery::<_>::new(ProjectRef::Slug(ProjectSlug::new(CompactString::new(
    "demurgos/eternaltwin",
  ))))
  .set_context(context);
  query.auth = Some(GitlabAuth::PrivateToken(TOKEN.parse().unwrap()));
  let res = client.call(&query).await.unwrap();
  dbg!(res);
}
