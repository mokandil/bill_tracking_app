// Define the Bill structure and its methods
pub struct Bill {
    pub id: usize,
    pub description: String,
    pub amount: f64,
}

impl Bill {
    pub fn new(id: usize, description: String, amount: f64) -> Self {
        Self {
            id,
            description,
            amount,
        }
    }
}
