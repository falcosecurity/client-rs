use falco::client::{FalcoConnect, Connect};
use grpcio::{EnvBuilder};
use std::sync::Arc;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let connection = FalcoConnect {env: env};
    connection.connect();
}
