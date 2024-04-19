use crate::common::project::ProjectRef;
use crate::common::release::ReleaseLinkType;
use crate::GitlabAuth;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
