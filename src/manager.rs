use crate::bill::Bill;

pub struct BillManager {
    bills: Vec<Bill>,
    next_id: usize,
}

impl BillManager {
    pub fn new() -> Self {
        Self {
            bills: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_bill(&mut self, description: String, amount: f64) {
        let bill = Bill::new(self.next_id, description, amount);
        self.bills.push(bill);
        self.next_id += 1;
    }

    pub fn remove_bill(&mut self, id: usize) -> bool {
        if let Some(pos) = self.bills.iter().position(|x| x.id == id) {
            self.bills.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn edit_bill(
        &mut self,
        id: usize,
        new_description: Option<String>,
        new_amount: Option<f64>,
    ) -> bool {
        if let Some(bill) = self.bills.iter_mut().find(|x| x.id == id) {
            if let Some(desc) = new_description {
                bill.description = desc;
            }
            if let Some(amount) = new_amount {
                bill.amount = amount;
            }
            true
        } else {
            false
        }
    }

    pub fn view_bills(&self) {
        for bill in &self.bills {
            println!(
                "ID: {}, Description: {}, Amount: ${:.2}",
                bill.id, bill.description, bill.amount
            );
        }
    }
}
