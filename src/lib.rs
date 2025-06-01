pub use ::chrono;
pub use ::compact_str;
#[cfg(feature = "http")]
pub use ::demurgos_headers::UserAgent;
#[cfg(feature = "reqwest")]
pub use ::reqwest;
#[cfg(feature = "serde")]
pub use ::serde;
pub use ::tower_service;
pub use ::url;
use compact_str::CompactString;
use std::future::Future;

use crate::common::project::Project;
use crate::common::Page;
use crate::query::get_project_list::GetProjectListQuery;
use tower_service::Service;

pub mod client;
pub mod command;
pub mod common;
pub mod context;
#[cfg(feature = "http")]
pub mod http;
pub mod query;
pub mod url_util;

pub trait GitlabClient<Cx>: Send + Sync {
  type Error<'req>
  where
    Cx: 'req;
  fn get_project_list(
    self,
    query: &GetProjectListQuery<Cx>,
  ) -> impl Send + Future<Output = Result<Page<Project>, Self::Error<'_>>>;
}

impl<S, Cx> GitlabClient<Cx> for &'_ mut S
where
  Self: Send + Sync,
  Cx: 'static + Send + Sync,
  for<'req> S: Service<&'req GetProjectListQuery<Cx>, Response = Page<Project>>,
  for<'req> <S as Service<&'req GetProjectListQuery<Cx>>>::Future: Send,
{
  type Error<'req>
    = <S as Service<&'req GetProjectListQuery<Cx>>>::Error
  where
    Cx: 'req;

  async fn get_project_list(self, query: &GetProjectListQuery<Cx>) -> Result<Page<Project>, Self::Error<'_>> {
    self.call(query).await
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputPackageStatus {
  Default,
  Hidden,
}

impl InputPackageStatus {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Default => "default",
      Self::Hidden => "hidden",
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PackageStatus {
  Default,
  Hidden,
  Processing,
  Error,
  PendingDestruction,
}

impl PackageStatus {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Default => "default",
      Self::Hidden => "hidden",
      Self::Processing => "processing",
      Self::Error => "error",
      Self::PendingDestruction => "pending_destruction",
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GitlabAuth<Token = CompactString> {
  PrivateToken(Token),
  JobToken(Token),
}

pub type GitlabAuthView<'s> = GitlabAuth<&'s str>;

impl<Token: AsRef<str>> GitlabAuth<Token> {
  pub fn as_view(&self) -> GitlabAuthView<'_> {
    match self {
      Self::PrivateToken(token) => GitlabAuth::PrivateToken(token.as_ref()),
      Self::JobToken(token) => GitlabAuth::JobToken(token.as_ref()),
    }
  }

  pub fn http_header(&self) -> (&'static str, &str) {
    match self {
      Self::PrivateToken(token) => ("PRIVATE-TOKEN", token.as_ref()),
      Self::JobToken(token) => ("JOB-TOKEN", token.as_ref()),
    }
  }
}
