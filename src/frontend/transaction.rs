use crate::backend::Selection;
use std::collections::HashMap;

pub struct Transaction {
    pub selections: HashMap<String, Selection>,
}
