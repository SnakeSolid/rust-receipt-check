mod data;

pub use self::data::CategoryNameData;
pub use self::data::TicketItemData;

use sqlite::Connection;
use sqlite::State;
use std::error::Error;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Database {
    inner: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new<P>(path: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let connection = sqlite::open(path)?;
        connection.execute(include_str!("create_database.sql"))?;

        Ok(Self {
            inner: Arc::new(Mutex::new(connection)),
        })
    }

    pub async fn category_name(
        &self,
        product: &str,
    ) -> Result<Option<CategoryNameData>, Box<dyn Error>> {
        info!("Category name: product = {}", product);

        let lock = self.inner.lock().await;
        let mut query =
            lock.prepare("SELECT category, name FROM products WHERE product = :product AND category IS NOT NULL AND name IS NOT NULL")?;
        query.bind((":product", product))?;

        let result = match query.next()? {
            State::Row => {
                let category = query.read(0)?;
                let name = query.read(1)?;

                Some(CategoryNameData::new(category, name))
            }
            State::Done => None,
        };

        Ok(result)
    }

    pub async fn next_uncatigorized_product(
        &self,
        ticket: &str,
    ) -> Result<Option<String>, Box<dyn Error>> {
        info!("Count ticket: {}", ticket);

        let lock = self.inner.lock().await;
        let mut query = lock.prepare("SELECT t.product FROM tickets AS t LEFT OUTER JOIN products AS p ON ( p.product = t.product ) WHERE p.product IS NULL LIMIT 1")?;

        let result = match query.next()? {
            State::Row => Some(query.read(0)?),
            State::Done => None,
        };

        Ok(result)
    }

    pub async fn set_category_name(
        &self,
        product: &str,
        category: &str,
        name: &str,
    ) -> Result<(), Box<dyn Error>> {
        info!(
            "Set category name: product = {}, category = {}, name = {}",
            product, category, name
        );

        let lock = self.inner.lock().await;
        let mut query =
            lock.prepare("INSERT OR REPLACE INTO products (category, name, product) VALUES (:category, :name, :product)")?;
        query.bind((":product", product))?;
        query.bind((":category", category))?;
        query.bind((":name", name))?;
        query.next()?;

        Ok(())
    }

    pub async fn ticket_item_count(&self, ticket: &str) -> Result<usize, Box<dyn Error>> {
        info!("Count ticket: {}", ticket);

        let lock = self.inner.lock().await;
        let mut query = lock.prepare("SELECT COUNT(*) FROM tickets WHERE ticket = :ticket")?;
        query.bind((":ticket", ticket))?;

        let result = match query.next()? {
            State::Row => query.read(0)?,
            State::Done => 0,
        };

        Ok(result as usize)
    }

    pub async fn remove_ticket(&self, ticket: &str) -> Result<(), Box<dyn Error>> {
        info!("Remove ticket: {}", ticket);

        let lock = self.inner.lock().await;
        let mut query = lock.prepare("DELETE FROM tickets WHERE ticket = :ticket")?;
        query.bind((":ticket", ticket))?;
        query.next()?;

        Ok(())
    }

    pub async fn insert_ticket_item(
        &self,
        ticket: &str,
        product: &str,
        quantity: f64,
        sum: f64,
    ) -> Result<(), Box<dyn Error>> {
        info!(
            "Insert ticket: ticket = {}, product = {}, quantity = {}, sum = {}",
            ticket, product, quantity, sum
        );

        let lock = self.inner.lock().await;
        let mut query = lock.prepare(
            "INSERT INTO tickets (ticket, product, quantity, sum) VALUES (:ticket, :product, :quantity, :sum)",
        )?;
        query.bind((":ticket", ticket))?;
        query.bind((":product", product))?;
        query.bind((":quantity", quantity))?;
        query.bind((":sum", sum))?;
        query.next()?;

        Ok(())
    }

    pub async fn ticket_items(&self) -> Result<Vec<TicketItemData>, Box<dyn Error>> {
        info!("Ticket items");

        let lock = self.inner.lock().await;
        let mut query = lock.prepare(
            "SELECT t.ticket, t.product, p.category, p.name, t.quantity, t.sum
        	FROM tickets AS t
        		LEFT OUTER JOIN products AS p ON (p.product = t.product)
        	ORDER BY t.ticket, t.product",
        )?;
        let mut result = Vec::new();

        while let State::Row = query.next()? {
            let ticket = query.read(0)?;
            let product = query.read(1)?;
            let category = query.read(2)?;
            let name = query.read(3)?;
            let quantity = query.read(4)?;
            let sum = query.read(5)?;
            let item = TicketItemData::new(ticket, product, category, name, quantity, sum);
            result.push(item);
        }

        Ok(result)
    }
}
