use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply {
    success: bool,
    message: Option<String>,
}

impl Reply {
    pub fn success() -> Self {
        Reply {
            success: true,
            message: None,
        }
    }

    pub fn error(message: &str) -> Self {
        Reply {
            success: false,
            message: Some(message.into()),
        }
    }
}
