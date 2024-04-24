#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TopicId(u64);

impl TopicId {
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
