#[derive(Debug, Clone, PartialEq)]
pub enum Protocol {
    HTTP,
    HTTPS,
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::HTTP
    }
}

impl Protocol {
    pub fn new() -> Self {
        Protocol::default()
    }

    pub fn value(&self) -> &str {
        match self {
            Protocol::HTTP => "http",
            Protocol::HTTPS => "https",
        }
    }

    pub fn is_http(&self) -> bool {
        self.value() == Protocol::HTTP.value()
    }

    pub fn is_https(&self) -> bool {
        self.value() == Protocol::HTTPS.value()
    }
}
