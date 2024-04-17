use crate::{GitlabAuth, InputPackageStatus, ProjectRef};

/// Publish a generic package file
///
/// <https://docs.gitlab.com/ee/user/packages/generic_packages/#publish-a-package-file>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PublishPackageFileRequest<Str = String, Bytes = Vec<u8>, const SELECT: bool = false> {
    pub auth: Option<GitlabAuth<Str>>,
    pub project: ProjectRef<Str>,
    pub package_name: Str,
    pub package_version: Str,
    pub filename: Str,
    pub status: InputPackageStatus,
    pub data: Bytes,
}

pub type PublishPackageFileRequestView<'req, const SELECT: bool> =
PublishPackageFileRequest<&'req str, &'req [u8], SELECT>;

impl<Str: AsRef<str>, Bytes: AsRef<[u8]>, const SELECT: bool> PublishPackageFileRequest<Str, Bytes, SELECT> {
    pub fn as_view(&self) -> PublishPackageFileRequestView<'_, SELECT> {
        PublishPackageFileRequestView {
            auth: self.auth.as_ref().map(GitlabAuth::as_view),
            project: self.project.as_view(),
            package_name: self.package_name.as_ref(),
            package_version: self.package_version.as_ref(),
            filename: self.filename.as_ref(),
            status: self.status,
            data: self.data.as_ref(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum PublishPackageFileError<Inner> {
    #[error("failed to send `PublishPackageFile` request: {0}")]
    Send(String),
    #[error("failed to receive `PublishPackageFile` response: {0}")]
    Receive(String),
    #[error("failed to parse `PublishPackageFile` response with body = {1}: {0}")]
    ResponseFormat(String, String),
    #[error("`PublishPackageFile` is forbidden for provided auth")]
    Forbidden,
    #[error("unexpected `PublishPackageFile` error: {0}")]
    Other(String),
    #[error(transparent)]
    Inner(Inner),
}
