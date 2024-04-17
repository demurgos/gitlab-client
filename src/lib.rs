pub use ::reqwest;
pub use ::tower_service;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tower_service::Service;
use url::Url;
use crate::common::project::{Project, ProjectRef};
use crate::query::get_project_list::GetProjectListQueryView;

pub mod client;
pub mod command;
#[cfg(feature = "http")]
pub mod http;
pub mod query;
mod common;

#[async_trait]
pub trait GitlabClient: Send + Sync {
  type Error<'req>;
  async fn get_project_list(self, query: GetProjectListQueryView<'_>) -> Result<Vec<Project>, Self::Error<'_>>;

  // async fn publish_package_file(
  //   &self,
  //   req: PublishPackageFileRequestView<'_, true>,
  // ) -> Result<GenericPackageFile, PublishPackageFileError>;
  //
  // async fn get_package_file(&self, req: GetPackageFileRequestView<'_>) -> Result<Vec<u8>, GetPackageFileError>;
  //
  // async fn create_release(&self, req: CreateReleaseRequestView<'_, String>) -> Result<Release, CreateReleaseError>;
  //
  // async fn get_release(&self, req: GetReleaseRequestView<'_>) -> Result<Release, GetReleaseError>;
  //
  // async fn create_release_link(
  //   &self,
  //   req: CreateReleaseLinkRequestView<'_>,
  // ) -> Result<ReleaseLink, CreateReleaseLinkError>;
}

#[async_trait]
impl<'a, S> GitlabClient for &'a mut S
where
  Self: Send + Sync,
  S: for<'req> Service<GetProjectListQueryView<'req>, Response = Vec<Project>>,
  for<'req> <S as Service<GetProjectListQueryView<'req>>>::Future: Send,
{
  type Error<'req> = <S as Service<GetProjectListQueryView<'req>>>::Error;

  async fn get_project_list(self, query: GetProjectListQueryView<'_>) -> Result<Vec<Project>, Self::Error<'_>> {
    self.call(query).await
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryBase<Str = String> {
  pub instance_url: Str,
  pub auth: Option<GitlabAuth<Str>>,
}

pub type QueryBaseView<'req> = QueryBase<&'req str>;

impl QueryBase<String> {
  pub fn new() -> Self {
    Self {
      instance_url: String::from("https://gitlab.com/"),
      auth: None,
    }
  }
}

impl<Str> QueryBase<Str>
where
  Str: AsRef<str>,
{
  pub fn api_url<I>(&self, segments: I) -> Url
  where
    I: IntoIterator,
    I::Item: AsRef<str>,
  {
    let mut res = Url::parse(self.instance_url.as_ref()).expect("invalid instance URL");
    {
      let mut p = res.path_segments_mut().expect("GitLab URL has path segments");
      p.extend(["api", "v4"]);
      p.extend(segments);
    }
    res
  }

  pub fn as_view(&self) -> QueryBaseView<'_> {
    QueryBaseView {
      instance_url: self.instance_url.as_ref(),
      auth: self.auth.as_ref().map(|a| a.as_view()),
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputPackageStatus {
  Default,
  Hidden,
}

impl InputPackageStatus {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Default => "default",
      Self::Hidden => "hidden",
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PackageStatus {
  Default,
  Hidden,
  Processing,
  Error,
  PendingDestruction,
}

impl PackageStatus {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Default => "default",
      Self::Hidden => "hidden",
      Self::Processing => "processing",
      Self::Error => "error",
      Self::PendingDestruction => "pending_destruction",
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GitlabAuth<Token = String> {
  PrivateToken(Token),
  JobToken(Token),
}

pub type GitlabAuthView<'s> = GitlabAuth<&'s str>;

impl<Token: AsRef<str>> GitlabAuth<Token> {
  pub fn as_view(&self) -> GitlabAuthView<'_> {
    match self {
      Self::PrivateToken(token) => GitlabAuth::PrivateToken(token.as_ref()),
      Self::JobToken(token) => GitlabAuth::JobToken(token.as_ref()),
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GenericPackageFile {
  pub id: u64,
  pub package_id: u64,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub size: u64,
  pub file_store: u64,
  pub file_md5: Option<String>,
  pub file_sha1: Option<String>,
  pub file_name: String,
  pub file: GitlabFile,
  pub file_sha256: Option<String>,
  pub verification_retry_at: Option<DateTime<Utc>>,
  pub verified_at: Option<DateTime<Utc>>,
  pub verification_failure: Option<String>,
  pub verification_retry_count: Option<u64>,
  pub verification_checksum: Option<String>,
  pub verification_state: u64,
  pub verification_started_at: Option<DateTime<Utc>>,
  pub status: String,
  // TODO: PackageStatus
  pub new_file_path: Option<String>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GitlabFile {
  pub url: String,
}

/// Get a generic package file
///
/// <https://docs.gitlab.com/ee/user/packages/generic_packages/#download-package-file>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetPackageFileRequest<Str = String> {
  pub auth: Option<GitlabAuth<Str>>,
  pub project: ProjectRef<Str>,
  pub package_name: Str,
  pub package_version: Str,
  pub filename: Str,
}

pub type GetPackageFileRequestView<'req> = GetPackageFileRequest<&'req str>;

impl<Str: AsRef<str>> GetPackageFileRequest<Str> {
  pub fn as_view(&self) -> GetPackageFileRequestView<'_> {
    GetPackageFileRequestView {
      auth: self.auth.as_ref().map(GitlabAuth::as_view),
      project: self.project.as_view(),
      package_name: self.package_name.as_ref(),
      package_version: self.package_version.as_ref(),
      filename: self.filename.as_ref(),
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum GetPackageFileError {
  #[error("failed to send `GetPackageFile` request: {0}")]
  Send(String),
  #[error("failed to receive `GetPackageFile` response: {0}")]
  Receive(String),
  #[error("unexpected `GetPackageFile` error: {0}")]
  Other(String),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SortDirection {
  /// Ascending: lowest first, largest last
  Asc,
  /// Descending: largest first, lowest last
  Desc,
}

/// Criteria used to order packages
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PackageOrder {
  CreatedAt,
  Name,
  Version,
  Type,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PackageType {
  Composer,
  Conan,
  Generic,
  Golang,
  Helm,
  Maven,
  Npm,
  Nuget,
  Pypi,
  TerraformModule,
}

/// List project packages
///
/// <https://docs.gitlab.com/ee/api/packages.html#within-a-project>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListProjectPackagesRequest<Str = String> {
  pub auth: Option<GitlabAuth<Str>>,
  pub project: ProjectRef<Str>,
  pub order_by: PackageOrder,
  pub sort: SortDirection,
  pub package_type: PackageType,
  pub package_name: Str,
  pub include_versionless: bool,
  pub status: PackageStatus,
}

pub type ListProjectPackagesRequestView<'req> = ListProjectPackagesRequest<&'req str>;

impl<Str: AsRef<str>> ListProjectPackagesRequest<Str> {
  pub fn as_view(&self) -> ListProjectPackagesRequestView<'_> {
    ListProjectPackagesRequestView {
      auth: self.auth.as_ref().map(GitlabAuth::as_view),
      project: self.project.as_view(),
      order_by: self.order_by,
      sort: self.sort,
      package_type: self.package_type,
      package_name: self.package_name.as_ref(),
      include_versionless: self.include_versionless,
      status: self.status,
    }
  }
}

/// Criteria used to order releases
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReleaseOrder {
  ReleasedAt,
  CreatedAt,
}

/// List project releases
///
/// <https://docs.gitlab.com/ee/api/releases/#list-releases>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListProjectReleasesRequest<Str = String> {
  pub auth: Option<GitlabAuth<Str>>,
  pub project: ProjectRef<Str>,
  pub order_by: ReleaseOrder,
  pub sort: SortDirection,
  pub include_html_description: bool,
}

pub type ListProjectReleasesRequestView<'req> = ListProjectReleasesRequest<&'req str>;

impl<Str: AsRef<str>> ListProjectReleasesRequest<Str> {
  pub fn as_view(&self) -> ListProjectReleasesRequestView<'_> {
    ListProjectReleasesRequestView {
      auth: self.auth.as_ref().map(GitlabAuth::as_view),
      project: self.project.as_view(),
      order_by: self.order_by,
      sort: self.sort,
      include_html_description: self.include_html_description,
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReleaseLinkType {
  #[cfg_attr(feature = "serde", serde(rename = "other"))]
  Other,
  #[cfg_attr(feature = "serde", serde(rename = "runbook"))]
  Runbook,
  #[cfg_attr(feature = "serde", serde(rename = "image"))]
  Image,
  #[cfg_attr(feature = "serde", serde(rename = "package"))]
  Package,
}

impl ReleaseLinkType {
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Other => "other",
      Self::Runbook => "runbook",
      Self::Image => "image",
      Self::Package => "package",
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputReleaseAssets<Links = Vec<InputReleaseLink<String>>> {
  pub links: Links,
}

pub type InputReleaseAssetsView<'req, Str = String> = InputReleaseAssets<&'req [InputReleaseLink<Str>]>;

impl<Links> InputReleaseAssets<Links> {
  pub fn as_view<Str>(&self) -> InputReleaseAssetsView<'_, Str>
  where
    Links: AsRef<[InputReleaseLink<Str>]>,
  {
    InputReleaseAssetsView {
      links: self.links.as_ref(),
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputReleaseLink<Str = String> {
  pub name: Str,
  pub url: Str,
  pub direct_asset_path: Option<Str>,
  pub link_type: ReleaseLinkType,
}

pub type ReleaseLinkView<'req> = InputReleaseLink<&'req str>;

impl<Str: AsRef<str>> InputReleaseLink<Str> {
  pub fn as_view(&self) -> ReleaseLinkView<'_> {
    ReleaseLinkView {
      name: self.name.as_ref(),
      url: self.url.as_ref(),
      direct_asset_path: self.direct_asset_path.as_ref().map(|s| s.as_ref()),
      link_type: self.link_type,
    }
  }
}

/// Create a project release
///
/// <https://docs.gitlab.com/ee/api/releases/#create-a-release>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateReleaseRequest<Str = String, Assets = InputReleaseAssets> {
  pub auth: Option<GitlabAuth<Str>>,
  pub project: ProjectRef<Str>,
  pub tag_name: Str,
  pub name: Option<Str>,
  pub tag_message: Option<Str>,
  pub description: Option<Str>,
  pub r#ref: Option<Str>,
  // milestones: Vec<Str>,
  pub assets: Assets,
  pub released_at: Option<DateTime<Utc>>,
}

pub type CreateReleaseRequestView<'req, Str> = CreateReleaseRequest<&'req str, InputReleaseAssetsView<'req, Str>>;

impl<Str: AsRef<str>, Links> CreateReleaseRequest<Str, InputReleaseAssets<Links>>
where
  Links: AsRef<[InputReleaseLink<Str>]>,
{
  pub fn as_view(&self) -> CreateReleaseRequestView<'_, Str> {
    CreateReleaseRequestView {
      auth: self.auth.as_ref().map(GitlabAuth::as_view),
      project: self.project.as_view(),
      tag_name: self.tag_name.as_ref(),
      name: self.name.as_ref().map(|s| s.as_ref()),
      tag_message: self.tag_message.as_ref().map(|s| s.as_ref()),
      description: self.description.as_ref().map(|s| s.as_ref()),
      r#ref: self.r#ref.as_ref().map(|s| s.as_ref()),
      // milestones: Vec<Str>,
      assets: self.assets.as_view(),
      released_at: self.released_at,
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum CreateReleaseError {
  #[error("release already exists")]
  AlreadyExists,
  #[error("failed to send `CreateRelease` request: {0}")]
  Send(String),
  #[error("failed to receive `CreateRelease` response: {0}")]
  Receive(String),
  #[error("failed to parse `CreateRelease` response with body = {1}: {0}")]
  ResponseFormat(String, String),
  #[error("unexpected `CreateRelease` error: {0}")]
  Other(String),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Release {
  pub name: String,
  pub tag_name: String,
  pub description: Option<String>,
  pub created_at: DateTime<Utc>,
  pub released_at: DateTime<Utc>,
  pub upcoming_release: bool,
  pub author: Author,
  pub commit: Commit,
  pub milestones: Option<Vec<Milestone>>,
  pub commit_path: String,
  pub tag_path: String,
  pub assets: ReleaseAssets,
  pub evidences: Vec<ReleaseEvidence>,
  pub _links: ReleaseLinks,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthorId(u64);

impl AuthorId {
  pub const fn new(id: u64) -> Self {
    Self(id)
  }

  pub const fn into_u64(self) -> u64 {
    self.0
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Author {
  pub id: AuthorId,
  pub name: String,
  pub username: String,
  pub state: String,
  pub avatar_url: String,
  pub web_url: String,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Commit {
  pub id: String,
  pub short_id: String,
  pub title: String,
  pub created_at: DateTime<Utc>,
  pub parent_ids: Vec<String>,
  pub message: String,
  pub author_name: String,
  pub author_email: String,
  pub authored_date: DateTime<Utc>,
  pub committer_name: String,
  pub committer_email: String,
  pub committed_date: DateTime<Utc>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Milestone {
  // TODO
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseAssets {
  pub count: u64,
  pub sources: Vec<ReleaseSource>,
  pub links: Vec<ReleaseLink>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseSource {
  pub format: String,
  pub url: String,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseEvidence {
  // TODO
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseLinks {
  closed_issues_url: String,
  closed_merge_requests_url: String,
  edit_url: String,
  merged_merge_requests_url: String,
  opened_issues_url: String,
  opened_merge_requests_url: String,
  #[cfg_attr(feature = "serde", serde(rename = "self"))]
  this: String,
}

/// Get a project release
///
/// <https://docs.gitlab.com/ee/api/releases/#get-a-release-by-a-tag-name>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetReleaseRequest<Str = String> {
  pub auth: Option<GitlabAuth<Str>>,
  pub project: ProjectRef<Str>,
  pub tag_name: Str,
  pub include_html_description: bool,
}

pub type GetReleaseRequestView<'req> = GetReleaseRequest<&'req str>;

impl<Str: AsRef<str>> GetReleaseRequest<Str> {
  pub fn as_view(&self) -> GetReleaseRequestView<'_> {
    GetReleaseRequestView {
      auth: self.auth.as_ref().map(GitlabAuth::as_view),
      project: self.project.as_view(),
      tag_name: self.tag_name.as_ref(),
      include_html_description: self.include_html_description,
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum GetReleaseError {
  #[error("failed to send `GetRelease` request: {0}")]
  Send(String),
  #[error("failed to receive `GetRelease` response: {0}")]
  Receive(String),
  #[error("release not found")]
  NotFound,
  #[error("unexpected `GetRelease` error: {0}")]
  Other(String),
}

/// Update a project release
///
/// <https://docs.gitlab.com/ee/api/releases/#update-a-release>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UpdateReleaseRequest<Str = String> {
  pub auth: Option<GitlabAuth<Str>>,
  pub project: ProjectRef<Str>,
  pub tag_name: Str,
  pub name: Option<Str>,
  pub description: Option<Str>,
  // milestones: Vec<Str>,
  pub released_at: Option<DateTime<Utc>>,
}

pub type UpdateReleaseRequestView<'req> = UpdateReleaseRequest<&'req str>;

impl<Str: AsRef<str>> UpdateReleaseRequest<Str> {
  pub fn as_view(&self) -> UpdateReleaseRequestView<'_> {
    UpdateReleaseRequestView {
      auth: self.auth.as_ref().map(GitlabAuth::as_view),
      project: self.project.as_view(),
      tag_name: self.tag_name.as_ref(),
      name: self.name.as_ref().map(|s| s.as_ref()),
      description: self.description.as_ref().map(|s| s.as_ref()),
      // milestones: Vec<Str>,
      released_at: self.released_at,
    }
  }
}

/// Create a project release link
///
/// <https://docs.gitlab.com/ee/api/releases/links.html#create-a-release-link>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateReleaseLinkRequest<Str = String> {
  pub auth: Option<GitlabAuth<Str>>,
  pub project: ProjectRef<Str>,
  pub tag_name: Str,
  pub name: Str,
  pub url: Str,
  pub direct_asset_path: Option<Str>,
  pub link_type: ReleaseLinkType,
}

pub type CreateReleaseLinkRequestView<'req> = CreateReleaseLinkRequest<&'req str>;

impl<Str: AsRef<str>> CreateReleaseLinkRequest<Str> {
  pub fn as_view(&self) -> CreateReleaseLinkRequestView<'_> {
    CreateReleaseLinkRequestView {
      auth: self.auth.as_ref().map(GitlabAuth::as_view),
      project: self.project.as_view(),
      tag_name: self.tag_name.as_ref(),
      name: self.name.as_ref(),
      url: self.url.as_ref(),
      direct_asset_path: self.direct_asset_path.as_ref().map(|s| s.as_ref()),
      link_type: self.link_type,
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum CreateReleaseLinkError {
  #[error("failed to send `CreateReleaseLink` request: {0}")]
  Send(String),
  #[error("failed to receive `CreateReleaseLink` response: {0}")]
  Receive(String),
  #[error("failed to parse `CreateReleaseLink` response with body = {1}: {0}")]
  ResponseFormat(String, String),
  #[error("unexpected `CreateReleaseLink` error: {0}")]
  Other(String),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseLinkId(u64);

impl ReleaseLinkId {
  pub const fn new(id: u64) -> Self {
    Self(id)
  }

  pub const fn into_u64(self) -> u64 {
    self.0
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseLink {
  pub id: ReleaseLinkId,
  pub name: String,
  pub url: String,
  pub direct_asset_url: String,
  pub link_type: ReleaseLinkType,
}

#[cfg(test)]
mod test {
  use crate::{
    Author, AuthorId, Commit, GenericPackageFile, GitlabFile, Release, ReleaseAssets, ReleaseEvidence, ReleaseLink,
    ReleaseLinkId, ReleaseLinkType, ReleaseLinks, ReleaseSource,
  };
  use chrono::{TimeZone, Utc};

  #[cfg_attr(feature = "serde", test)]
  #[allow(deprecated)]
  fn read_publish_package_file_response() {
    let raw = r#"{"id":72696499,"package_id":13447789,"created_at":"2023-03-22T21:37:30.948Z","updated_at":"2023-03-22T21:37:30.948Z","size":11,"file_store":2,"file_md5":null,"file_sha1":null,"file_name":"eternaltwin3","file":{"url":"https://storage.googleapis.com/gitlab-gprd-package-repo/4a/ac/4aac49a9b7a3711a7fc154e49d6751b1b0d75358c4657d14c0bd55291a20c250/packages/13447789/files/72696499/eternaltwin3?GoogleAccessId=gitlab-object-storage-prd@gitlab-production.iam.gserviceaccount.com\u0026Signature=RozKlG66y15cnKPi5kNZZ2PVdJlYlzCGJ7fOOGnPU9kkOHoQqqAY3EwIBqhE%0Aydqr4u%2FYx9A%2BaoNPv9XUNlqnyHl0BLzFYjh5hUwFktix8IwD%2BkMgbrhJN0Yj%0AcHbJsVrDBEDSyMRjNWaBDo%2BzeAe9exYQdQ9iOyQct3zLXTkEwjS5fnnBoF0L%0APohSUcBLOeTm0gzZJJ1n4UB5yPb7RoDPZelN%2BCZxNdcY2AMjahQcnLgnzbnJ%0AFEHliT9107dZU33iKFprMQGcqCKupTFO%2FIu11uik2S%2BKAlAOjZySsKiV8%2BBE%0A%2Bi2pttVA%2FEW2P8soM9ZtYj4ReP1N8ZPPpW0AG6iTzA%3D%3D\u0026Expires=1679521651"},"file_sha256":"0ca093111f402faa55be1cd71006270644b58619eb0c2408b97b7d24bb70dd09","verification_retry_at":null,"verified_at":null,"verification_failure":null,"verification_retry_count":null,"verification_checksum":null,"verification_state":0,"verification_started_at":null,"status":"default","new_file_path":null}"#;
    let actual: GenericPackageFile = serde_json::from_str(raw).unwrap();
    let expected = GenericPackageFile {
            id: 72696499,
            package_id: 13447789,
            created_at: Utc.ymd(2023, 3, 22).and_hms_milli(21, 37, 30, 948),
            updated_at: Utc.ymd(2023, 3, 22).and_hms_milli(21, 37, 30, 948),
            size: 11,
            file_store: 2,
            file_md5: None,
            file_sha1: None,
            file_name: "eternaltwin3".to_string(),
            file: GitlabFile { url: r#"https://storage.googleapis.com/gitlab-gprd-package-repo/4a/ac/4aac49a9b7a3711a7fc154e49d6751b1b0d75358c4657d14c0bd55291a20c250/packages/13447789/files/72696499/eternaltwin3?GoogleAccessId=gitlab-object-storage-prd@gitlab-production.iam.gserviceaccount.com&Signature=RozKlG66y15cnKPi5kNZZ2PVdJlYlzCGJ7fOOGnPU9kkOHoQqqAY3EwIBqhE%0Aydqr4u%2FYx9A%2BaoNPv9XUNlqnyHl0BLzFYjh5hUwFktix8IwD%2BkMgbrhJN0Yj%0AcHbJsVrDBEDSyMRjNWaBDo%2BzeAe9exYQdQ9iOyQct3zLXTkEwjS5fnnBoF0L%0APohSUcBLOeTm0gzZJJ1n4UB5yPb7RoDPZelN%2BCZxNdcY2AMjahQcnLgnzbnJ%0AFEHliT9107dZU33iKFprMQGcqCKupTFO%2FIu11uik2S%2BKAlAOjZySsKiV8%2BBE%0A%2Bi2pttVA%2FEW2P8soM9ZtYj4ReP1N8ZPPpW0AG6iTzA%3D%3D&Expires=1679521651"#.to_string() },
            file_sha256: Some("0ca093111f402faa55be1cd71006270644b58619eb0c2408b97b7d24bb70dd09".to_string()),
            verification_retry_at: None,
            verified_at: None,
            verification_failure: None,
            verification_retry_count: None,
            verification_checksum: None,
            verification_state: 0,
            verification_started_at: None,
            status: "default".to_string(),
            new_file_path: None,
        };
    assert_eq!(actual, expected);
  }

  #[cfg_attr(feature = "serde", test)]
  #[allow(deprecated)]
  fn read_release() {
    let raw = r#"{"name": "v0.12.5","tag_name": "v0.12.5","description": null,"created_at": "2023-03-25T23:03:17.165Z","released_at": "2023-03-25T23:03:17.165Z","upcoming_release": false,"author": {"id": 743516,"username": "demurgos","name": "Charles Samborski","state": "active","avatar_url": "https://secure.gravatar.com/avatar/4ccbb457b6d50bb79ec32b66c2f4e301?s=80&d=identicon","web_url": "https://gitlab.com/demurgos"},"commit": {"id": "e8cfd320ee2b8dcee22b0bc0ceaf52d7dd667c2d","short_id": "e8cfd320","created_at": "2023-03-17T00:39:35.000+01:00","parent_ids": ["c8528d12e05c9ee803547eb556cb5394ac3d1abc"],"title": "[bin] Add support for precompiled binary","message": "[bin] Add support for precompiled binary\n\nThis commit adds support for precompiled binaries built in GitLab CI.\n","author_name": "Charles Samborski","author_email": "demurgos@demurgos.net","authored_date": "2023-03-16T23:56:54.000+01:00","committer_name": "Charles Samborski","committer_email": "demurgos@demurgos.net","committed_date": "2023-03-17T00:39:35.000+01:00","trailers": {},"web_url": "https://gitlab.com/eternaltwin/eternaltwin/-/commit/e8cfd320ee2b8dcee22b0bc0ceaf52d7dd667c2d"},"commit_path": "/eternaltwin/eternaltwin/-/commit/e8cfd320ee2b8dcee22b0bc0ceaf52d7dd667c2d","tag_path": "/eternaltwin/eternaltwin/-/tags/v0.12.5","assets": {"count": 5,"sources": [{"format": "zip","url": "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.zip"},{"format": "tar.gz","url": "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.tar.gz"},{"format": "tar.bz2","url": "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.tar.bz2"},{"format": "tar","url": "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.tar"}],"links": [{"id": 1492034,"name": "eternaltwin-x86_64-apple-darwin","url": "https://gitlab.com/eternaltwin/eternaltwin/-/packages/13511182","direct_asset_url": "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5/downloads/eternaltwin-x86_64-apple-darwin","link_type": "package"}]},"evidences": [{"sha": "f9021712d926f78fc9272ac24733492facd5fb4353ed","filepath": "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5/evidences/4460894.json","collected_at": "2023-03-25T23:03:17.328Z"}],"_links": {"closed_issues_url": "https://gitlab.com/eternaltwin/eternaltwin/-/issues?release_tag=v0.12.5&scope=all&state=closed","closed_merge_requests_url": "https://gitlab.com/eternaltwin/eternaltwin/-/merge_requests?release_tag=v0.12.5&scope=all&state=closed","edit_url": "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5/edit","merged_merge_requests_url": "https://gitlab.com/eternaltwin/eternaltwin/-/merge_requests?release_tag=v0.12.5&scope=all&state=merged","opened_issues_url": "https://gitlab.com/eternaltwin/eternaltwin/-/issues?release_tag=v0.12.5&scope=all&state=opened","opened_merge_requests_url": "https://gitlab.com/eternaltwin/eternaltwin/-/merge_requests?release_tag=v0.12.5&scope=all&state=opened","self": "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5"}}"#;
    let actual: Release = serde_json::from_str(raw).unwrap();
    let expected = Release {
            name: "v0.12.5".to_string(),
            tag_name: "v0.12.5".to_string(),
            description: None,
            created_at: Utc.ymd(2023, 3, 25).and_hms_milli(23, 3, 17, 165),
            released_at: Utc.ymd(2023, 3, 25).and_hms_milli(23, 3, 17, 165),
            upcoming_release: false,
            author: Author {
                id: AuthorId::new(743516),
                name: "Charles Samborski".to_string(),
                username: "demurgos".to_string(),
                state: "active".to_string(),
                avatar_url: "https://secure.gravatar.com/avatar/4ccbb457b6d50bb79ec32b66c2f4e301?s=80&d=identicon".to_string(),
                web_url: "https://gitlab.com/demurgos".to_string(),
            },
            commit: Commit {
                id: "e8cfd320ee2b8dcee22b0bc0ceaf52d7dd667c2d".to_string(),
                short_id: "e8cfd320".to_string(),
                title: "[bin] Add support for precompiled binary".to_string(),
                created_at: Utc.ymd(2023, 3, 16).and_hms(23, 39, 35),
                parent_ids: vec![
                    "c8528d12e05c9ee803547eb556cb5394ac3d1abc".to_string(),
                ],
                message: "[bin] Add support for precompiled binary\n\nThis commit adds support for precompiled binaries built in GitLab CI.\n".to_string(),
                author_name: "Charles Samborski".to_string(),
                author_email: "demurgos@demurgos.net".to_string(),
                authored_date: Utc.ymd(2023, 3, 16).and_hms(22, 56, 54),
                committer_name: "Charles Samborski".to_string(),
                committer_email: "demurgos@demurgos.net".to_string(),
                committed_date: Utc.ymd(2023, 3, 16).and_hms(23, 39, 35),
            },
            milestones: None,
            commit_path: "/eternaltwin/eternaltwin/-/commit/e8cfd320ee2b8dcee22b0bc0ceaf52d7dd667c2d".to_string(),
            tag_path: "/eternaltwin/eternaltwin/-/tags/v0.12.5".to_string(),
            assets: ReleaseAssets {
                count: 5,
                sources: vec![
                    ReleaseSource {
                        format: "zip".to_string(),
                        url: "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.zip".to_string(),
                    },
                    ReleaseSource {
                        format: "tar.gz".to_string(),
                        url: "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.tar.gz".to_string(),
                    },
                    ReleaseSource {
                        format: "tar.bz2".to_string(),
                        url: "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.tar.bz2".to_string(),
                    },
                    ReleaseSource {
                        format: "tar".to_string(),
                        url: "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.tar".to_string(),
                    },
                ],
                links: vec![
                    ReleaseLink {
                        id: ReleaseLinkId::new(1492034),
                        name: "eternaltwin-x86_64-apple-darwin".to_string(),
                        url: "https://gitlab.com/eternaltwin/eternaltwin/-/packages/13511182".to_string(),
                        direct_asset_url: "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5/downloads/eternaltwin-x86_64-apple-darwin".to_string(),
                        link_type: ReleaseLinkType::Package,
                    }
                ],
            },
            evidences: vec![
                ReleaseEvidence {},
            ],
            _links: ReleaseLinks {
                closed_issues_url: "https://gitlab.com/eternaltwin/eternaltwin/-/issues?release_tag=v0.12.5&scope=all&state=closed".to_string(),
                closed_merge_requests_url: "https://gitlab.com/eternaltwin/eternaltwin/-/merge_requests?release_tag=v0.12.5&scope=all&state=closed".to_string(),
                edit_url: "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5/edit".to_string(),
                merged_merge_requests_url: "https://gitlab.com/eternaltwin/eternaltwin/-/merge_requests?release_tag=v0.12.5&scope=all&state=merged".to_string(),
                opened_issues_url: "https://gitlab.com/eternaltwin/eternaltwin/-/issues?release_tag=v0.12.5&scope=all&state=opened".to_string(),
                opened_merge_requests_url: "https://gitlab.com/eternaltwin/eternaltwin/-/merge_requests?release_tag=v0.12.5&scope=all&state=opened".to_string(),
                this: "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5".to_string(),
            },
        };
    assert_eq!(actual, expected);
  }

  #[cfg_attr(feature = "serde", test)]
  #[allow(deprecated)]
  fn read_release_link() {
    let raw = r#"{"id":1492019,"name":"eternaltwin-x86_64-unknown-linux-gnu","url":"https://gitlab.com/eternaltwin/eternaltwin/-/packages/13511234","direct_asset_url":"https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5/downloads/eternaltwin-x86_64-unknown-linux-gnu","link_type":"package"}"#;
    let actual: ReleaseLink = serde_json::from_str(raw).unwrap();
    let expected = ReleaseLink {
      id: ReleaseLinkId::new(1492019),
      name: "eternaltwin-x86_64-unknown-linux-gnu".to_string(),
      url: "https://gitlab.com/eternaltwin/eternaltwin/-/packages/13511234".to_string(),
      direct_asset_url:
        "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5/downloads/eternaltwin-x86_64-unknown-linux-gnu"
          .to_string(),
      link_type: ReleaseLinkType::Package,
    };
    assert_eq!(actual, expected);
  }
}
