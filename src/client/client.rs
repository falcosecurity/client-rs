use std::sync::Arc;

/// A result holding an [`Error`](Error).
// pub type Result<T> = result::Result<T, dyn Error>;

use crate::{
    config,
    // protos::{output, output_grpc},
};
// use futures::*;
use grpcio::{ChannelBuilder, ChannelCredentialsBuilder, Environment};
use std::{fs::File, io::Read, path::Path};

pub trait Connect {
    fn connect(&self, config: config::Config) -> Arc<grpcio::Channel>;
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
    fn connect(&self, config: config::Config) -> Arc<grpcio::Channel> {
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
        let ch = ChannelBuilder::new(self.env.clone())
            .secure_connect(config.endpoint.as_str(), credentials);
        Arc::new(ch)
    }
}
//TODO(fntlnz,leodido): keepalive, timeout, reconnect ?

#[derive(Clone)]
pub struct Client {
    // pub env: Arc<Environment>,
}

impl Client {
    pub fn new(config: config::Config) -> Result<Client, ()> {
        // let ca_crt = include_str!("/tmp/certs/ca.crt").into();
        // let client_crt = include_str!("/tmp/certs/client.crt").into();
        // let client_key = include_str!("/tmp/certs/client.key").into();

        Ok(Client{})
    }

    // pub fn outputs(&self, key: impl Into<Key>) -> impl Future<Output = Result<Option<Value>>> {
    //     // requests::new_raw_get_request(key, self.cf.clone()).execute(self.rpc.clone())
    // }
}
