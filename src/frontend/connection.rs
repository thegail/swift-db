use super::transaction::Transaction;
use std::collections::HashMap;

pub struct Connection {
    pub transactions: HashMap<String, Transaction>,
}
