use crate::common::package::{PackageOrder, PackageType};
use crate::common::project::ProjectRef;
use crate::common::KeysetPagination;
use crate::context::EmptyContext;
use crate::{GitlabAuth, PackageStatus};
use compact_str::CompactString;

/// List project packages
///
/// <https://docs.gitlab.com/ee/api/packages.html#within-a-project>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetProjectPackageListQuery<Cx, Str = CompactString> {
  pub context: Cx,
  pub auth: Option<GitlabAuth<Str>>,
  pub pagination: Option<KeysetPagination<PackageOrder>>,
  pub project: ProjectRef<Str>,
  pub package_type: Option<PackageType>,
  pub package_name: Option<Str>,
  pub include_versionless: Option<bool>,
  pub status: Option<PackageStatus>,
}

pub type GetProjectPackageListQueryView<'req, Cx> = GetProjectPackageListQuery<&'req Cx, &'req str>;

impl<Cx, Str> GetProjectPackageListQuery<Cx, Str> {
  pub fn set_context<NewCx>(self, new_context: NewCx) -> GetProjectPackageListQuery<NewCx, Str> {
    GetProjectPackageListQuery {
      context: new_context,
      auth: self.auth,
      pagination: self.pagination,
      project: self.project,
      package_type: self.package_type,
      package_name: self.package_name,
      include_versionless: self.include_versionless,
      status: self.status,
    }
  }

  pub fn as_view(&self) -> GetProjectPackageListQueryView<'_, Cx>
  where
    Str: AsRef<str>,
  {
    GetProjectPackageListQueryView {
      context: &self.context,
      auth: self.auth.as_ref().map(GitlabAuth::as_view),
      pagination: self.pagination,
      project: self.project.as_view(),
      package_type: self.package_type,
      package_name: self.package_name.as_ref().map(|s| s.as_ref()),
      include_versionless: self.include_versionless,
      status: self.status,
    }
  }
}

impl<Str: AsRef<str>> GetProjectPackageListQuery<EmptyContext, Str> {
  pub const fn new(project: ProjectRef<Str>) -> Self {
    Self {
      context: EmptyContext::new(),
      auth: None,
      pagination: None,
      project,
      package_type: None,
      package_name: None,
      include_versionless: None,
      status: None,
    }
  }
}
