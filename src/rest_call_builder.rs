use std::marker::PhantomData;

pub struct Missing;
pub struct Present;

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct RestCallBuilder<State = (Missing, Missing, Missing, Missing)> {
    pub url: Option<String>,
    pub port: Option<u16>,
    pub method: Option<HttpMethod>,
    pub headers: Option<Vec<String>>,

    state: PhantomData<State>,
}

impl RestCallBuilder {
    pub fn new() -> RestCallBuilder<(Missing, Missing, Missing, Missing)> {
        RestCallBuilder {
            url: None,
            port: None,
            method: None,
            headers: None,

            state: PhantomData::<(Missing, Missing, Missing, Missing)>,
        }
    }
}

impl<T1, T2, T3> RestCallBuilder<(Missing, T1, T2, T3)> {
    pub fn with_url(self, url: &str) -> RestCallBuilder<(Present, T1, T2, T3)> {
        RestCallBuilder {
            url: Some(url.to_string()),
            port: self.port,
            method: self.method,
            headers: self.headers,

            state: PhantomData::<(Present, T1, T2, T3)>,
        }
    }
}

impl<T1, T2, T3> RestCallBuilder<(T1, Missing, T2, T3)> {
    pub fn with_port(self, port: u16) -> RestCallBuilder<(T1, Present, T2, T3)> {
        RestCallBuilder {
            url: self.url,
            port: Some(port),
            method: self.method,
            headers: self.headers,

            state: PhantomData::<(T1, Present, T2, T3)>,
        }
    }
}

impl<T1, T2, T3> RestCallBuilder<(T1, T2, Missing, T3)> {
    pub fn with_method(self, method: HttpMethod) -> RestCallBuilder<(T1, T2, Present, T3)> {
        RestCallBuilder {
            url: self.url,
            port: self.port,
            method: Some(method),
            headers: self.headers,

            state: PhantomData::<(T1, T2, Present, T3)>,
        }
    }
}

impl<T1, T2, T3> RestCallBuilder<(T1, T2, T3, Missing)> {
    pub fn with_headers(self, headers: Vec<String>) -> RestCallBuilder<(T1, T2, T3, Present)> {
        RestCallBuilder {
            url: self.url,
            port: self.port,
            method: self.method,
            headers: Some(headers),

            state: PhantomData::<(T1, T2, T3, Present)>,
        }
    }
}

impl<T1> RestCallBuilder<(Present, Present, Present, T1)> {
    pub fn execute(self) -> String {
        "some http result".to_string()
    }
}
