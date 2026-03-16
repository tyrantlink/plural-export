use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct ProxyTag {
    pub prefix: Option<String>,
    pub suffix: Option<String>
}
