use compact_str::CompactString;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;

mod namespace;
pub mod project;
pub mod topic;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SortOrder {
  Asc,
  Desc,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Visibility {
  Public,
  Internal,
  Private,
}

/// Access Level, also referred as Role
///
/// See <https://docs.gitlab.com/ee/api/members.html#roles>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AccessLevel {
  NoAccess = 0,
  MinimalAccess = 5,
  Guest = 10,
  Reporter = 20,
  Developer = 30,
  Maintainer = 40,
  Owner = 50,
}

/// See <https://docs.gitlab.com/ee/api/rest/index.html#keyset-based-pagination>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeysetPagination<TyOrderBy> {
  /// Number of items to list per page (default: `20`, max: `100`).
  pub per_page: Option<NonZeroU8>,
  pub order_by: TyOrderBy,
  pub sort: SortOrder,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Page<T> {
  /// Cursor to the first page
  pub first: Option<CompactString>,
  /// Cursor to the next page
  pub next: Option<CompactString>,
  pub items: Vec<T>,
}
