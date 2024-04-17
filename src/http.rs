use crate::{
  CreateReleaseError, CreateReleaseLinkError, CreateReleaseLinkRequestView, CreateReleaseRequestView,
  GenericPackageFile, GetPackageFileError, GetPackageFileRequestView, GetReleaseError, GetReleaseRequestView,
  GitlabAuth, GitlabAuthView, GitlabClient, InputPackageStatus, InputReleaseAssetsView, PublishPackageFileError,
  PublishPackageFileRequestView, Release, ReleaseLink,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::Serialize;
use url::Url;

pub struct HttpGitlabClient {
  client: Client,
  server: Url,
}

impl HttpGitlabClient {
  pub fn new() -> Self {
    Self {
      client: Client::builder()
        .user_agent("gitlab_client")
        .build()
        .expect("building the client always succeeds"),
      server: Url::parse("https://gitlab.com/").expect("the gitlab URL is well-formed"),
    }
  }

  pub fn api_url<I>(&self, segments: I) -> Url
  where
    I: IntoIterator,
    I::Item: AsRef<str>,
  {
    let mut res = self.server.clone();
    {
      let mut p = res.path_segments_mut().expect("GitLab URL has path segments");
      p.extend(["api", "v4"]);
      p.extend(segments);
    }
    res
  }
}

impl Default for HttpGitlabClient {
  fn default() -> Self {
    Self::new()
  }
}

#[async_trait]
impl GitlabClient for HttpGitlabClient {
  async fn publish_package_file(
    &self,
    req: PublishPackageFileRequestView<'_, true>,
  ) -> Result<GenericPackageFile, PublishPackageFileError> {
    let mut url = req.project.with_str(|project| {
      self.api_url([
        "projects",
        project,
        "packages",
        "generic",
        req.package_name,
        req.package_version,
        req.filename,
      ])
    });
    {
      let mut q = url.query_pairs_mut();
      q.append_pair(
        "status",
        match req.status {
          InputPackageStatus::Default => "default",
          InputPackageStatus::Hidden => "hidden",
        },
      );
      q.append_pair("select", "package_file");
    }
    let res = self
      .client
      .put(url)
      .gitlab_auth(req.auth)
      .body(req.data.to_vec())
      .send()
      .await
      .map_err(|e| PublishPackageFileError::Send(format!("{e:?}")))?;
    match res.status() {
      StatusCode::OK | StatusCode::CREATED => {
        let body = res
          .text()
          .await
          .map_err(|e| PublishPackageFileError::Receive(format!("{e:?}")))?;
        let body: GenericPackageFile =
          serde_json::from_str(&body).map_err(|e| PublishPackageFileError::ResponseFormat(format!("{e:?}"), body))?;
        Ok(body)
      }
      StatusCode::FORBIDDEN => Err(PublishPackageFileError::Forbidden),
      code => Err(PublishPackageFileError::Receive(format!(
        "unexpected status code: {}",
        code
      ))),
    }
  }

  async fn get_package_file(&self, req: GetPackageFileRequestView<'_>) -> Result<Vec<u8>, GetPackageFileError> {
    let url = req.project.with_str(|project| {
      self.api_url([
        "projects",
        project,
        "packages",
        "generic",
        req.package_name,
        req.package_version,
        req.filename,
      ])
    });
    let res = self
      .client
      .get(url)
      .gitlab_auth(req.auth)
      .send()
      .await
      .map_err(|e| GetPackageFileError::Send(format!("{e:?}")))?;
    let body: Vec<u8> = res
      .bytes()
      .await
      .map_err(|e| GetPackageFileError::Receive(format!("{e:?}")))?
      .to_vec();
    Ok(body)
  }

  async fn create_release(&self, req: CreateReleaseRequestView<'_, String>) -> Result<Release, CreateReleaseError> {
    #[derive(Debug, Serialize)]
    struct Body<'r> {
      name: Option<&'r str>,
      tag_name: &'r str,
      tag_message: Option<&'r str>,
      description: Option<&'r str>,
      r#ref: Option<&'r str>,
      assets: InputReleaseAssetsView<'r>,
      released_at: Option<DateTime<Utc>>,
    }

    let url = req
      .project
      .with_str(|project| self.api_url(["projects", project, "releases"]));
    let res = self
      .client
      .post(url)
      .gitlab_auth(req.auth)
      .json(&Body {
        name: req.name,
        tag_name: req.tag_name,
        tag_message: req.tag_message,
        description: req.description,
        r#ref: req.r#ref,
        assets: req.assets,
        released_at: req.released_at,
      })
      .send()
      .await
      .map_err(|e| CreateReleaseError::Send(format!("{e:?}")))?;

    if res.status() == StatusCode::CONFLICT {
      return Err(CreateReleaseError::AlreadyExists);
    }

    match res.status() {
      StatusCode::OK | StatusCode::CREATED => {
        let body = res
          .text()
          .await
          .map_err(|e| CreateReleaseError::Receive(format!("{e:?}")))?;
        let body: Release =
          serde_json::from_str(&body).map_err(|e| CreateReleaseError::ResponseFormat(format!("{e:?}"), body))?;
        Ok(body)
      }
      code => Err(CreateReleaseError::Receive(format!("unexpected status code: {}", code))),
    }
  }

  async fn get_release(&self, req: GetReleaseRequestView<'_>) -> Result<Release, GetReleaseError> {
    let url = req
      .project
      .with_str(|project| self.api_url(["projects", project, "releases", req.tag_name]));
    let res = self
      .client
      .get(url)
      .gitlab_auth(req.auth)
      .send()
      .await
      .map_err(|e| GetReleaseError::Send(format!("{e:?}")))?;

    match res.status() {
      StatusCode::OK => {
        let body: Release = res
          .json()
          .await
          .map_err(|e| GetReleaseError::Receive(format!("{e:?}")))?;
        Ok(body)
      }
      StatusCode::NOT_FOUND => Err(GetReleaseError::NotFound),
      code => Err(GetReleaseError::Receive(format!("unexpected status code: {}", code))),
    }
  }

  async fn create_release_link(
    &self,
    req: CreateReleaseLinkRequestView<'_>,
  ) -> Result<ReleaseLink, CreateReleaseLinkError> {
    #[derive(Debug, Serialize)]
    struct Body<'r> {
      name: &'r str,
      url: &'r str,
      direct_asset_path: Option<&'r str>,
      link_type: &'r str,
    }

    let url = req
      .project
      .with_str(|project| self.api_url(["projects", project, "releases", req.tag_name, "assets", "links"]));
    let res = self
      .client
      .post(url)
      .gitlab_auth(req.auth)
      .json(&Body {
        name: req.name,
        url: req.url,
        direct_asset_path: req.direct_asset_path,
        link_type: req.link_type.as_str(),
      })
      .send()
      .await
      .map_err(|e| CreateReleaseLinkError::Send(format!("{e:?}")))?;

    match res.status() {
      StatusCode::OK | StatusCode::CREATED => {
        let body = res
          .text()
          .await
          .map_err(|e| CreateReleaseLinkError::Receive(format!("{e:?}")))?;
        let body: ReleaseLink =
          serde_json::from_str(&body).map_err(|e| CreateReleaseLinkError::ResponseFormat(format!("{e:?}"), body))?;
        Ok(body)
      }
      code => Err(CreateReleaseLinkError::Receive(format!(
        "unexpected status code: {}",
        code
      ))),
    }
  }
}

trait RequestBuilderExt {
  fn gitlab_auth(self, gl_auth: Option<GitlabAuthView<'_>>) -> Self;
}

impl RequestBuilderExt for RequestBuilder {
  fn gitlab_auth(self, gl_auth: Option<GitlabAuthView<'_>>) -> Self {
    match gl_auth {
      None => self,
      Some(GitlabAuth::PrivateToken(token)) => self.header("PRIVATE-TOKEN", token),
      Some(GitlabAuth::JobToken(token)) => self.header("JOB-TOKEN", token),
    }
  }
}
