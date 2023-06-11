#[derive(Debug)]
pub struct TicketItemData {
    ticket: String,
    product: String,
    category: Option<String>,
    name: Option<String>,
    quantity: f64,
    sum: f64,
}

impl TicketItemData {
    pub fn new(
        ticket: String,
        product: String,
        category: Option<String>,
        name: Option<String>,
        quantity: f64,
        sum: f64,
    ) -> Self {
        Self {
            ticket,
            product,
            category,
            name,
            quantity,
            sum,
        }
    }

    pub fn ticket(&self) -> &str {
        &self.ticket
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
pub struct CategoryNameData {
    category: String,
    name: String,
}

impl CategoryNameData {
    pub fn new(category: String, name: String) -> Self {
        Self {
            category: category.into(),
            name: name.into(),
        }
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
