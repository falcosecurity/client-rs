use std::sync::Arc;

use crate::api::output_grpc;
use crate::{certs, config, Error, Result};
use grpcio::{ChannelBuilder, ChannelCredentialsBuilder, Environment};

pub trait Connect {
    fn connect(env: Arc<Environment>, config: config::Config) -> Result<grpcio::Channel>;
}

pub struct FalcoConnect {
    pub env: Arc<Environment>,
}

impl Connect for FalcoConnect {
    fn connect(env: Arc<Environment>, config: config::Config) -> Result<grpcio::Channel> {
        if let (Some(ca_path), Some(cert_path), Some(key_path)) =
            (&config.ca, &config.cert, &config.key)
        {
            let root_cert = certs::load_pem_file(ca_path.as_ref())?;
            let client_cert = certs::load_pem_file(cert_path.as_ref())?;
            let client_key = certs::load_pem_file(key_path.as_ref())?;

            let credentials = ChannelCredentialsBuilder::new()
                // Set the PEM encoded server root cert to verify server's identity
                .root_cert(root_cert)
                // Set the PEM encoded client side cert and key
                .cert(client_cert, client_key)
                // Create channel credentials
                .build();

            Ok(ChannelBuilder::new(env).secure_connect(config.endpoint.as_str(), credentials))
        } else {
            Err(Error::internal_error(
                "something wrong during client configuration",
            ))
        }
    }
}
//TODO(fntlnz,leodido): keepalive, timeout, reconnect ?

#[derive(Clone)]
pub struct Client {
    channel: grpcio::Channel,
}

impl Client {
    pub fn new(config: config::Config) -> Client {
        // TODO(fntlnz): make the completion queue configurable
        let env = Arc::new(Environment::new(2));
        let channel = FalcoConnect::connect(env, config);
        Client { channel: channel }
    }

    pub fn outputs(&self) -> output_grpc::ServiceClient {
        output_grpc::ServiceClient::new(self.channel.clone())
    }
}
