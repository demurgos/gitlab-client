use crate::common::project::ProjectRef;
use crate::common::release::{InputReleaseAssets, InputReleaseAssetsView, InputReleaseLink};
use crate::GitlabAuth;
use chrono::{DateTime, Utc};
use compact_str::CompactString;

/// Create a project release
///
/// <https://docs.gitlab.com/ee/api/releases/#create-a-release>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateReleaseCommand<Cx, Str = CompactString, Assets = InputReleaseAssets> {
  pub context: Cx,
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

pub type CreateReleaseCommandView<'req, Cx, Str> =
  CreateReleaseCommand<&'req Cx, &'req str, InputReleaseAssetsView<'req, Str>>;

impl<Cx, Str: AsRef<str>, Links> CreateReleaseCommand<Cx, Str, InputReleaseAssets<Links>>
where
  Links: AsRef<[InputReleaseLink<Str>]>,
{
  pub fn as_view(&self) -> CreateReleaseCommandView<'_, Cx, Str> {
    CreateReleaseCommandView {
      context: &self.context,
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
