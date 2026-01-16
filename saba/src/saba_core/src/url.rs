use alloc::string::String;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

use alloc::string::ToString;

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }

    fn is_http(&self) -> bool {
        if self.url.contains("http://") {
            return true;
        }
        false
    }

    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only http:// URLs are supported".to_string());
        }
    }
}
