use compact_str::CompactString;

/// Criteria used to order tree records
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TreeRecordOrder {
  Name,
}

#[cfg_attr(
  feature = "serde",
  derive(serde::Serialize, serde::Deserialize),
  serde(rename_all = "lowercase")
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TreeRecordType {
  Tree,
  Blob,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TreeRecord<Str = CompactString> {
  pub id: Str,
  pub name: Str,
  pub r#type: TreeRecordType,
  pub path: Str,
  pub mode: Str,
}
