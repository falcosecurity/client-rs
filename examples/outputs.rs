use falco::client::Client;
use falco::config::Config;
use falco::protos::output;
use futures::*;

fn main() {
    // gRPC configuration
    // with_auth is mandatory, Falco only supports mutual TLS connections.
    let config = Config::new("localhost:5060").with_auth(
        "/tmp/certs/ca.crt",
        "/tmp/certs/client.crt",
        "/tmp/certs/client.key",
    );

    // gRPC client
    let client = Client::new(config);

    // Output request
    // Keepalive true means that the client will wait indefinitely for new events to come
    // Use keepalive false if you only want to receive the accumulated events and stop
    let mut req = output::request::default();
    req.keepalive = true;

    // Subscribe to output events
    let mut res = client.outputs().subscribe(&req).unwrap();

    // Consume the events
    loop {
        let f = res.into_future();
        match f.wait() {
            Ok((Some(element), s)) => {
                res = s;
                println!("{:#?}", element);
            }
            // EOF
            Ok((None, _)) => break,
            // Error
            Err((e, _)) => panic!("error: {:?}", e),
        }
    }
}
