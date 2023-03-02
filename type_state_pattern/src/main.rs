use connection::{Connected, Disconnected};
use typed_builder_example::HttpRequest;

use crate::http_request_builder::{HttpMethod, HttpRequestBuilder};

mod connection;
mod http_request_builder;
mod typed_builder_example;

pub fn main() {
    connection();
    rest_call_builder();
    rest_call_builder_using_typed_builder_crate();
}

/// Simple example with two separate types
fn connection() {
    let device: Connected = Disconnected::connect("https://192.168.1.42");

    device.send_data("data 1"); // <- compiles because device is of type Connected
    device.send_data("data 2");

    let device: Disconnected = device.disconnect();

    // device.send_data("data 3"); // <- does not compile because device is of type Disconnected
}

/// Builder example with generic type state
///
/// "with_method", "with_server_name" and "with_port" must be called
/// exactly once. The order does not matter. Calling "with_headers"
/// is optional. The requirements are enforced at compile time.
fn rest_call_builder() {
    let result = HttpRequestBuilder::new()
        .with_method(HttpMethod::GET) // <- mandatory
        .with_server_name("my-server.com") // <- mandatory
        .with_port(8080) // <- mandatory
        .with_headers(vec![]) // <- optional
        .execute();

    println!("{}", result);
}

/// Builder example using the crate "typed-builder"
fn rest_call_builder_using_typed_builder_crate() {
    let request = HttpRequest::builder()
        .method(HttpMethod::GET) // <- mandatory
        .server_name("my-server.com".to_string()) // <- mandatory
        .port(8080) // <- mandatory
        .headers(vec![]) // <- optional
        .build();

    println!("{}", request.execute());
}
