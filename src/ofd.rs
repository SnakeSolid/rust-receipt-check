use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use time::macros::format_description;
use time::PrimitiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
struct OfdTicketParams {
    #[serde(rename = "t")]
    time: String,
    #[serde(rename = "s")]
    sum: f32,
    #[serde(rename = "fn")]
    fiscal_storage: u64,
    #[serde(rename = "i")]
    index: u32,
    #[serde(rename = "fp")]
    fiscal_signature: u64,
    #[serde(rename = "n")]
    number: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct OfdTicketResponse {
    ticket: OfdTicket,
}

#[derive(Debug, Serialize, Deserialize)]
struct OfdTicket {
    #[serde(rename = "transactionDate")]
    transaction_date: String,
    items: Vec<OfdTicketItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OfdTicketItem {
    name: String,
    quantity: f32,
    sum: u64,
}

#[derive(Debug)]
pub struct Ticket {
    datetime: PrimitiveDateTime,
    items: Vec<TicketItem>,
}

impl Ticket {
    pub fn datetime(&self) -> &PrimitiveDateTime {
        &self.datetime
    }

    pub fn items(&self) -> &[TicketItem] {
        &self.items
    }
}

impl TryFrom<OfdTicketResponse> for Ticket {
    type Error = Box<dyn Error>;

    fn try_from(value: OfdTicketResponse) -> Result<Self, Self::Error> {
        Self::try_from(value.ticket)
    }
}

impl TryFrom<OfdTicket> for Ticket {
    type Error = Box<dyn Error>;

    fn try_from(value: OfdTicket) -> Result<Self, Self::Error> {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
        let mut items = HashMap::new();

        for item in value.items {
            let name = item.name.clone();
            let entry = items.entry(name.clone()).or_insert_with(|| TicketItem::new(&name));
            entry.quantity += item.quantity;
            entry.sum += 0.01 * item.sum as f32;
        }

        Ok(Ticket {
            datetime: PrimitiveDateTime::parse(&value.transaction_date, &format)?,
            items: items.into_iter().map(|(_key, value)| value).collect(),
        })
    }
}

#[derive(Debug)]
pub struct TicketItem {
    name: String,
    quantity: f32,
    sum: f32,
}

impl TicketItem {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            quantity: 0.0,
            sum: 0.0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn quantity(&self) -> f32 {
        self.quantity
    }

    pub fn sum(&self) -> f32 {
        self.sum
    }
}

impl From<OfdTicketItem> for TicketItem {
    fn from(value: OfdTicketItem) -> Self {
        TicketItem {
            name: value.name,
            quantity: value.quantity,
            sum: 0.01 * value.sum as f32,
        }
    }
}

pub async fn load_ticket(data: &str) -> Result<Ticket, Box<dyn Error>> {
    let params: OfdTicketParams = serde_qs::from_str(data)?;

    info!("OFD ticket params: {:?}", params);

    let uri = format!("https://consumer.1-ofd.ru/api/tickets/ticket/{}", data);
    let body = reqwest::get(uri).await?.text().await?;

    info!("ODF response body: {:?}", body);

    let response: OfdTicketResponse = serde_json::from_str(&body)?;

    info!("ODF response ticket: {:?}", response);

    Ok(response.try_into()?)
}
