use crate::common::project::ProjectRef;
use crate::common::release::ReleaseOrder;
use crate::common::KeysetPagination;
use crate::context::EmptyContext;
use crate::GitlabAuth;
use compact_str::CompactString;

/// List project releases
///
/// <https://docs.gitlab.com/ee/api/releases/#list-releases>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetProjectReleaseListQuery<Cx, Str = CompactString> {
  pub context: Cx,
  pub auth: Option<GitlabAuth<Str>>,
  pub pagination: Option<KeysetPagination<ReleaseOrder>>,
  pub project: ProjectRef<Str>,
  pub include_html_description: Option<bool>,
}

pub type GetProjectReleaseListQueryView<'req, Cx> = GetProjectReleaseListQuery<&'req Cx, &'req str>;

impl<Cx, Str> GetProjectReleaseListQuery<Cx, Str> {
  pub fn set_context<NewCx>(self, new_context: NewCx) -> GetProjectReleaseListQuery<NewCx, Str> {
    GetProjectReleaseListQuery {
      context: new_context,
      auth: self.auth,
      pagination: self.pagination,
      project: self.project,
      include_html_description: self.include_html_description,
    }
  }

  pub fn as_view(&self) -> GetProjectReleaseListQueryView<'_, Cx>
  where
    Str: AsRef<str>,
  {
    GetProjectReleaseListQueryView {
      context: &self.context,
      auth: self.auth.as_ref().map(GitlabAuth::as_view),
      pagination: self.pagination,
      project: self.project.as_view(),
      include_html_description: self.include_html_description,
    }
  }
}

impl<Str: AsRef<str>> GetProjectReleaseListQuery<EmptyContext, Str> {
  pub const fn new(project: ProjectRef<Str>) -> Self {
    Self {
      context: EmptyContext::new(),
      auth: None,
      pagination: None,
      project,
      include_html_description: None,
    }
  }
}
