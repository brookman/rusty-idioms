use typed_builder::TypedBuilder;

use crate::http_request_builder::HttpMethod;

#[derive(TypedBuilder)]
pub struct HttpRequest {
    // Mandatory fields:
    pub server_name: String,
    pub port: u16,
    pub method: HttpMethod,

    // Optional field with default value:
    #[builder(default=vec![])]
    pub headers: Vec<String>,
}

impl HttpRequest {
    pub fn execute(&self) -> String {
        format!("HTTP {:?} {}:{}", self.method, self.server_name, self.port).to_string()
    }
}
