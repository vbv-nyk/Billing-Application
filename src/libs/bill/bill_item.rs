use super::super::utilities::*;

pub struct BillItem {
    pub name: String,
    pub amount: u32,
}

impl BillItem {
    pub fn new() -> BillItem {
        let item_name = get_user_string_input("Enter Item Name");
        let item_amount = get_user_number_input("Enter Item Amount");

        BillItem {
            name: item_name,
            amount: item_amount,
        }
    }
}
