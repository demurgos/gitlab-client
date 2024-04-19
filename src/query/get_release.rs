use crate::common::project::ProjectRef;
use crate::GitlabAuth;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
