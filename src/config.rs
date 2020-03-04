use serde_derive::{Deserialize, Serialize};
use std::default::Default;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthConfig {
    pub(crate) ca: PathBuf,
    pub(crate) cert: PathBuf,
    pub(crate) key: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub(crate) endpoint: String,
    #[serde(flatten)]
    pub(crate) auth: Option<AuthConfig>,
}

impl Config {
    /// Create a new [`Config`](Config) with the given Falco gRPC endpoint.
    ///
    /// ```rust
    /// use falco::config::Config;
    /// let config = Config::new("localhost:5060");
    /// ```
    pub fn new(endpoint: &str) -> Self {
        Config {
            endpoint: endpoint.into(),
            auth: None,
        }
    }

    /// Set the certificate authority, certificate, and key locations for the [`Config`](Config).
    ///
    /// This is required by default from the Falco gRPC server.
    ///
    /// ```rust
    /// use falco::config::Config;
    /// let config = Config::new("localhost:5060")
    ///     .with_auth("ca.crt", "client.crt", "client.key");
    /// ```
    pub fn with_auth(
        mut self,
        ca: impl Into<PathBuf>,
        cert: impl Into<PathBuf>,
        key: impl Into<PathBuf>,
    ) -> Self {
        self.auth = Some(AuthConfig {
            ca: ca.into(),
            cert: cert.into(),
            key: key.into(),
        });
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            endpoint: "localhost:5060".to_owned(),
            auth: Some(AuthConfig {
                ca: PathBuf::from("/tmp/certs/ca.crt"),
                cert: PathBuf::from("/etc/certs/client.crt"),
                key: PathBuf::from("/etc/certs/client.key"),
            }),
        }
    }
}
