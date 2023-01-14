use crate::backend::Reference;
use crate::schema::Document;
use crate::util::LockType;

pub struct Selection {
    pub reference: Reference,
    pub lock: LockType,
    pub cached: Option<Document>,
}

impl Selection {
    pub fn new(reference: Reference, lock: LockType) -> Self {
        Self {
            reference,
            lock,
            cached: None,
        }
    }
}
