use crate::database::TicketItemData;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReplyItem {
    Categorized {
        date: String,
        category: String,
        name: String,
        quantity: f64,
        sum: f64,
    },
    Uncategorized {
        date: String,
        product: String,
        quantity: f64,
        sum: f64,
    },
}

impl From<TicketItemData> for ReplyItem {
    fn from(value: TicketItemData) -> Self {
        match (value.category(), value.name()) {
            (Some(category), Some(name)) => ReplyItem::Categorized {
                date: value.date().into(),
                category: category.clone(),
                name: name.clone(),
                quantity: value.quantity(),
                sum: value.sum(),
            },
            _ => ReplyItem::Uncategorized {
                date: value.date().into(),
                product: value.product().into(),
                quantity: value.quantity(),
                sum: value.sum(),
            },
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
    pub fn list(items: Vec<TicketItemData>) -> Self {
        let items = items.into_iter().map(ReplyItem::from).collect();

        Reply::List {
            success: true,
            items,
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
