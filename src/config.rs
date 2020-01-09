use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub(crate) endpoint: String,
    pub(crate) ca: Option<PathBuf>,
    pub(crate) cert: Option<PathBuf>,
    pub(crate) key: Option<PathBuf>,
}

impl Config {
    /// Create a new [`Config`](Config) with the given Falco gRPC endpoint.
    ///
    /// ```rust
    /// # use falco::client::Config;
    /// let config = Config::new("localhost:5060");
    /// ```
    pub fn new(endpoint: String) -> Self {
        Config {
            endpoint: endpoint,
            ca: None,
            cert: None,
            key: None,
        }
    }

    /// Set the certificate authority, certificate, and key locations for the [`Config`](Config).
    ///
    /// This is required by default from the Falco gRPC server.
    ///
    /// ```rust
    /// # use falco::client::Config;
    /// let config = Config::new("localhost:5060")
    ///     .with_auth("root.crt", "client.crt", "client.key");
    /// ```
    pub fn with_auth(
        mut self,
        ca: impl Into<PathBuf>,
        cert: impl Into<PathBuf>,
        key: impl Into<PathBuf>,
    ) -> Self {
        self.ca = Some(ca.into());
        self.cert = Some(cert.into());
        self.key = Some(key.into());
        self
    }
}
