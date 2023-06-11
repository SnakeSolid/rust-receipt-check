use crate::database::Database;
use crate::database::TicketItemData;
use serde::Deserialize;
use serde::Serialize;
use std::convert::Infallible;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ReplyItem {
    Categorized {
        ticket: String,
        category: String,
        name: String,
        quantity: f64,
        sum: f64,
    },
    Uncategorized {
        ticket: String,
        product: String,
        quantity: f64,
        sum: f64,
    },
}

impl From<TicketItemData> for ReplyItem {
    fn from(value: TicketItemData) -> Self {
        match (value.category(), value.name()) {
            (Some(category), Some(name)) => ReplyItem::Categorized {
                ticket: value.ticket().into(),
                category: category.clone(),
                name: name.clone(),
                quantity: value.quantity(),
                sum: value.sum(),
            },
            _ => ReplyItem::Uncategorized {
                ticket: value.ticket().into(),
                product: value.product().into(),
                quantity: value.quantity(),
                sum: value.sum(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Reply {
    success: bool,
    items: Option<Vec<ReplyItem>>,
    message: Option<String>,
}

impl Reply {
    pub fn success(items: Vec<TicketItemData>) -> Self {
        let items = items.into_iter().map(ReplyItem::from).collect();

        Reply {
            success: true,
            items: Some(items),
            message: None,
        }
    }

    pub fn error(message: &str) -> Self {
        Reply {
            success: false,
            items: None,
            message: Some(message.into()),
        }
    }
}

macro_rules! no_fail {
    ($message:expr, $callback:expr) => {
        match $callback {
            Ok(result) => result,
            Err(error) => {
                warn!("{}: {}", $message, error);

                let message = format!("{}", error);

                return Ok(warp::reply::json(&Reply::error(&message)));
            }
        }
    };
}

pub async fn tickets(database: Database) -> Result<impl warp::Reply, Infallible> {
    info!("Request tickets");

    let items = no_fail!("Failed to read items", database.ticket_items().await);

    Ok(warp::reply::json(&Reply::success(items)))
}
