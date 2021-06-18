use falco::api::outputs;
use falco::client::Client;
use falco::config::Config;
use futures::{Async, Future, Sink, Stream};
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time;

fn main() {
    // gRPC configuration
    // with_auth is mandatory, Falco only supports mutual TLS connections.
    let config = Config::new("localhost:5060").with_auth(
        "/tmp/certs/ca.crt",
        "/tmp/certs/client.crt",
        "/tmp/certs/client.key",
    );

    // gRPC client
    let client = Client::new(config).unwrap();

    // Output request
    // Keepalive true means that the client will wait indefinitely for new events to come
    // Use keepalive false if you only want to receive the accumulated events and stop
    let req = outputs::request::default();
    let wrflags = grpcio::WriteFlags::default();
    //req.keepalive = true;

    // Subscribe to output events
    let (mut sink, mut receiver) = client.outputs().sub().unwrap();

    let sink_lock = Arc::new(Mutex::new(sink.wait()));

    let sink_lock_copy = Arc::clone(&sink_lock);

    // Spawn a thread to request new data every 100ms
    spawn(move || {
        let req = outputs::request::default();
        let time_to_sleep = time::Duration::from_millis(100);
        loop {
            sleep(time_to_sleep);
            let mut sink = sink_lock_copy.lock().unwrap();
            sink.send((req.clone(), wrflags)).expect("Failed to send");
        }
    });

    // Consume the events
    loop {
        {
            let mut sink = sink_lock.lock().unwrap();
            sink.send((req.clone(), wrflags)).expect("Failed to send");
        }
        let f = receiver.into_future();
        match f.wait() {
            Ok((Some(element), s)) => {
                receiver = s;
                println!("{:#?}", element);
            }
            // EOF
            Ok((None, _)) => break,
            // Error
            Err((e, _)) => panic!("error: {:?}", e),
        }
    }
}
