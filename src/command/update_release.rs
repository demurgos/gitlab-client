use crate::common::project::ProjectRef;
use crate::GitlabAuth;
use chrono::{DateTime, Utc};

/// Update a project release
///
/// <https://docs.gitlab.com/ee/api/releases/#update-a-release>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UpdateReleaseCommand<Cx, Str = String> {
  pub context: Cx,
  pub auth: Option<GitlabAuth<Str>>,
  pub project: ProjectRef<Str>,
  pub tag_name: Str,
  pub name: Option<Str>,
  pub description: Option<Str>,
  // milestones: Vec<Str>,
  pub released_at: Option<DateTime<Utc>>,
}

pub type UpdateReleaseCommandView<'req, Cx> = UpdateReleaseCommand<&'req Cx, &'req str>;

impl<Cx, Str: AsRef<str>> UpdateReleaseCommand<Cx, Str> {
  pub fn as_view(&self) -> UpdateReleaseCommandView<'_, Cx> {
    UpdateReleaseCommandView {
      context: &self.context,
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
