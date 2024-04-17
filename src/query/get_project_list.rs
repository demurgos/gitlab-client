use crate::common::project::{ProjectId, ProjectOrderField};
use crate::common::topic::TopicId;
use crate::common::{AccessLevel, SortOrder, Visibility};
use chrono::{DateTime, Utc};
use compact_str::CompactString;

/// List all projects
///
/// <https://docs.gitlab.com/ee/api/projects.html#list-all-projects>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetProjectListQuery<ExtraInput, Str = CompactString> {
  pub extra_input: ExtraInput,
  pub archived: Option<bool>,
  pub id_after: Option<ProjectId>,
  pub id_before: Option<ProjectId>,
  pub imported: Option<bool>,
  pub include_hidden: Option<bool>,
  pub include_pending_delete: Option<bool>,
  pub last_activity_after: Option<DateTime<Utc>>,
  pub last_activity_before: Option<DateTime<Utc>>,
  pub membership: Option<bool>,
  pub min_access_level: Option<AccessLevel>,
  pub order_by: Option<ProjectOrderField>,
  pub owned: Option<bool>,
  pub repository_checksum_failed: Option<bool>,
  pub repository_storage: Option<Str>,
  pub search_namespaces: Option<bool>,
  pub search: Option<Str>,
  pub simple: Option<bool>,
  pub sort: Option<SortOrder>,
  pub starred: Option<bool>,
  pub statistics: Option<bool>,
  pub topic_ic: Option<TopicId>,
  pub topic: Vec<Str>,
  pub updated_after: Option<DateTime<Utc>>,
  pub updated_before: Option<DateTime<Utc>>,
  pub visibility: Option<Visibility>,
  pub wiki_checksum_failed: Option<bool>,
  pub with_custom_attributes: Option<bool>,
  pub with_issues_enabled: Option<bool>,
  pub with_merge_requests_enabled: Option<bool>,
  pub with_programming_language: Vec<Str>,
}

pub type GetProjectListQueryView<'req, ExtraInput> = GetProjectListQuery<&'req ExtraInput, &'req str>;

impl<ExtraInput, Str> GetProjectListQuery<ExtraInput, Str> {
  pub fn with_extra_input<NewExtraInput>(
    mut self,
    new_extra_input: NewExtraInput,
  ) -> GetProjectListQuery<NewExtraInput, Str> {
    GetProjectListQuery {
      extra_input: new_extra_input,
      archived: self.archived,
      id_after: self.id_after,
      id_before: self.id_before,
      imported: self.imported,
      include_hidden: self.include_hidden,
      include_pending_delete: self.include_pending_delete,
      last_activity_after: self.last_activity_after,
      last_activity_before: self.last_activity_before,
      membership: self.membership,
      min_access_level: self.min_access_level,
      order_by: self.order_by,
      owned: self.owned,
      repository_checksum_failed: self.repository_checksum_failed,
      repository_storage: self.repository_storage,
      search_namespaces: self.search_namespaces,
      search: self.search,
      simple: self.simple,
      sort: self.sort,
      starred: self.starred,
      statistics: self.statistics,
      topic_ic: self.topic_ic,
      topic: self.topic,
      updated_after: self.updated_after,
      updated_before: self.updated_before,
      visibility: self.visibility,
      wiki_checksum_failed: self.wiki_checksum_failed,
      with_custom_attributes: self.with_custom_attributes,
      with_issues_enabled: self.with_issues_enabled,
      with_merge_requests_enabled: self.with_merge_requests_enabled,
      with_programming_language: self.with_programming_language,
    }
  }

  pub fn as_view(&self) -> GetProjectListQueryView<'_, ExtraInput>
  where
    Str: AsRef<str>,
  {
    GetProjectListQueryView {
      extra_input: &self.extra_input,
      archived: self.archived,
      id_after: self.id_after,
      id_before: self.id_before,
      imported: self.imported,
      include_hidden: self.include_hidden,
      include_pending_delete: self.include_pending_delete,
      last_activity_after: self.last_activity_after,
      last_activity_before: self.last_activity_before,
      membership: self.membership,
      min_access_level: self.min_access_level,
      order_by: self.order_by,
      owned: self.owned,
      repository_checksum_failed: self.repository_checksum_failed,
      repository_storage: self.repository_storage.as_ref().map(|s| s.as_ref()),
      search_namespaces: self.search_namespaces,
      search: self.search.as_ref().map(|s| s.as_ref()),
      simple: self.simple,
      sort: self.sort,
      starred: self.starred,
      statistics: self.statistics,
      topic_ic: self.topic_ic,
      topic: Vec::from_iter(self.topic.iter().map(|s| s.as_ref())),
      updated_after: self.updated_after,
      updated_before: self.updated_before,
      visibility: self.visibility,
      wiki_checksum_failed: self.wiki_checksum_failed,
      with_custom_attributes: self.with_custom_attributes,
      with_issues_enabled: self.with_issues_enabled,
      with_merge_requests_enabled: self.with_merge_requests_enabled,
      with_programming_language: Vec::from_iter(self.topic.iter().map(|s| s.as_ref())),
    }
  }
}

impl<Str: AsRef<str>> GetProjectListQuery<(), Str> {
  pub const fn new() -> Self {
    Self {
      extra_input: (),
      archived: None,
      id_after: None,
      id_before: None,
      imported: None,
      include_hidden: None,
      include_pending_delete: None,
      last_activity_after: None,
      last_activity_before: None,
      membership: None,
      min_access_level: None,
      order_by: None,
      owned: None,
      repository_checksum_failed: None,
      repository_storage: None,
      search_namespaces: None,
      search: None,
      simple: None,
      sort: None,
      starred: None,
      statistics: None,
      topic_ic: None,
      topic: Vec::new(),
      updated_after: None,
      updated_before: None,
      visibility: None,
      wiki_checksum_failed: None,
      with_custom_attributes: None,
      with_issues_enabled: None,
      with_merge_requests_enabled: None,
      with_programming_language: Vec::new(),
    }
  }
}
