use chrono::{DateTime, Utc};
use compact_str::CompactString;
use url::Url;
use crate::common::namespace::Namespace;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectId(u64);

impl ProjectId {
  pub const fn new(id: u64) -> Self {
    Self(id)
  }

  pub const fn into_u64(self) -> u64 {
    self.0
  }

  /// Calls `f` with the string representation of this id as an argument.
  #[inline]
  pub fn with_str<R, F>(self, f: F) -> R
    where
      F: for<'a> FnOnce(&'a str) -> R,
  {
    let mut buf = ::itoa::Buffer::new();
    f(buf.format(self.0))
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectSlug<Slug = CompactString>(Slug);

impl<Slug: AsRef<str>> ProjectSlug<Slug> {
  pub fn new(slug: Slug) -> Self {
    Self(slug)
  }

  pub fn as_str(&self) -> &str {
    self.0.as_ref()
  }
}

pub type ProjectSlugView<'slug> = ProjectSlug<&'slug str>;

impl<Slug: AsRef<str>> ProjectSlug<Slug> {
  pub fn as_view(&self) -> ProjectSlugView<'_> {
    ProjectSlug(self.0.as_ref())
  }

  /// Calls `f` with the string representation of this slug as an argument.
  #[inline]
  pub fn with_str<R, F>(&self, f: F) -> R
    where
      F: for<'a> FnOnce(&'a str) -> R,
  {
    f(self.as_str())
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProjectRef<Slug = CompactString> {
  Id(ProjectId),
  Slug(ProjectSlug<Slug>),
}

pub type ProjectRefView<'slug> = ProjectRef<&'slug str>;

impl<Slug: AsRef<str>> ProjectRef<Slug> {
  pub fn as_view(&self) -> ProjectRefView<'_> {
    match self {
      Self::Id(id) => ProjectRef::Id(*id),
      Self::Slug(slug) => ProjectRef::Slug(slug.as_view()),
    }
  }

  /// Calls `f` with the string representation of this project ref as an argument.
  #[inline]
  pub fn with_str<R, F>(&self, f: F) -> R
    where
      F: for<'a> FnOnce(&'a str) -> R,
  {
    match self {
      Self::Id(id) => id.with_str(f),
      Self::Slug(slug) => slug.with_str(f),
    }
  }
}


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Project {
  pub id: ProjectId,
  pub description: Option<CompactString>,
  pub name: CompactString,
  pub name_with_namespace: CompactString,
  pub path: CompactString,
  pub path_with_namespace: CompactString,
  pub created_at: DateTime<Utc>,
  pub default_branch: CompactString,
  pub tag_list: Vec<CompactString>,
  pub topics: Vec<CompactString>,
  pub ssh_url_to_repo: CompactString,
  pub http_url_to_repo: Url,
  pub web_url: Url,
  pub readme_url: Option<Url>,
  pub forks_count: u64,
  pub avatar_url: Option<CompactString>,
  pub star_count: u64,
  pub last_activity_at: DateTime<Utc>,
  pub namespace: Namespace,
}

/// Fields that can be used for project ordering
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProjectOrderField {
  Id,
  Name,
  Path,
  CreatedAt,
  UpdatedAt,
  LastActivityAt,
  Similarity,
  RespositorySize,
  StorageSize,
  PackageSize,
  WikiSize,
}
