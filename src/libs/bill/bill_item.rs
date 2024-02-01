use super::super::utilities::*;

pub fn get_item_name() -> String {
    loop {
        let item_name = get_user_input("Enter Item Name");
        if let Ok(item_name) = item_name {
            return item_name;
        } else {
            println!("You haven't entered a valid string");
        }
    }
}

pub fn get_item_amount() -> u32 {
    loop {
        let amount = get_user_input("Enter Item Amount");
        if let Ok(amount) = amount {
            if let Ok(item_amount) = amount.trim().parse::<u32>() {
                return item_amount;
            } else {
                println!("Entered amount is not an integer");
            }
        }
    }
}

pub struct BillItem {
    pub name: String,
    pub amount: u32,
}

impl BillItem {
    pub fn new() -> BillItem {
        let item_name = get_item_name();
        let item_amount = get_item_amount();

        BillItem {
            name: item_name,
            amount: item_amount,
        }
    }
}
