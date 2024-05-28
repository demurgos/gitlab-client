use crate::common::project::ProjectRef;
use crate::common::tree::TreeRecordOrder;
use crate::common::KeysetPagination;
use crate::context::EmptyContext;
use crate::GitlabAuth;
use compact_str::CompactString;

/// Get a list of repository files and directories in a project.
///
/// <https://docs.gitlab.com/ee/api/repositories.html#list-repository-tree>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetTreeRecordListQuery<Cx, Str = CompactString> {
  pub context: Cx,
  pub auth: Option<GitlabAuth<Str>>,
  pub project: ProjectRef<Str>,
  pub pagination: Option<KeysetPagination<TreeRecordOrder>>,
  pub path: Option<Str>,
  pub recursive: Option<bool>,
  pub r#ref: Option<Str>,
}

pub type GetTreeRecordListQueryView<'req, Cx> = GetTreeRecordListQuery<&'req Cx, &'req str>;

impl<Cx, Str: AsRef<str>> GetTreeRecordListQuery<Cx, Str> {
  pub fn set_context<NewCx>(self, new_context: NewCx) -> GetTreeRecordListQuery<NewCx, Str> {
    GetTreeRecordListQuery {
      context: new_context,
      auth: self.auth,
      project: self.project,
      pagination: self.pagination,
      path: self.path,
      recursive: self.recursive,
      r#ref: self.r#ref,
    }
  }

  pub fn as_view(&self) -> GetTreeRecordListQueryView<'_, Cx> {
    GetTreeRecordListQueryView {
      context: &self.context,
      auth: self.auth.as_ref().map(GitlabAuth::as_view),
      project: self.project.as_view(),
      pagination: self.pagination,
      path: self.path.as_ref().map(AsRef::as_ref),
      recursive: self.recursive,
      r#ref: self.r#ref.as_ref().map(AsRef::as_ref),
    }
  }
}

impl<Str: AsRef<str>> GetTreeRecordListQuery<EmptyContext, Str> {
  pub const fn new(project: ProjectRef<Str>) -> Self {
    Self {
      context: EmptyContext::new(),
      auth: None,
      project,
      pagination: None,
      path: None,
      recursive: None,
      r#ref: None,
    }
  }
}
