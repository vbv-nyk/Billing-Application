use std::io::{self};

struct Bill {
    items: Vec<BillItem>,
}

impl Bill {
    fn new() -> Self {
        Bill { items: vec![] }
    }

    fn add(&mut self) {
        let bill_item = BillItem::new();
        self.items.push(bill_item)
    }

    fn take_action(&mut self, user_option: BillOptions) {
        match user_option {
            BillOptions::ADD => self.add(),
            _ => (),
        }
    }

    fn display_items(&self) {
        println!("Following are the items in your cart");
        println!("Name\tCost");
        for item in &self.items {
            println!("{}\t{}", item.name, item.amount);
        }
    }
}

struct BillItem {
    name: String,
    amount: u32,
}

impl BillItem {
    fn new() -> BillItem {
        let item_name = get_item_name();
        let item_amount = get_item_amount();

        BillItem {
            name: item_name,
            amount: item_amount,
        }
    }
}

fn get_item_name() -> String {
    loop {
        let item_name = get_user_input("Enter Item Name");
        if let Ok(item_name) = item_name {
            return item_name;
        } else {
            println!("You haven't entered a valid string");
        }
    }
}

fn get_item_amount() -> u32 {
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

enum BillOptions {
    ADD,
    REMOVE,
    EDIT,
    EXIT,
    INVALID,
}

impl BillOptions {
    fn new(op: String) -> Self {
        match op.as_str() {
            "1" => BillOptions::ADD,
            "2" => BillOptions::REMOVE,
            "3" => BillOptions::EDIT,
            "4" => BillOptions::EXIT,
            _ => BillOptions::INVALID,
        }
    }
}

fn get_user_input(prompt: &str) -> Result<String, io::Error> {
    println!("{prompt}");
    let mut buffer = String::from("");
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn main() {
    let mut bill = Bill::new();
    'main: loop {
        println!("1. Add Bill\n2. Remove Bill\n3. Edit Bill\n4. Exit");
        let choice = get_user_input("Enter Your Choice");
        if let Ok(op) = choice {
            let user_option = BillOptions::new(op);
            match user_option {
                BillOptions::EXIT => {
                    println!("Thank you for shopping with us");
                    break 'main;
                }
                _ => bill.take_action(user_option),
            }
            bill.display_items();
        } else {
            println!("Error Parsing Input");
        }
    }
}
