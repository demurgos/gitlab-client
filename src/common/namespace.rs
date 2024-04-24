use compact_str::CompactString;
use url::Url;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamespaceId(u64);

impl NamespaceId {
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Namespace {
  pub id: NamespaceId,
  pub name: CompactString,
  pub path: CompactString,
  pub kind: NamespaceKind,
  pub full_path: CompactString,
  pub parent_id: Option<NamespaceId>,
  pub web_url: Url,
  pub avatar_url: Option<CompactString>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NamespaceKind {
  #[cfg_attr(feature = "serde", serde(rename = "group"))]
  Group,
  #[cfg_attr(feature = "serde", serde(rename = "user"))]
  User,
}
