use crate::context::EmptyContext;
use crate::GitlabAuth;
use compact_str::CompactString;

/// Get a page from the project list
///
/// <https://docs.gitlab.com/ee/api/projects.html#list-all-projects>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetProjectListPageQuery<Cx, Str = CompactString> {
  pub context: Cx,
  pub auth: Option<GitlabAuth<Str>>,
  pub cursor: Str,
}

pub type GetProjectListPageQueryView<'req, Cx> = GetProjectListPageQuery<&'req Cx, &'req str>;

impl<Cx, Str> GetProjectListPageQuery<Cx, Str> {
  pub fn set_context<NewCx>(self, new_context: NewCx) -> GetProjectListPageQuery<NewCx, Str> {
    GetProjectListPageQuery {
      context: new_context,
      auth: self.auth,
      cursor: self.cursor,
    }
  }

  pub fn as_view(&self) -> GetProjectListPageQueryView<'_, Cx>
  where
    Str: AsRef<str>,
  {
    GetProjectListPageQueryView {
      context: &self.context,
      auth: self.auth.as_ref().map(GitlabAuth::as_view),
      cursor: self.cursor.as_ref(),
    }
  }
}

impl<Str: AsRef<str>> GetProjectListPageQuery<EmptyContext, Str> {
  pub const fn new(cursor: Str) -> Self {
    Self {
      context: EmptyContext::new(),
      auth: None,
      cursor,
    }
  }
}
