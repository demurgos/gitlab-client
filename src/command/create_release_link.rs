use crate::common::project::ProjectRef;
use crate::common::release::ReleaseLinkType;
use crate::GitlabAuth;

/// Create a project release link
///
/// <https://docs.gitlab.com/ee/api/releases/links.html#create-a-release-link>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateReleaseLinkCommand<Cx, Str = String> {
  pub context: Cx,
  pub auth: Option<GitlabAuth<Str>>,
  pub project: ProjectRef<Str>,
  pub tag_name: Str,
  pub name: Str,
  pub url: Str,
  pub direct_asset_path: Option<Str>,
  pub link_type: ReleaseLinkType,
}

pub type CreateReleaseLinkCommandView<'req, Cx> = CreateReleaseLinkCommand<&'req Cx, &'req str>;

impl<Cx, Str: AsRef<str>> CreateReleaseLinkCommand<Cx, Str> {
  pub fn as_view(&self) -> CreateReleaseLinkCommandView<'_, Cx> {
    CreateReleaseLinkCommandView {
      context: &self.context,
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
