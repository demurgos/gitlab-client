use chrono::{DateTime, Utc};
use compact_str::CompactString;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GenericPackageFile {
  pub id: u64,
  pub package_id: u64,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub size: u64,
  pub file_store: u64,
  pub file_md5: Option<CompactString>,
  pub file_sha1: Option<CompactString>,
  pub file_name: CompactString,
  pub file: GitlabFile,
  pub file_sha256: Option<CompactString>,
  pub verification_retry_at: Option<DateTime<Utc>>,
  pub verified_at: Option<DateTime<Utc>>,
  pub verification_failure: Option<CompactString>,
  pub verification_retry_count: Option<u64>,
  pub verification_checksum: Option<CompactString>,
  pub verification_state: u64,
  pub verification_started_at: Option<DateTime<Utc>>,
  pub status: CompactString,
  // TODO: PackageStatus
  pub new_file_path: Option<CompactString>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GitlabFile {
  pub url: CompactString,
}
