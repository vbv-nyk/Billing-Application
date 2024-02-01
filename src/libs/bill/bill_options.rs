#[derive(Debug)]
pub enum BillOptions {
    ADD,
    REMOVE,
    EDIT,
    EXIT,
    INVALID,
}

impl BillOptions {
    pub fn new(op: String) -> Self {
        match op.as_str() {
            "1" => BillOptions::ADD,
            "2" => BillOptions::REMOVE,
            "3" => BillOptions::EDIT,
            "4" => BillOptions::EXIT,
            _ => BillOptions::INVALID,
        }
    }
}
