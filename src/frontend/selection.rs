use crate::backend::Reference;
use crate::language::LockType;

pub struct Selection {
    pub reference: Reference,
    pub lock: LockType,
}
