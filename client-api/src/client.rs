use crate::{types::ApiResult, Error};
use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Certificate, Url,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use sysinfo::System;

#[derive(Debug, Clone)]
pub struct Client {
    base_url: Url,
    client: reqwest::Client,
}

impl Client {
    /// Creates a client for connecting to the LCU api
    ///
    /// # Errors
    /// Fails if the client process is not running
    pub fn new() -> Result<Self, Error> {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cmd_args = sys
            .processes()
            .values()
            .find(|p| p.name() == "LeagueClientUx.exe")
            .map(|p| p.cmd().join(" "))
            .ok_or(Error::ClientNotFound)?;

        let port = cmd_args
            .split(' ')
            .find_map(|x| x.strip_prefix("--app-port="))
            .map(str::parse)
            .ok_or(Error::PortNotFound)??;
        let mut base_url = Url::parse("https://127.0.0.1")?;
        base_url
            .set_port(Some(port))
            .map_err(|()| Error::InvalidPort(port))?;

        let auth_token = cmd_args
            .split(' ')
            .find_map(|x| x.strip_prefix("--remoting-auth-token="))
            .ok_or(Error::AuthNotFound)?;
        let encoded_auth_token = BASE64_STANDARD_NO_PAD.encode(format!("riot:{auth_token}"));

        let cert = Certificate::from_pem(include_bytes!("../riotgames.pem"))?;
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("Basic {encoded_auth_token}").as_str())?,
        );
        let client = reqwest::ClientBuilder::new()
            // Fast detection of client being closed
            .connect_timeout(Duration::from_millis(100))
            .add_root_certificate(cert)
            .default_headers(headers)
            .build()?;

        Ok(Self { base_url, client })
    }

    pub(crate) async fn get<T: for<'a> Deserialize<'a>, U: Serialize + Sync + ?Sized>(
        &self,
        endpoint: &str,
        query: &U,
    ) -> Result<T, Error> {
        log::info!("GET {endpoint}");
        let mut url = self.base_url.clone();
        url.set_path(endpoint);
        self.client
            .get(url)
            .query(query)
            .send()
            .await?
            .json::<ApiResult<T>>()
            .await?
            .into()
    }

    pub(crate) async fn post<T: for<'a> Deserialize<'a>, R: Serialize + Sync>(
        &self,
        endpoint: &str,
        body: &Option<R>,
    ) -> Result<T, Error> {
        log::info!("POST {endpoint}");
        let mut url = self.base_url.clone();
        url.set_path(endpoint);
        self.client
            .post(url)
            .json(body)
            .send()
            .await?
            .json::<ApiResult<T>>()
            .await?
            .into()
    }

    pub(crate) async fn patch_empty_response<R: Serialize + Sync>(
        &self,
        endpoint: &str,
        body: &R,
    ) -> Result<(), Error> {
        log::info!("PATCH {endpoint}");
        let mut url = self.base_url.clone();
        url.set_path(endpoint);
        self.client.patch(url).json(body).send().await?;
        Ok(())
    }

    pub(crate) async fn put_empty_response<R: Serialize + Sync>(
        &self,
        endpoint: &str,
        body: &R,
    ) -> Result<(), Error> {
        log::info!("PUT {endpoint}");
        let mut url = self.base_url.clone();
        url.set_path(endpoint);
        self.client.put(url).json(body).send().await?;
        Ok(())
    }
}
