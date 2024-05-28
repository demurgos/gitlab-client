use std::ops::Deref;
use url::Url;

/// A very restricted version of frunk hlist to hold the context for gitlab client requests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context<TyGitlabUrl> {
  gitlab_url: TyGitlabUrl,
}

impl<TyGitlabUrl> Context<TyGitlabUrl> {
  pub fn set_gitlab_url<NewGitlabUrl>(self, gitlab_url: NewGitlabUrl) -> Context<NewGitlabUrl> {
    Context { gitlab_url }
  }
}

pub type EmptyContext = Context<()>;

impl EmptyContext {
  pub const fn new() -> Self {
    Self { gitlab_url: () }
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

impl GetRef<GitlabUrl> for Context<GitlabUrl> {
  fn get_ref(&self) -> &GitlabUrl {
    &self.gitlab_url
  }
}
