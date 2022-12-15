#[derive(Clone)]
pub enum LockType {
    Read,
    Write,
    BlockingWrite,
}
