use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;
use eyre::{ContextCompat, Result, WrapErr};
use regex_lite::Regex;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Certificate,
};
use serde::{Deserialize, Serialize};
use sysinfo::{ProcessExt, System, SystemExt};

type Port = u16;

pub struct Client {
    port: Port,
    client: reqwest::blocking::Client,
}

impl Client {
    /// Creates a client for connecting to the LCU api
    ///
    /// # Errors
    /// Fails if the client process is not running
    pub fn new() -> Result<Self> {
        let port_re = Regex::new(r"--app-port=([0-9]*)")?;
        let auth_token_re = Regex::new(r"--remoting-auth-token=([\w-]*)")?;

        let mut sys = System::new_all();
        sys.refresh_all();

        let cmd_args = sys
            .processes()
            .values()
            .find(|p| p.name() == "LeagueClientUx.exe")
            .map(|p| p.cmd().join(" "))
            .context("Failed to find LCU process")?;

        let port = port_re
            .captures(&cmd_args)
            .and_then(|x| x.get(1))
            .map(|x| x.as_str().parse())
            .context("Failed to parse port")??;
        let auth_token = auth_token_re
            .captures(&cmd_args)
            .and_then(|x| x.get(1))
            .map(|x| x.as_str().to_owned())
            .context("Failed to parse auth token")?;
        let encoded_auth_token = BASE64_STANDARD_NO_PAD.encode(format!("riot:{auth_token}"));

        let cert = Certificate::from_pem(include_bytes!("./riotgames.pem"))?;
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("Basic {encoded_auth_token}").as_str())?,
        );
        let client = reqwest::blocking::ClientBuilder::new()
            .add_root_certificate(cert)
            .default_headers(headers)
            .build()?;

        Ok(Client { port, client })
    }

    pub(crate) fn get<T: for<'a> Deserialize<'a>>(&self, endpoint: &str) -> Result<T> {
        self.client
            .get(format!("https://127.0.0.1:{}{endpoint}", self.port))
            .send()?
            .json()
            .wrap_err("Failed to deserialize response")
    }

    pub(crate) fn post<T: for<'a> Deserialize<'a>, R: Serialize>(
        &self,
        endpoint: &str,
        body: &Option<R>,
    ) -> Result<T> {
        // let mut req_builder = self
        //     .client
        //     .post(format!("https://127.0.0.1:{}{endpoint}", self.port));
        //
        // if let Some(b) = body {
        //     req_builder = req_builder.json(&b);
        // }
        //
        // req_builder.send()?.json().into()
        self.client
            .post(format!("https://127.0.0.1:{}{endpoint}", self.port))
            .json(&body)
            .send()?
            .json()
            .wrap_err("Failed to deserialize response")
    }
}
