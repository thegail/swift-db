use crate::backend::{OperationError, Response};
use crate::util::{LockType, RetainCount};
use std::sync::mpsc::Sender;

type ResponseSender = Sender<Result<Response, OperationError>>;

enum LockState {
    ReadRetained(RetainCount),
    WriteRetained(RetainCount),
    Blocked,
}

pub struct Lock {
    state: LockState,
    waiting: Vec<(ResponseSender, LockType)>,
}

impl Lock {
    pub fn new(lock: LockType) -> Self {
        let state = match lock {
            LockType::Read => LockState::ReadRetained(0),
            LockType::Write => LockState::WriteRetained(0),
            LockType::BlockingWrite => LockState::Blocked,
        };
        Self {
            state,
            waiting: Vec::new(),
        }
    }

    pub fn queue(&mut self, return_sender: ResponseSender, lock: LockType) {
        if self.evaluate_state(&lock) {
            return_sender.send(Ok(Response::Ok)).unwrap_or(());
        } else {
            self.waiting.push((return_sender, lock));
        }
    }

    pub fn release(&mut self, lock: &LockType) -> bool {
        match self.state {
            LockState::ReadRetained(0) => return self.get_next(true),
            LockState::ReadRetained(ref mut retain_count) => *retain_count -= 1,
            LockState::WriteRetained(ref mut retain_count) => match lock {
                LockType::Read => *retain_count -= 1,
                LockType::Write => self.state = LockState::ReadRetained(*retain_count),
                _ => unreachable!(),
            },
            LockState::Blocked => return self.get_next(true),
        }
        false
    }

    fn evaluate_state(&mut self, lock: &LockType) -> bool {
        // Blocking cases
        if let LockType::BlockingWrite = lock {
            return false;
        }
        if let LockState::Blocked = self.state {
            return false;
        }
        if let LockType::Write = lock {
            if let LockState::WriteRetained(_) = self.state {
                return false;
            }
        }
        // Nonblocking cases
        match lock {
            LockType::Read => match self.state {
                LockState::ReadRetained(ref mut retain_count) => *retain_count += 1,
                LockState::WriteRetained(ref mut retain_count) => *retain_count += 1,
                _ => unreachable!(),
            },
            LockType::Write => match self.state {
                LockState::ReadRetained(retain_count) => {
                    self.state = LockState::WriteRetained(retain_count)
                }
                _ => unreachable!(),
            },
            LockType::BlockingWrite => unreachable!(),
        }
        true
    }

    fn poll_next(&mut self) {
        if self.waiting.is_empty() {
            return;
        }
        let next = &self.waiting[0].1.clone();
        if self.evaluate_state(next) {
            self.waiting
                .remove(0)
                .0
                .send(Ok(Response::Ok))
                .unwrap_or(());
        }
    }

    fn get_next(&mut self, terminate_on_empty: bool) -> bool {
        if self.waiting.is_empty() {
            return terminate_on_empty;
        }
        let next = self.waiting.remove(0);
        next.0.send(Ok(Response::Ok)).unwrap_or(());
        self.state = match next.1 {
            LockType::Read => LockState::ReadRetained(1),
            LockType::Write => LockState::WriteRetained(0),
            LockType::BlockingWrite => LockState::Blocked,
        };
        self.poll_next();
        false
    }
}
