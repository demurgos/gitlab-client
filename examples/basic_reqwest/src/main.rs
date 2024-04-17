use gitlab_client::client::reqwest::ReqwestGitlabClient;
use gitlab_client::command::publish_package_file::PublishPackageFileRequest;
use gitlab_client::query::get_project_list::GetProjectListQuery;
use gitlab_client::tower_service::Service;
use gitlab_client::GitlabClient;
use gitlab_client::{InputPackageStatus, ProjectId, ProjectRef, QueryBase};

#[tokio::main]
async fn main() {
  let client = gitlab_client::reqwest::Client::new();
  let mut client = ReqwestGitlabClient::new(client);
  let base = QueryBase::new();
  let query: GetProjectListQuery = GetProjectListQuery::new(base);
  let res = client.get_project_list(query.as_view()).await;
  dbg!(res);
}
