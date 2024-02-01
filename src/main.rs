use std::io::{self};
mod libs;

use libs::bill::bill_options::*;
use libs::bill::*;
use libs::utilities::*;

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
