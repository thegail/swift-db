use super::transaction::Transaction;

pub struct Connection {
    pub transactions: HashMap<String, Transaction>,
}
