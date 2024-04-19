use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Release {
  pub name: String,
  pub tag_name: String,
  pub description: Option<String>,
  pub created_at: DateTime<Utc>,
  pub released_at: DateTime<Utc>,
  pub upcoming_release: bool,
  pub author: Author,
  pub commit: Commit,
  pub milestones: Option<Vec<Milestone>>,
  pub commit_path: String,
  pub tag_path: String,
  pub assets: ReleaseAssets,
  pub evidences: Vec<ReleaseEvidence>,
  pub _links: ReleaseLinks,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthorId(u64);

impl AuthorId {
  pub const fn new(id: u64) -> Self {
    Self(id)
  }

  pub const fn into_u64(self) -> u64 {
    self.0
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Author {
  pub id: AuthorId,
  pub name: String,
  pub username: String,
  pub state: String,
  pub avatar_url: String,
  pub web_url: String,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Commit {
  pub id: String,
  pub short_id: String,
  pub title: String,
  pub created_at: DateTime<Utc>,
  pub parent_ids: Vec<String>,
  pub message: String,
  pub author_name: String,
  pub author_email: String,
  pub authored_date: DateTime<Utc>,
  pub committer_name: String,
  pub committer_email: String,
  pub committed_date: DateTime<Utc>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Milestone {
  // TODO
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseAssets {
  pub count: u64,
  pub sources: Vec<ReleaseSource>,
  pub links: Vec<ReleaseLink>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseSource {
  pub format: String,
  pub url: String,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseEvidence {
  // TODO
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseLinks {
  closed_issues_url: String,
  closed_merge_requests_url: String,
  edit_url: String,
  merged_merge_requests_url: String,
  opened_issues_url: String,
  opened_merge_requests_url: String,
  #[cfg_attr(feature = "serde", serde(rename = "self"))]
  this: String,
}

/// Criteria used to order releases
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReleaseOrder {
  ReleasedAt,
  CreatedAt,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseLinkId(u64);

impl ReleaseLinkId {
  pub const fn new(id: u64) -> Self {
    Self(id)
  }

  pub const fn into_u64(self) -> u64 {
    self.0
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseLink {
  pub id: ReleaseLinkId,
  pub name: String,
  pub url: String,
  pub direct_asset_url: String,
  pub link_type: ReleaseLinkType,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReleaseLinkType {
  #[cfg_attr(feature = "serde", serde(rename = "other"))]
  Other,
  #[cfg_attr(feature = "serde", serde(rename = "runbook"))]
  Runbook,
  #[cfg_attr(feature = "serde", serde(rename = "image"))]
  Image,
  #[cfg_attr(feature = "serde", serde(rename = "package"))]
  Package,
}

impl ReleaseLinkType {
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Other => "other",
      Self::Runbook => "runbook",
      Self::Image => "image",
      Self::Package => "package",
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputReleaseAssets<Links = Vec<InputReleaseLink<String>>> {
  pub links: Links,
}

pub type InputReleaseAssetsView<'req, Str = String> = InputReleaseAssets<&'req [InputReleaseLink<Str>]>;

impl<Links> InputReleaseAssets<Links> {
  pub fn as_view<Str>(&self) -> InputReleaseAssetsView<'_, Str>
  where
    Links: AsRef<[InputReleaseLink<Str>]>,
  {
    InputReleaseAssetsView {
      links: self.links.as_ref(),
    }
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputReleaseLink<Str = String> {
  pub name: Str,
  pub url: Str,
  pub direct_asset_path: Option<Str>,
  pub link_type: ReleaseLinkType,
}

pub type InputReleaseLinkView<'req> = InputReleaseLink<&'req str>;

impl<Str: AsRef<str>> InputReleaseLink<Str> {
  pub fn as_view(&self) -> InputReleaseLinkView<'_> {
    InputReleaseLinkView {
      name: self.name.as_ref(),
      url: self.url.as_ref(),
      direct_asset_path: self.direct_asset_path.as_ref().map(|s| s.as_ref()),
      link_type: self.link_type,
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use chrono::{TimeZone, Utc};

  #[cfg_attr(feature = "serde", test)]
  #[allow(deprecated)]
  fn read_release() {
    let raw = r#"{"name": "v0.12.5","tag_name": "v0.12.5","description": null,"created_at": "2023-03-25T23:03:17.165Z","released_at": "2023-03-25T23:03:17.165Z","upcoming_release": false,"author": {"id": 743516,"username": "demurgos","name": "Charles Samborski","state": "active","avatar_url": "https://secure.gravatar.com/avatar/4ccbb457b6d50bb79ec32b66c2f4e301?s=80&d=identicon","web_url": "https://gitlab.com/demurgos"},"commit": {"id": "e8cfd320ee2b8dcee22b0bc0ceaf52d7dd667c2d","short_id": "e8cfd320","created_at": "2023-03-17T00:39:35.000+01:00","parent_ids": ["c8528d12e05c9ee803547eb556cb5394ac3d1abc"],"title": "[bin] Add support for precompiled binary","message": "[bin] Add support for precompiled binary\n\nThis commit adds support for precompiled binaries built in GitLab CI.\n","author_name": "Charles Samborski","author_email": "demurgos@demurgos.net","authored_date": "2023-03-16T23:56:54.000+01:00","committer_name": "Charles Samborski","committer_email": "demurgos@demurgos.net","committed_date": "2023-03-17T00:39:35.000+01:00","trailers": {},"web_url": "https://gitlab.com/eternaltwin/eternaltwin/-/commit/e8cfd320ee2b8dcee22b0bc0ceaf52d7dd667c2d"},"commit_path": "/eternaltwin/eternaltwin/-/commit/e8cfd320ee2b8dcee22b0bc0ceaf52d7dd667c2d","tag_path": "/eternaltwin/eternaltwin/-/tags/v0.12.5","assets": {"count": 5,"sources": [{"format": "zip","url": "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.zip"},{"format": "tar.gz","url": "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.tar.gz"},{"format": "tar.bz2","url": "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.tar.bz2"},{"format": "tar","url": "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.tar"}],"links": [{"id": 1492034,"name": "eternaltwin-x86_64-apple-darwin","url": "https://gitlab.com/eternaltwin/eternaltwin/-/packages/13511182","direct_asset_url": "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5/downloads/eternaltwin-x86_64-apple-darwin","link_type": "package"}]},"evidences": [{"sha": "f9021712d926f78fc9272ac24733492facd5fb4353ed","filepath": "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5/evidences/4460894.json","collected_at": "2023-03-25T23:03:17.328Z"}],"_links": {"closed_issues_url": "https://gitlab.com/eternaltwin/eternaltwin/-/issues?release_tag=v0.12.5&scope=all&state=closed","closed_merge_requests_url": "https://gitlab.com/eternaltwin/eternaltwin/-/merge_requests?release_tag=v0.12.5&scope=all&state=closed","edit_url": "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5/edit","merged_merge_requests_url": "https://gitlab.com/eternaltwin/eternaltwin/-/merge_requests?release_tag=v0.12.5&scope=all&state=merged","opened_issues_url": "https://gitlab.com/eternaltwin/eternaltwin/-/issues?release_tag=v0.12.5&scope=all&state=opened","opened_merge_requests_url": "https://gitlab.com/eternaltwin/eternaltwin/-/merge_requests?release_tag=v0.12.5&scope=all&state=opened","self": "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5"}}"#;
    let actual: Release = serde_json::from_str(raw).unwrap();
    let expected = Release {
      name: "v0.12.5".to_string(),
      tag_name: "v0.12.5".to_string(),
      description: None,
      created_at: Utc.ymd(2023, 3, 25).and_hms_milli(23, 3, 17, 165),
      released_at: Utc.ymd(2023, 3, 25).and_hms_milli(23, 3, 17, 165),
      upcoming_release: false,
      author: Author {
        id: AuthorId::new(743516),
        name: "Charles Samborski".to_string(),
        username: "demurgos".to_string(),
        state: "active".to_string(),
        avatar_url: "https://secure.gravatar.com/avatar/4ccbb457b6d50bb79ec32b66c2f4e301?s=80&d=identicon".to_string(),
        web_url: "https://gitlab.com/demurgos".to_string(),
      },
      commit: Commit {
        id: "e8cfd320ee2b8dcee22b0bc0ceaf52d7dd667c2d".to_string(),
        short_id: "e8cfd320".to_string(),
        title: "[bin] Add support for precompiled binary".to_string(),
        created_at: Utc.ymd(2023, 3, 16).and_hms(23, 39, 35),
        parent_ids: vec![
          "c8528d12e05c9ee803547eb556cb5394ac3d1abc".to_string(),
        ],
        message: "[bin] Add support for precompiled binary\n\nThis commit adds support for precompiled binaries built in GitLab CI.\n".to_string(),
        author_name: "Charles Samborski".to_string(),
        author_email: "demurgos@demurgos.net".to_string(),
        authored_date: Utc.ymd(2023, 3, 16).and_hms(22, 56, 54),
        committer_name: "Charles Samborski".to_string(),
        committer_email: "demurgos@demurgos.net".to_string(),
        committed_date: Utc.ymd(2023, 3, 16).and_hms(23, 39, 35),
      },
      milestones: None,
      commit_path: "/eternaltwin/eternaltwin/-/commit/e8cfd320ee2b8dcee22b0bc0ceaf52d7dd667c2d".to_string(),
      tag_path: "/eternaltwin/eternaltwin/-/tags/v0.12.5".to_string(),
      assets: ReleaseAssets {
        count: 5,
        sources: vec![
          ReleaseSource {
            format: "zip".to_string(),
            url: "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.zip".to_string(),
          },
          ReleaseSource {
            format: "tar.gz".to_string(),
            url: "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.tar.gz".to_string(),
          },
          ReleaseSource {
            format: "tar.bz2".to_string(),
            url: "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.tar.bz2".to_string(),
          },
          ReleaseSource {
            format: "tar".to_string(),
            url: "https://gitlab.com/eternaltwin/eternaltwin/-/archive/v0.12.5/eternaltwin-v0.12.5.tar".to_string(),
          },
        ],
        links: vec![
          ReleaseLink {
            id: ReleaseLinkId::new(1492034),
            name: "eternaltwin-x86_64-apple-darwin".to_string(),
            url: "https://gitlab.com/eternaltwin/eternaltwin/-/packages/13511182".to_string(),
            direct_asset_url: "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5/downloads/eternaltwin-x86_64-apple-darwin".to_string(),
            link_type: ReleaseLinkType::Package,
          }
        ],
      },
      evidences: vec![
        ReleaseEvidence {},
      ],
      _links: ReleaseLinks {
        closed_issues_url: "https://gitlab.com/eternaltwin/eternaltwin/-/issues?release_tag=v0.12.5&scope=all&state=closed".to_string(),
        closed_merge_requests_url: "https://gitlab.com/eternaltwin/eternaltwin/-/merge_requests?release_tag=v0.12.5&scope=all&state=closed".to_string(),
        edit_url: "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5/edit".to_string(),
        merged_merge_requests_url: "https://gitlab.com/eternaltwin/eternaltwin/-/merge_requests?release_tag=v0.12.5&scope=all&state=merged".to_string(),
        opened_issues_url: "https://gitlab.com/eternaltwin/eternaltwin/-/issues?release_tag=v0.12.5&scope=all&state=opened".to_string(),
        opened_merge_requests_url: "https://gitlab.com/eternaltwin/eternaltwin/-/merge_requests?release_tag=v0.12.5&scope=all&state=opened".to_string(),
        this: "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5".to_string(),
      },
    };
    assert_eq!(actual, expected);
  }

  #[cfg_attr(feature = "serde", test)]
  #[allow(deprecated)]
  fn read_release_link() {
    let raw = r#"{"id":1492019,"name":"eternaltwin-x86_64-unknown-linux-gnu","url":"https://gitlab.com/eternaltwin/eternaltwin/-/packages/13511234","direct_asset_url":"https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5/downloads/eternaltwin-x86_64-unknown-linux-gnu","link_type":"package"}"#;
    let actual: ReleaseLink = serde_json::from_str(raw).unwrap();
    let expected = ReleaseLink {
      id: ReleaseLinkId::new(1492019),
      name: "eternaltwin-x86_64-unknown-linux-gnu".to_string(),
      url: "https://gitlab.com/eternaltwin/eternaltwin/-/packages/13511234".to_string(),
      direct_asset_url:
        "https://gitlab.com/eternaltwin/eternaltwin/-/releases/v0.12.5/downloads/eternaltwin-x86_64-unknown-linux-gnu"
          .to_string(),
      link_type: ReleaseLinkType::Package,
    };
    assert_eq!(actual, expected);
  }
}
