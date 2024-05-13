use crate::bill::Bill;
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

const FILE_PATH: &str = "bills.json";

pub struct BillManager {
    bills: Vec<Bill>,
    next_id: usize,
}

impl BillManager {
    pub fn new() -> Self {
        let mut manager = Self {
            bills: Vec::new(),
            next_id: 1,
        };
        manager.load_bills();
        manager
    }

    pub fn add_bill(&mut self, description: String, amount: f64) {
        let bill = Bill::new(self.next_id, description, amount);
        self.bills.push(bill);
        self.next_id += 1;
        self.save_bills();
    }

    pub fn remove_bill(&mut self, id: usize) -> bool {
        if let Some(pos) = self.bills.iter().position(|x| x.id == id) {
            self.bills.remove(pos);
            self.save_bills();
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
        let found = self.bills.iter_mut().find(|x| x.id == id);
        if let Some(bill) = found {
            if let Some(desc) = new_description {
                bill.description = desc;
            }
            if let Some(amount) = new_amount {
                bill.amount = amount;
            }
            self.save_bills();
            true
        } else {
            false
        }
    }

    fn save_bills(&self) {
        let file_path = FILE_PATH;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .unwrap();
        let data = serde_json::to_string(&self.bills).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    fn load_bills(&mut self) {
        let file_path = FILE_PATH;
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => return, // No file to load from, start with an empty list
        };

        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        self.bills = serde_json::from_str(&data).unwrap_or_else(|_| Vec::new());
        if let Some(max) = self.bills.iter().max_by_key(|bill| bill.id) {
            self.next_id = max.id + 1;
        }
    }

    pub fn view_bills(&self) {
        if self.bills.is_empty() {
            println!("No bills found.");
        } else {
            for bill in &self.bills {
                println!(
                    "ID: {}, Description: {}, Amount: {}",
                    bill.id, bill.description, bill.amount
                );
            }
        }
        // show total amount of bills
        let total: f64 = self.bills.iter().map(|x| x.amount).sum();
        println!("\nTotal amount: {}", total);
    }
}
