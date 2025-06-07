use compact_str::CompactString;
use std::num::NonZeroU8;

pub mod group;
pub mod namespace;
pub mod package;
pub mod project;
pub mod release;
pub mod topic;
pub mod tree;
pub mod user;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SortOrder {
  /// Ascending: lowest first, largest last
  Asc,
  /// Descending: largest first, lowest last
  Desc,
}

impl SortOrder {
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Asc => "asc",
      Self::Desc => "desc",
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Visibility {
  #[cfg_attr(feature = "serde", serde(rename = "public"))]
  Public,
  #[cfg_attr(feature = "serde", serde(rename = "internal"))]
  Internal,
  #[cfg_attr(feature = "serde", serde(rename = "private"))]
  Private,
}

/// Access Level, also referred as Role
///
/// See <https://docs.gitlab.com/ee/api/members.html#roles>
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

impl AccessLevel {
  pub const fn to_lower(&self) -> &'static str {
    match self {
      AccessLevel::NoAccess => "no_access",
      AccessLevel::MinimalAccess => "minimal_access",
      AccessLevel::Guest => "guest",
      AccessLevel::Reporter => "reporter",
      AccessLevel::Developer => "developer",
      AccessLevel::Maintainer => "maintainer",
      AccessLevel::Owner => "owner",
    }
  }

  pub fn from_lower(input: &str) -> Option<Self> {
    match input {
      "no_access" => Some(AccessLevel::NoAccess),
      "minimal_access" => Some(AccessLevel::MinimalAccess),
      "guest" => Some(AccessLevel::Guest),
      "reporter" => Some(AccessLevel::Reporter),
      "developer" => Some(AccessLevel::Developer),
      "maintainer" => Some(AccessLevel::Maintainer),
      "owner" => Some(AccessLevel::Owner),
      _ => None,
    }
  }

  pub const fn to_u8(&self) -> u8 {
    match self {
      AccessLevel::NoAccess => 0,
      AccessLevel::MinimalAccess => 5,
      AccessLevel::Guest => 10,
      AccessLevel::Reporter => 20,
      AccessLevel::Developer => 30,
      AccessLevel::Maintainer => 40,
      AccessLevel::Owner => 50,
    }
  }

  pub const fn from_u8(input: u8) -> Option<Self> {
    match input {
      0 => Some(AccessLevel::NoAccess),
      5 => Some(AccessLevel::MinimalAccess),
      10 => Some(AccessLevel::Guest),
      20 => Some(AccessLevel::Reporter),
      30 => Some(AccessLevel::Developer),
      40 => Some(AccessLevel::Maintainer),
      50 => Some(AccessLevel::Owner),
      _ => None,
    }
  }
}

#[cfg(feature = "serde")]
impl serde::Serialize for AccessLevel {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(self.to_lower())
  }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AccessLevel {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    use serde::de::Error;
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum StrOrInt<'a> {
      Str(std::borrow::Cow<'a, str>),
      Int(u8),
    }
    let val = StrOrInt::deserialize(deserializer)?;
    let val = match val {
      StrOrInt::Str(s) => Self::from_lower(s.as_ref()),
      StrOrInt::Int(i) => Self::from_u8(i),
    };
    val.ok_or_else(|| D::Error::custom("unexpected `AccessLevel` value"))
  }
}

/// See <https://docs.gitlab.com/ee/api/rest/index.html#keyset-based-pagination>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeysetPagination<TyOrderBy> {
  /// Number of items to list per page (default: `20`, max: `100`).
  pub per_page: Option<NonZeroU8>,
  // todo: optional
  pub order_by: TyOrderBy,
  // todo: optional
  pub sort: SortOrder,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Page<T> {
  /// Cursor to the first page
  pub first: Option<CompactString>,
  /// Cursor to the next page
  pub next: Option<CompactString>,
  /// Cursor to the last page
  pub last: Option<CompactString>,
  pub items: Vec<T>,
}
