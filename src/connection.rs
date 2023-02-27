pub struct Disconnected;
pub struct Connected {
    url: String,
}

impl Disconnected {
    pub fn connect(url: &str) -> Connected {
        Connected {
            url: url.to_string(),
        }
    }
}

impl Connected {
    pub fn send_data(&self, data: &str) {
        // ...
    }

    pub fn disconnect(self) -> Disconnected {
        Disconnected
    }
}
