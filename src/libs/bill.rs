pub mod bill_item;
pub mod bill_options;

use bill_item::BillItem;
use bill_options::BillOptions;
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

    pub fn take_action(&mut self, user_option: BillOptions) {
        match user_option {
            BillOptions::ADD => self.add(),
            _ => (),
        }
    }

    pub fn display_items(&self) {
        println!("Following are the items in your cart");
        println!("Name\tCost");
        for item in &self.items {
            println!("{}\t{}", item.name, item.amount);
        }
    }
}
