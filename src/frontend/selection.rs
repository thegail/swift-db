use crate::backend::Reference;
use crate::util::LockType;

pub struct Selection {
    pub reference: Reference,
    pub lock: LockType,
}
