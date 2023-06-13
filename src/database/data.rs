#[derive(Debug)]
pub struct TicketItemData {
    date: String,
    product: String,
    category: Option<String>,
    name: Option<String>,
    quantity: f64,
    sum: f64,
}

impl TicketItemData {
    pub fn new(
        date: String,
        product: String,
        category: Option<String>,
        name: Option<String>,
        quantity: f64,
        sum: f64,
    ) -> Self {
        Self {
            date,
            product,
            category,
            name,
            quantity,
            sum,
        }
    }

    pub fn date(&self) -> &str {
        &self.date
    }

    pub fn product(&self) -> &str {
        &self.product
    }

    pub fn category(&self) -> Option<&String> {
        self.category.as_ref()
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn quantity(&self) -> f64 {
        self.quantity
    }

    pub fn sum(&self) -> f64 {
        self.sum
    }
}

#[derive(Debug)]
pub struct ProductData {
    product: String,
    category: Option<String>,
    name: Option<String>,
}

impl ProductData {
    pub fn new(product: String, category: Option<String>, name: Option<String>) -> Self {
        Self {
            product,
            category,
            name,
        }
    }

    pub fn product(&self) -> &str {
        &self.product
    }

    pub fn category(&self) -> Option<&String> {
        self.category.as_ref()
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}
