use std::time::Duration;

use crate::{types::ApiError, Error};
use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;
use regex_lite::Regex;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Certificate,
};
use serde::{Deserialize, Serialize};
use sysinfo::{ProcessExt, System, SystemExt};

type Port = u16;

#[derive(Debug, Clone)]
pub struct Client {
    port: Port,
    client: reqwest::Client,
}

impl Client {
    /// Creates a client for connecting to the LCU api
    ///
    /// # Errors
    /// Fails if the client process is not running
    pub fn new() -> Result<Self, Error> {
        let port_re = Regex::new(r"--app-port=([0-9]*)")?;
        let auth_token_re = Regex::new(r"--remoting-auth-token=([\w-]*)")?;

        let mut sys = System::new_all();
        sys.refresh_all();

        let cmd_args = sys
            .processes()
            .values()
            .find(|p| p.name() == "LeagueClientUx.exe")
            .map(|p| p.cmd().join(" "))
            .ok_or(Error::ClientNotFound)?;

        let port = port_re
            .captures(&cmd_args)
            .and_then(|x| x.get(1))
            .map(|x| x.as_str().parse())
            .ok_or(Error::PortNotFound)??;
        let auth_token = auth_token_re
            .captures(&cmd_args)
            .and_then(|x| x.get(1))
            .map(|x| x.as_str().to_owned())
            .ok_or(Error::PortNotFound)?;
        let encoded_auth_token = BASE64_STANDARD_NO_PAD.encode(format!("riot:{auth_token}"));

        let cert = Certificate::from_pem(include_bytes!("../riotgames.pem"))?;
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("Basic {encoded_auth_token}").as_str())?,
        );
        let client = reqwest::ClientBuilder::new()
            // Fast detection of client being closed
            .connect_timeout(Duration::new(0, 100_000_000))
            .add_root_certificate(cert)
            .default_headers(headers)
            .build()?;

        Ok(Client { port, client })
    }

    pub(crate) async fn get<T: for<'a> Deserialize<'a>>(&self, endpoint: &str) -> Result<T, Error> {
        deserialize_response(
            &self
                .client
                .get(format!("https://127.0.0.1:{}{endpoint}", self.port))
                .send()
                .await?
                .bytes()
                .await?,
        )
    }

    pub(crate) async fn post<T: for<'a> Deserialize<'a>, R: Serialize>(
        &self,
        endpoint: &str,
        body: &Option<R>,
    ) -> Result<T, Error> {
        deserialize_response(
            &self
                .client
                .post(format!("https://127.0.0.1:{}{endpoint}", self.port))
                .json(&body)
                .send()
                .await?
                .bytes()
                .await?,
        )
    }
}

fn deserialize_response<T: for<'a> Deserialize<'a>>(body: &[u8]) -> Result<T, Error> {
    if let Ok(val) = serde_json::from_slice(body) {
        Ok(val)
    } else {
        let api_error = serde_json::from_slice::<ApiError>(body)?;
        Err(Error::ApiError(api_error))
    }
}
