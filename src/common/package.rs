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

/// Criteria used to order packages
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PackageOrder {
  CreatedAt,
  Name,
  Version,
  Type,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PackageType {
  Composer,
  Conan,
  Generic,
  Golang,
  Helm,
  Maven,
  Npm,
  Nuget,
  Pypi,
  TerraformModule,
}

#[cfg(test)]
mod test {
  use super::*;
  use chrono::TimeZone;

  #[cfg_attr(feature = "serde", test)]
  #[allow(deprecated)]
  fn read_publish_package_file_response() {
    let raw = r#"{"id":72696499,"package_id":13447789,"created_at":"2023-03-22T21:37:30.948Z","updated_at":"2023-03-22T21:37:30.948Z","size":11,"file_store":2,"file_md5":null,"file_sha1":null,"file_name":"eternaltwin3","file":{"url":"https://storage.googleapis.com/gitlab-gprd-package-repo/4a/ac/4aac49a9b7a3711a7fc154e49d6751b1b0d75358c4657d14c0bd55291a20c250/packages/13447789/files/72696499/eternaltwin3?GoogleAccessId=gitlab-object-storage-prd@gitlab-production.iam.gserviceaccount.com\u0026Signature=RozKlG66y15cnKPi5kNZZ2PVdJlYlzCGJ7fOOGnPU9kkOHoQqqAY3EwIBqhE%0Aydqr4u%2FYx9A%2BaoNPv9XUNlqnyHl0BLzFYjh5hUwFktix8IwD%2BkMgbrhJN0Yj%0AcHbJsVrDBEDSyMRjNWaBDo%2BzeAe9exYQdQ9iOyQct3zLXTkEwjS5fnnBoF0L%0APohSUcBLOeTm0gzZJJ1n4UB5yPb7RoDPZelN%2BCZxNdcY2AMjahQcnLgnzbnJ%0AFEHliT9107dZU33iKFprMQGcqCKupTFO%2FIu11uik2S%2BKAlAOjZySsKiV8%2BBE%0A%2Bi2pttVA%2FEW2P8soM9ZtYj4ReP1N8ZPPpW0AG6iTzA%3D%3D\u0026Expires=1679521651"},"file_sha256":"0ca093111f402faa55be1cd71006270644b58619eb0c2408b97b7d24bb70dd09","verification_retry_at":null,"verified_at":null,"verification_failure":null,"verification_retry_count":null,"verification_checksum":null,"verification_state":0,"verification_started_at":null,"status":"default","new_file_path":null}"#;
    let actual: GenericPackageFile = serde_json::from_str(raw).unwrap();
    let expected = GenericPackageFile {
      id: 72696499,
      package_id: 13447789,
      created_at: Utc.ymd(2023, 3, 22).and_hms_milli(21, 37, 30, 948),
      updated_at: Utc.ymd(2023, 3, 22).and_hms_milli(21, 37, 30, 948),
      size: 11,
      file_store: 2,
      file_md5: None,
      file_sha1: None,
      file_name: CompactString::new("eternaltwin3"),
      file: GitlabFile {
        url: CompactString::new(
          r#"https://storage.googleapis.com/gitlab-gprd-package-repo/4a/ac/4aac49a9b7a3711a7fc154e49d6751b1b0d75358c4657d14c0bd55291a20c250/packages/13447789/files/72696499/eternaltwin3?GoogleAccessId=gitlab-object-storage-prd@gitlab-production.iam.gserviceaccount.com&Signature=RozKlG66y15cnKPi5kNZZ2PVdJlYlzCGJ7fOOGnPU9kkOHoQqqAY3EwIBqhE%0Aydqr4u%2FYx9A%2BaoNPv9XUNlqnyHl0BLzFYjh5hUwFktix8IwD%2BkMgbrhJN0Yj%0AcHbJsVrDBEDSyMRjNWaBDo%2BzeAe9exYQdQ9iOyQct3zLXTkEwjS5fnnBoF0L%0APohSUcBLOeTm0gzZJJ1n4UB5yPb7RoDPZelN%2BCZxNdcY2AMjahQcnLgnzbnJ%0AFEHliT9107dZU33iKFprMQGcqCKupTFO%2FIu11uik2S%2BKAlAOjZySsKiV8%2BBE%0A%2Bi2pttVA%2FEW2P8soM9ZtYj4ReP1N8ZPPpW0AG6iTzA%3D%3D&Expires=1679521651"#,
        ),
      },
      file_sha256: Some(CompactString::new(
        "0ca093111f402faa55be1cd71006270644b58619eb0c2408b97b7d24bb70dd09",
      )),
      verification_retry_at: None,
      verified_at: None,
      verification_failure: None,
      verification_retry_count: None,
      verification_checksum: None,
      verification_state: 0,
      verification_started_at: None,
      status: CompactString::new("default"),
      new_file_path: None,
    };
    assert_eq!(actual, expected);
  }
}
