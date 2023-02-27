use crate::{
    connection::Disconnected,
    rest_call_builder::{HttpMethod, RestCallBuilder},
};

mod connection;
mod rest_call_builder;

fn main() {
    connection();
    rest_call_builder();
}

fn connection() {
    let device = Disconnected::connect("https://192.168.1.42");

    device.send_data("data 1");
    device.send_data("data 2");

    let device = device.disconnect();

    // device.send_data("data 3");
}

fn rest_call_builder() {
    let result = RestCallBuilder::new()
        .with_method(HttpMethod::GET)
        .with_url("https://my-url.com/api")
        .with_port(8080)
        // .with_headers(vec![])
        .execute();

    println!("{}", result);
}
