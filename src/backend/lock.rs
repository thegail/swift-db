use crate::backend::{OperationError, Response};
use std::sync::mpsc::Sender;

type ResponseSender = Sender<Result<Response, OperationError>>;

pub struct Lock {
    retain_count: Option<u32>,
    waiting: Vec<(ResponseSender, bool)>,
}

impl Lock {
    pub fn new(blocking: bool) -> Self {
        let retain_count = if blocking { None } else { Some(0) };
        Self {
            retain_count,
            waiting: Vec::new(),
        }
    }

    pub fn queue(&mut self, return_sender: ResponseSender, blocking: bool) {
        if blocking {
            self.waiting.push((return_sender, true));
        } else {
            if let Some(ref mut retain_count) = self.retain_count {
                *retain_count += 1;
                return_sender.send(Ok(Response::Ok)).unwrap_or(());
            } else {
                self.waiting.push((return_sender, false));
            }
        }
    }

    pub fn release(&mut self) -> bool {
        if let Some(ref mut retain_count) = self.retain_count {
            *retain_count -= 1;
            if *retain_count == 0 {
                self.retain_count = None;
                self.get_next(true)
            } else {
                false
            }
        } else {
            self.get_next(true)
        }
    }

    fn get_next(&mut self, terminate_on_empty: bool) -> bool {
        if self.waiting.is_empty() {
            return terminate_on_empty;
        }
        let next = self.waiting.remove(0);
        next.0.send(Ok(Response::Ok)).unwrap_or(());
        if !next.1 {
            self.retain_count = Some(1);
            self.get_next(false);
        }
        false
    }
}
