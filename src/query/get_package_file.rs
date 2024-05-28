use crate::common::project::ProjectRef;
use crate::GitlabAuth;
use compact_str::CompactString;

/// Get a generic package file
///
/// <https://docs.gitlab.com/ee/user/packages/generic_packages/#download-package-file>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetPackageFileQuery<Cx, Str = CompactString> {
  pub context: Cx,
  pub auth: Option<GitlabAuth<Str>>,
  pub project: ProjectRef<Str>,
  pub package_name: Str,
  pub package_version: Str,
  pub filename: Str,
}

pub type GetPackageFileQueryView<'req, Cx> = GetPackageFileQuery<&'req Cx, &'req str>;

impl<Cx, Str: AsRef<str>> GetPackageFileQuery<Cx, Str> {
  pub fn as_view(&self) -> GetPackageFileQueryView<'_, Cx> {
    GetPackageFileQueryView {
      context: &self.context,
      auth: self.auth.as_ref().map(GitlabAuth::as_view),
      project: self.project.as_view(),
      package_name: self.package_name.as_ref(),
      package_version: self.package_version.as_ref(),
      filename: self.filename.as_ref(),
    }
  }
}
