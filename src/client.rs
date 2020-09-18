use std::sync::Arc;

use crate::api::outputs_grpc;
use crate::{certs, config, Result};
use grpcio::{ChannelBuilder, ChannelCredentialsBuilder, Environment};

pub trait Connect {
    fn connect(env: Arc<Environment>, config: config::Config) -> Result<grpcio::Channel>;
}

pub struct FalcoConnect {
    pub env: Arc<Environment>,
}

impl Connect for FalcoConnect {
    fn connect(env: Arc<Environment>, config: config::Config) -> Result<grpcio::Channel> {
        let auth = config
            .auth
            .as_ref()
            .ok_or(internal_err!("unencrypted connections are not supported"))?;

        let root_cert = certs::load_pem_file(auth.ca.as_ref())?;
        let client_cert = certs::load_pem_file(auth.cert.as_ref())?;
        let client_key = certs::load_pem_file(auth.key.as_ref())?;

        let credentials = ChannelCredentialsBuilder::new()
            // Set the PEM encoded server root cert to verify server's identity
            .root_cert(root_cert)
            // Set the PEM encoded client side cert and key
            .cert(client_cert, client_key)
            // Create channel credentials
            .build();

        Ok(ChannelBuilder::new(env).secure_connect(config.endpoint.as_str(), credentials))
    }
}

// TODO(fntlnz,leodido): make the completion queue configurable
// TODO(fntlnz,leodido): keepalive, timeout, reconnect ?

#[derive(Clone)]
pub struct Client {
    channel: grpcio::Channel,
}

impl Client {
    pub fn new(config: config::Config) -> Result<Client> {
        let env = Arc::new(Environment::new(2));
        let channel = FalcoConnect::connect(env, config)?;
        Ok(Client { channel })
    }

    pub fn outputs(&self) -> outputs_grpc::ServiceClient {
        outputs_grpc::ServiceClient::new(self.channel.clone())
    }
}
