use crate::common::project::ProjectRef;
use crate::GitlabAuth;
use serde::{Deserialize, Serialize};

/// Get a project release
///
/// <https://docs.gitlab.com/ee/api/releases/#get-a-release-by-a-tag-name>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetProjectReleaseQuery<Cx, Str = String> {
  pub context: Cx,
  pub auth: Option<GitlabAuth<Str>>,
  pub project: ProjectRef<Str>,
  pub tag_name: Str,
  pub include_html_description: bool,
}

pub type GetProjectReleaseQueryView<'req, Cx> = GetProjectReleaseQuery<&'req Cx, &'req str>;

impl<Cx, Str: AsRef<str>> GetProjectReleaseQuery<Cx, Str> {
  pub fn as_view(&self) -> GetProjectReleaseQueryView<'_, Cx> {
    GetProjectReleaseQueryView {
      context: &self.context,
      auth: self.auth.as_ref().map(GitlabAuth::as_view),
      project: self.project.as_view(),
      tag_name: self.tag_name.as_ref(),
      include_html_description: self.include_html_description,
    }
  }
}
