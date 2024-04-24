use crate::common::group::GroupId;
use crate::common::namespace::Namespace;
use crate::common::user::{User, UserId};
use crate::common::{AccessLevel, Visibility};
use chrono::{DateTime, NaiveDate, Utc};
use compact_str::CompactString;
use url::Url;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectId(u64);

impl ProjectId {
  pub const fn new(id: u64) -> Self {
    Self(id)
  }

  pub const fn into_u64(self) -> u64 {
    self.0
  }

  /// Calls `f` with the string representation of this id as an argument.
  #[inline]
  pub fn with_str<R, F>(self, f: F) -> R
  where
    F: for<'a> FnOnce(&'a str) -> R,
  {
    let mut buf = ::itoa::Buffer::new();
    f(buf.format(self.0))
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectSlug<Slug = CompactString>(Slug);

impl<Slug: AsRef<str>> ProjectSlug<Slug> {
  pub fn new(slug: Slug) -> Self {
    Self(slug)
  }

  pub fn as_str(&self) -> &str {
    self.0.as_ref()
  }
}

pub type ProjectSlugView<'slug> = ProjectSlug<&'slug str>;

impl<Slug: AsRef<str>> ProjectSlug<Slug> {
  pub fn as_view(&self) -> ProjectSlugView<'_> {
    ProjectSlug(self.0.as_ref())
  }

  /// Calls `f` with the string representation of this slug as an argument.
  #[inline]
  pub fn with_str<R, F>(&self, f: F) -> R
  where
    F: for<'a> FnOnce(&'a str) -> R,
  {
    f(self.as_str())
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProjectRef<Slug = CompactString> {
  Id(ProjectId),
  Slug(ProjectSlug<Slug>),
}

pub type ProjectRefView<'slug> = ProjectRef<&'slug str>;

impl<Slug: AsRef<str>> ProjectRef<Slug> {
  pub fn as_view(&self) -> ProjectRefView<'_> {
    match self {
      Self::Id(id) => ProjectRef::Id(*id),
      Self::Slug(slug) => ProjectRef::Slug(slug.as_view()),
    }
  }

  /// Calls `f` with the string representation of this project ref as an argument.
  #[inline]
  pub fn with_str<R, F>(&self, f: F) -> R
  where
    F: for<'a> FnOnce(&'a str) -> R,
  {
    match self {
      Self::Id(id) => id.with_str(f),
      Self::Slug(slug) => slug.with_str(f),
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Project {
  // start of fields present in "SimpleProject"
  pub id: ProjectId,
  pub description: Option<CompactString>,
  pub name: CompactString,
  pub name_with_namespace: CompactString,
  pub path: CompactString,
  pub path_with_namespace: CompactString,
  pub created_at: DateTime<Utc>,
  pub default_branch: Option<CompactString>,
  pub tag_list: Vec<CompactString>,
  pub topics: Vec<CompactString>,
  pub ssh_url_to_repo: CompactString,
  pub http_url_to_repo: Url,
  pub web_url: Url,
  pub readme_url: Option<Url>,
  pub forks_count: Option<u64>,
  pub avatar_url: Option<CompactString>,
  pub star_count: u64,
  pub last_activity_at: DateTime<Utc>,
  pub namespace: Namespace,
  // end of fields present in "SimpleProject"
  pub container_registry_image_prefix: Option<CompactString>,
  pub _links: Option<ProjectLinks>,
  pub packages_enabled: Option<bool>,
  pub empty_repo: Option<bool>,
  pub archived: Option<bool>,
  pub visibility: Option<Visibility>,
  pub owner: Option<User>,
  pub resolve_outdated_diff_discussions: Option<bool>,
  pub container_expiration_policy: Option<ContainerExpirationPolicy>,
  pub repository_object_format: Option<CompactString>,
  pub issues_enabled: Option<bool>,
  pub merge_requests_enabled: Option<bool>,
  pub wiki_enabled: Option<bool>,
  pub jobs_enabled: Option<bool>,
  pub snippets_enabled: Option<bool>,
  pub container_registry_enabled: Option<bool>,
  pub service_desk_enabled: Option<bool>,
  pub service_desk_address: Option<CompactString>,
  pub can_create_merge_request_in: Option<bool>,
  pub issues_access_level: Option<CompactString>,
  pub repository_access_level: Option<CompactString>,
  pub merge_requests_access_level: Option<CompactString>,
  pub forking_access_level: Option<CompactString>,
  pub wiki_access_level: Option<CompactString>,
  pub builds_access_level: Option<CompactString>,
  pub snippets_access_level: Option<CompactString>,
  pub pages_access_level: Option<CompactString>,
  pub analytics_access_level: Option<CompactString>,
  pub container_registry_access_level: Option<CompactString>,
  pub security_and_compliance_access_level: Option<CompactString>,
  pub releases_access_level: Option<CompactString>,
  pub environments_access_level: Option<CompactString>,
  pub feature_flags_access_level: Option<CompactString>,
  pub infrastructure_access_level: Option<CompactString>,
  pub monitor_access_level: Option<CompactString>,
  pub model_experiments_access_level: Option<CompactString>,
  pub model_registry_access_level: Option<CompactString>,
  pub emails_disabled: Option<bool>,
  pub emails_enabled: Option<bool>,
  pub shared_runners_enabled: Option<bool>,
  pub lfs_enabled: Option<bool>,
  pub creator_id: Option<UserId>,
  pub forked_from_project: Option<Box<Project>>,
  pub mr_default_target_self: Option<bool>,
  pub import_url: Option<CompactString>,
  pub import_type: Option<CompactString>,
  pub import_status: Option<CompactString>,
  pub import_error: Option<CompactString>,
  pub open_issues_count: Option<u64>,
  pub description_html: Option<CompactString>,
  pub updated_at: Option<DateTime<Utc>>,
  pub ci_default_git_depth: Option<u64>,
  pub ci_forward_deployment_enabled: Option<bool>,
  pub ci_forward_deployment_rollback_allowed: Option<bool>,
  pub ci_job_token_scope_enabled: Option<bool>,
  pub ci_separated_caches: Option<bool>,
  pub ci_allow_fork_pipelines_to_run_in_parent_project: Option<bool>,
  pub build_git_strategy: Option<CompactString>,
  pub keep_latest_artifact: Option<bool>,
  pub restrict_user_defined_variables: Option<bool>,
  pub runners_token: Option<CompactString>,
  pub runner_token_expiration_interval: Option<CompactString>,
  pub group_runners_enabled: Option<bool>,
  pub auto_cancel_pending_pipelines: Option<CompactString>,
  pub build_timeout: Option<u64>,
  pub auto_devops_enabled: Option<bool>,
  pub auto_devops_deploy_strategy: Option<CompactString>,
  pub ci_config_path: Option<CompactString>,
  pub public_jobs: Option<bool>,
  pub shared_with_groups: Option<Vec<ProjectGroupShare>>,
  pub only_allow_merge_if_pipeline_succeeds: Option<bool>,
  pub allow_merge_on_skipped_pipeline: Option<bool>,
  pub request_access_enabled: Option<bool>,
  pub only_allow_merge_if_all_discussions_are_resolved: Option<bool>,
  pub remove_source_branch_after_merge: Option<bool>,
  pub printing_merge_request_link_enabled: Option<bool>,
  pub merge_method: Option<CompactString>,
  pub squash_option: Option<CompactString>,
  pub enforce_auth_checks_on_uploads: Option<bool>,
  pub suggestion_commit_message: Option<CompactString>,
  pub merge_commit_template: Option<CompactString>,
  pub squash_commit_template: Option<CompactString>,
  pub issue_branch_template: Option<CompactString>,
  pub warn_about_potentially_unwanted_characters: Option<bool>,
  pub autoclose_referenced_issues: Option<bool>,
  pub approvals_before_merge: Option<u64>,
  pub mirror: Option<bool>,
  pub external_authorization_classification_label: Option<CompactString>,
  pub marked_for_deletion_at: Option<NaiveDate>,
  pub marked_for_deletion_on: Option<NaiveDate>,
  pub requirements_enabled: Option<bool>,
  pub requirements_access_level: Option<CompactString>,
  pub security_and_compliance_enabled: Option<bool>,
  pub compliance_frameworks: Option<Vec<CompactString>>,
  pub issues_template: Option<CompactString>,
  pub merge_requests_template: Option<CompactString>,
  pub ci_restrict_pipeline_cancellation_role: Option<AccessLevel>,
  pub merge_pipelines_enabled: Option<bool>,
  pub merge_trains_enabled: Option<bool>,
  pub merge_trains_skip_train_allowed: Option<bool>,
  pub only_allow_merge_if_all_status_checks_passed: Option<bool>,
  pub allow_pipeline_trigger_approve_deployment: Option<bool>,
  pub prevent_merge_without_jira_issue: Option<bool>,
  pub permissions: Option<ProjectPermissions>,
}

/// Fields that can be used for project ordering
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProjectOrder {
  Id,
  Name,
  Path,
  CreatedAt,
  UpdatedAt,
  LastActivityAt,
  Similarity,
  RespositorySize,
  StorageSize,
  PackageSize,
  WikiSize,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectLinks {
  #[cfg_attr(feature = "serde", serde(rename = "self"))]
  pub this: CompactString,
  pub issues: Option<CompactString>,
  pub merge_requests: Option<CompactString>,
  pub repo_branches: Option<CompactString>,
  pub labels: Option<CompactString>,
  pub events: Option<CompactString>,
  pub members: Option<CompactString>,
  pub cluster_agents: Option<CompactString>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContainerExpirationPolicy {
  pub cadence: CompactString,
  pub enabled: bool,
  pub keep_n: Option<u64>,
  pub older_than: Option<CompactString>,
  pub name_regex: Option<CompactString>,
  pub name_regex_keep: Option<CompactString>,
  pub next_run_at: DateTime<Utc>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectPermissions {
  pub project_access: Option<ProjectAccess>,
  pub group_access: Option<CompactString>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectAccess {
  pub access_level: AccessLevel,
  pub notification_level: u64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectGroupShare {
  pub group_id: GroupId,
  pub group_name: CompactString,
  pub group_full_path: CompactString,
  pub group_access_level: AccessLevel,
}
