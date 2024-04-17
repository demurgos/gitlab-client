use gitlab_client::client::http::HttpGitlabClient;
use gitlab_client::context::{Context, GitlabUrl};
use gitlab_client::query::get_project_list::GetProjectListQuery;
use gitlab_client::reqwest::Url;
use gitlab_client::tower_service::Service;
use gitlab_client::GitlabClient;
use hyper_tls::HttpsConnector;

#[tokio::main]
async fn main() {
  let connector = HttpsConnector::new();
  let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new()).build(connector);
  let mut client = HttpGitlabClient::new(client);
  let context = Context::new().set_gitlab_url(GitlabUrl(Url::parse("https://gitlab.com/").unwrap()));
  let query = GetProjectListQuery::<_>::new().set_context(context);
  let res = client.call(query.as_view()).await;
  dbg!(res);
}
