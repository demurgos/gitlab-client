use crate::common::project::ProjectRef;
use crate::{GitlabAuth, InputPackageStatus};
use compact_str::CompactString;

/// Publish a generic package file
///
/// <https://docs.gitlab.com/ee/user/packages/generic_packages/#publish-a-package-file>
///
/// `SELECT` contols if `select=package_file` is applied
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PublishPackageFileCommand<Cx, Str = CompactString, Bytes = Vec<u8>, const SELECT: bool = false> {
  pub context: Cx,
  pub auth: Option<GitlabAuth<Str>>,
  pub project: ProjectRef<Str>,
  pub package_name: Str,
  pub package_version: Str,
  pub filename: Str,
  pub status: InputPackageStatus,
  pub data: Bytes,
}

pub type PublishPackageFileCommandView<'req, Cx, const SELECT: bool> =
  PublishPackageFileCommand<&'req Cx, &'req str, &'req [u8], SELECT>;

impl<Cx, Str: AsRef<str>, Bytes: AsRef<[u8]>, const SELECT: bool> PublishPackageFileCommand<Cx, Str, Bytes, SELECT> {
  pub fn as_view(&self) -> PublishPackageFileCommandView<'_, Cx, SELECT> {
    PublishPackageFileCommandView {
      context: &self.context,
      auth: self.auth.as_ref().map(GitlabAuth::as_view),
      project: self.project.as_view(),
      package_name: self.package_name.as_ref(),
      package_version: self.package_version.as_ref(),
      filename: self.filename.as_ref(),
      status: self.status,
      data: self.data.as_ref(),
    }
  }
}
