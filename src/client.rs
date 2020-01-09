use std::sync::Arc;

/// A result holding an [`Error`](Error).
// pub type Result<T> = result::Result<T, dyn Error>;
use crate::config;
// use futures::*;
use crate::api::output_grpc;
use grpcio::{ChannelBuilder, ChannelCredentialsBuilder, Environment};
use std::{fs::File, io::Read, path::Path};

pub trait Connect {
    fn connect(env: Arc<Environment>, config: config::Config) -> grpcio::Channel;
}

pub struct FalcoConnect {
    pub env: Arc<Environment>,
}

fn check_pem_file(path: &Path) -> Result<File, std::io::Error> {
    File::open(path)
    // .map_err(|e| internal_err!("failed to open {} to load {}: {:?}", path.display(), tag, e))
}

fn load_pem_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    let mut file = check_pem_file(path)?;
    let mut key = vec![];
    file.read_to_end(&mut key)
        // .map_err(|e| {
        //     internal_err!(
        //         "failed to load {} from path {}: {:?}",
        //         tag,
        //         path.display(),
        //         e
        //     )
        // })
        .map(|_| key)
}

impl Connect for FalcoConnect {
    fn connect(env: Arc<Environment>, config: config::Config) -> grpcio::Channel {
        let credentials = ChannelCredentialsBuilder::new()
            // Set the PEM encoded server root cert to verify server's identity
            .root_cert(load_pem_file(config.ca.unwrap().as_ref()).unwrap())
            // Set the PEM encoded client side cert and key
            .cert(
                load_pem_file(config.cert.unwrap().as_ref()).unwrap(),
                load_pem_file(config.key.unwrap().as_ref()).unwrap(),
            )
            // Create channel credentials
            .build();
        ChannelBuilder::new(env).secure_connect(config.endpoint.as_str(), credentials)
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
