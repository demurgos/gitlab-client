use crate::common::project::ProjectRef;
use crate::context::EmptyContext;
use crate::GitlabAuth;
use compact_str::CompactString;

/// Get a single project
///
/// <https://docs.gitlab.com/ee/api/projects.html#get-single-project>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetProjectQuery<Cx, Str = CompactString> {
  pub context: Cx,
  pub auth: Option<GitlabAuth<Str>>,
  pub id: ProjectRef<Str>,
  pub license: Option<bool>,
  pub statistics: Option<bool>,
  pub with_custom_attributes: Option<bool>,
}

pub type GetProjectQueryView<'req, Cx> = GetProjectQuery<&'req Cx, &'req str>;

impl<Cx, Str> GetProjectQuery<Cx, Str> {
  pub fn set_context<NewCx>(self, new_context: NewCx) -> GetProjectQuery<NewCx, Str> {
    GetProjectQuery {
      context: new_context,
      auth: self.auth,
      id: self.id,
      license: self.license,
      statistics: self.statistics,
      with_custom_attributes: self.with_custom_attributes,
    }
  }

  pub fn as_view(&self) -> GetProjectQueryView<'_, Cx>
  where
    Str: AsRef<str>,
  {
    GetProjectQueryView {
      context: &self.context,
      auth: self.auth.as_ref().map(GitlabAuth::as_view),
      id: self.id.as_view(),
      license: self.license,
      statistics: self.statistics,
      with_custom_attributes: self.with_custom_attributes,
    }
  }
}

impl<Str: AsRef<str>> GetProjectQuery<EmptyContext, Str> {
  pub const fn new(id: ProjectRef<Str>) -> Self {
    Self {
      context: EmptyContext::new(),
      auth: None,
      id,
      license: None,
      statistics: None,
      with_custom_attributes: None,
    }
  }
}
