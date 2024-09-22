#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Url {
    pub(crate) scheme: Option<String>,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) host: Option<String>,
    pub(crate) port: Option<u16>,
    pub(crate) path: Option<String>,
    pub(crate) query: Option<String>,
    pub(crate) fragment: Option<String>,
}
