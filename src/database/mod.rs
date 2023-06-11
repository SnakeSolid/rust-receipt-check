mod data;

pub use self::data::CategoryData;
pub use self::data::ProductData;
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

    pub async fn select_category_name(
        &self,
        product: &str,
    ) -> Result<Option<CategoryData>, Box<dyn Error>> {
        info!("Category name: product = {}", product);

        let lock = self.inner.lock().await;
        let mut query = lock.prepare(
            "SELECT category, name
            FROM products
            WHERE product = :product AND category IS NOT NULL AND name IS NOT NULL",
        )?;
        query.bind((":product", product))?;

        let result = match query.next()? {
            State::Row => {
                let category = query.read(0)?;
                let name = query.read(1)?;

                Some(CategoryData::new(category, name))
            }
            State::Done => None,
        };

        Ok(result)
    }

    pub async fn select_category_names(&self) -> Result<Vec<ProductData>, Box<dyn Error>> {
        info!("Category names");

        let lock = self.inner.lock().await;
        let mut query = lock.prepare(
            "SELECT DISTINCT t.product, p.category, p.name
            FROM tickets AS t LEFT OUTER JOIN products AS p ON (p.product = t.product)
            ORDER BY t.product",
        )?;
        let mut result = Vec::new();

        while let State::Row = query.next()? {
            let product = query.read(0)?;
            let category = query.read(1)?;
            let name = query.read(2)?;
            let item = ProductData::new(product, category, name);

            result.push(item);
        }

        Ok(result)
    }

    pub async fn updatre_product_category(
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

    pub async fn remove_ticket_items(&self) -> Result<(), Box<dyn Error>> {
        info!("Remove ticket items");

        let lock = self.inner.lock().await;
        let mut query = lock.prepare("DELETE FROM tickets")?;
        query.next()?;

        Ok(())
    }

    pub async fn insert_ticket_item(
        &self,
        ticket: &str,
        date: &str,
        product: &str,
        quantity: f64,
        sum: f64,
    ) -> Result<(), Box<dyn Error>> {
        info!(
            "Insert ticket: ticket = {}, date = {}, product = {}, quantity = {}, sum = {}",
            ticket, date, product, quantity, sum
        );

        let lock = self.inner.lock().await;
        let mut query = lock.prepare(
            "INSERT INTO tickets (ticket, date, product, quantity, sum) VALUES (:ticket, :date, :product, :quantity, :sum)",
        )?;
        query.bind((":ticket", ticket))?;
        query.bind((":date", date))?;
        query.bind((":product", product))?;
        query.bind((":quantity", quantity))?;
        query.bind((":sum", sum))?;
        query.next()?;

        Ok(())
    }

    pub async fn select_ticket_items(&self) -> Result<Vec<TicketItemData>, Box<dyn Error>> {
        info!("Ticket items");

        let lock = self.inner.lock().await;
        let mut query = lock.prepare(
            "SELECT t.date, t.product, p.category, p.name, t.quantity, t.sum
        	FROM tickets AS t
        		LEFT OUTER JOIN products AS p ON (p.product = t.product)
        	ORDER BY t.date, t.product",
        )?;
        let mut result = Vec::new();

        while let State::Row = query.next()? {
            let date = query.read(0)?;
            let product = query.read(1)?;
            let category = query.read(2)?;
            let name = query.read(3)?;
            let quantity = query.read(4)?;
            let sum = query.read(5)?;
            let item = TicketItemData::new(date, product, category, name, quantity, sum);
            result.push(item);
        }

        Ok(result)
    }
}
