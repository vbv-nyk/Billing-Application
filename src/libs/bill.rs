pub mod bill_item;
pub mod bill_options;

use bill_item::BillItem;
use bill_options::BillOptions;

use super::utilities::{convert_to_usize, get_user_number_input};
pub struct Bill {
    items: Vec<BillItem>,
}

impl Bill {
    pub fn new() -> Self {
        Bill { items: vec![] }
    }

    pub fn add(&mut self) {
        let bill_item = BillItem::new();
        self.items.push(bill_item)
    }

    pub fn remove(&mut self) {
        let position = get_user_number_input("Enter the item number to remove");
        let position = convert_to_usize(position);

        if position < self.items.len() {
            self.items.remove(position);
        }
    }

    pub fn take_action(&mut self, user_option: BillOptions) {
        match user_option {
            BillOptions::ADD => self.add(),
            BillOptions::REMOVE => self.remove(),
            _ => println!("{:#?}", user_option),
        }
    }

    pub fn display_items(&self) {
        println!("\nFollowing are the items in your cart");
        println!("Name\tCost");
        for item in &self.items {
            println!("{}\t{}\n", item.name, item.amount);
        }
    }
}
