use std::ops::Deref;
use url::Url;

/// A very restricted version of frunk hlist to hold the context for gitlab client requests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context<TyGitlabUrl, TyUserAgent> {
  gitlab_url: TyGitlabUrl,
  user_agent: TyUserAgent,
}

impl<TyGitlabUrl, TyUserAgent> Context<TyGitlabUrl, TyUserAgent> {
  pub fn set_gitlab_url<NewGitlabUrl>(self, gitlab_url: NewGitlabUrl) -> Context<NewGitlabUrl, TyUserAgent> {
    Context {
      gitlab_url,
      user_agent: self.user_agent,
    }
  }

  pub fn set_user_agent<NewUserAgent>(self, user_agent: NewUserAgent) -> Context<TyGitlabUrl, NewUserAgent> {
    Context {
      gitlab_url: self.gitlab_url,
      user_agent,
    }
  }
}

pub type EmptyContext = Context<(), ()>;

impl EmptyContext {
  pub const fn new() -> Self {
    Self {
      gitlab_url: (),
      user_agent: (),
    }
  }
}

impl Default for EmptyContext {
  fn default() -> Self {
    Self::new()
  }
}

pub trait GetRef<T: ?Sized> {
  fn get_ref(&self) -> &T;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GitlabUrl(pub Url);

impl Deref for GitlabUrl {
  type Target = Url;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<TyUserAgent> GetRef<GitlabUrl> for Context<GitlabUrl, TyUserAgent> {
  fn get_ref(&self) -> &GitlabUrl {
    &self.gitlab_url
  }
}

#[cfg(feature = "http")]
impl<TyGitlabUrl> GetRef<demurgos_headers::UserAgent> for Context<TyGitlabUrl, demurgos_headers::UserAgent> {
  fn get_ref(&self) -> &demurgos_headers::UserAgent {
    &self.user_agent
  }
}
