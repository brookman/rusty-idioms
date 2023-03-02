use std::marker::PhantomData;

pub struct Missing;
pub struct Present;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct HttpRequestBuilder<State = (Missing, Missing, Missing, Missing)> {
    pub server_name: Option<String>,
    pub port: Option<u16>,
    pub method: Option<HttpMethod>,
    pub headers: Option<Vec<String>>,

    state: PhantomData<State>,
}

impl HttpRequestBuilder {
    pub fn new() -> HttpRequestBuilder<(Missing, Missing, Missing, Missing)> {
        HttpRequestBuilder {
            server_name: None,
            port: None,
            method: None,
            headers: None,

            state: PhantomData::<(Missing, Missing, Missing, Missing)>,
        }
    }
}

impl<T1, T2, T3> HttpRequestBuilder<(Missing, T1, T2, T3)> {
    pub fn with_server_name(self, url: &str) -> HttpRequestBuilder<(Present, T1, T2, T3)> {
        HttpRequestBuilder {
            server_name: Some(url.to_string()),
            port: self.port,
            method: self.method,
            headers: self.headers,

            state: PhantomData::<(Present, T1, T2, T3)>,
        }
    }
}

impl<T1, T2, T3> HttpRequestBuilder<(T1, Missing, T2, T3)> {
    pub fn with_port(self, port: u16) -> HttpRequestBuilder<(T1, Present, T2, T3)> {
        HttpRequestBuilder {
            server_name: self.server_name,
            port: Some(port),
            method: self.method,
            headers: self.headers,

            state: PhantomData::<(T1, Present, T2, T3)>,
        }
    }
}

impl<T1, T2, T3> HttpRequestBuilder<(T1, T2, Missing, T3)> {
    pub fn with_method(self, method: HttpMethod) -> HttpRequestBuilder<(T1, T2, Present, T3)> {
        HttpRequestBuilder {
            server_name: self.server_name,
            port: self.port,
            method: Some(method),
            headers: self.headers,

            state: PhantomData::<(T1, T2, Present, T3)>,
        }
    }
}

impl<T1, T2, T3> HttpRequestBuilder<(T1, T2, T3, Missing)> {
    pub fn with_headers(self, headers: Vec<String>) -> HttpRequestBuilder<(T1, T2, T3, Present)> {
        HttpRequestBuilder {
            server_name: self.server_name,
            port: self.port,
            method: self.method,
            headers: Some(headers),

            state: PhantomData::<(T1, T2, T3, Present)>,
        }
    }
}

impl<T1> HttpRequestBuilder<(Present, Present, Present, T1)> {
    pub fn execute(self) -> String {
        format!("HTTP {:?} {}:{}", self.method.unwrap(), self.server_name.unwrap(), self.port.unwrap()).to_string()
    }
}
