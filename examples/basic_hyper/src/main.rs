use bytes::Bytes;
use gitlab_client::client::http::HttpGitlabClient;
use gitlab_client::common::project::{ProjectRef, ProjectSlug};
use gitlab_client::compact_str::CompactString;
use gitlab_client::context::{Context, GitlabUrl};
use gitlab_client::query::get_project::GetProjectQuery;
use gitlab_client::query::get_tree_record_list::GetTreeRecordListQuery;
use gitlab_client::tower_service::Service;
use gitlab_client::url::Url;
use gitlab_client::{GitlabAuth, UserAgent};
use http_body_util::Full;
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::Client;

#[tokio::main]
async fn main() {
  let authentication: Option<GitlabAuth> = if let Some(token) = std::env::var("GITLAB_PRIVATE_TOKEN").ok() {
    Some(GitlabAuth::PrivateToken(token.parse().unwrap()))
  } else {
    None
  };

  let connector = HttpsConnector::new();
  let client: Client<HttpsConnector<_>, Full<Bytes>> =
    Client::builder(hyper_util::rt::TokioExecutor::new()).build(connector);
  let mut client = HttpGitlabClient::new(client);
  let context = Context::new()
    .set_gitlab_url(GitlabUrl(Url::parse("https://gitlab.com/").unwrap()))
    .set_user_agent(UserAgent::from_static("gitlab_client_example/0.0.0"));
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
  {
    let mut query = GetProjectQuery::<_>::new(ProjectRef::Slug(ProjectSlug::new(CompactString::new(
      "demurgos/eternaltwin",
    ))))
    .set_context(context.clone());
    query.auth = authentication.clone();
    let res = client.call(&query).await.unwrap();
    eprintln!("successfully fetched project. create_at={:?}", res.created_at);
  }
  {
    let mut query = GetTreeRecordListQuery::<_>::new(ProjectRef::Slug(ProjectSlug::new(CompactString::new(
      "demurgos/eternaltwin",
    ))))
    .set_context(context);
    query.auth = authentication.clone();
    let res = client.call(&query).await.unwrap();
    eprintln!("successfully fetched project. created_at={:?}", res.items.len());
  }
}
