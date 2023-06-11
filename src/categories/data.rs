use crate::database::ProductData;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyItem {
    product: String,
    category: String,
    name: String,
}

impl From<ProductData> for ReplyItem {
    fn from(value: ProductData) -> Self {
        Self {
            product: value.product().into(),
            category: value.category().cloned().unwrap_or_default(),
            name: value.name().cloned().unwrap_or_default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Reply {
    List {
        success: bool,
        items: Vec<ReplyItem>,
    },
    Success {
        success: bool,
    },
    Error {
        success: bool,
        message: String,
    },
}

impl Reply {
    pub fn list(items: Vec<ProductData>) -> Self {
        let items = items.into_iter().map(ReplyItem::from).collect();

        Reply::List {
            success: true,
            items: items,
        }
    }

    pub fn success() -> Self {
        Reply::Success { success: true }
    }

    pub fn error(message: &str) -> Self {
        Reply::Error {
            success: false,
            message: message.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateParams {
    product: String,
    category: String,
    name: String,
}

impl UpdateParams {
    pub fn product(&self) -> &str {
        &self.product
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
