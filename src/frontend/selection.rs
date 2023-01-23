use crate::backend::Reference;
use crate::schema::Document;
use crate::util::LockType;

pub struct Selection {
    pub reference: Reference,
    pub lock: LockType,
    cached: Option<Document>,
    new_cached: Option<Document>,
}

impl Selection {
    pub fn new(reference: Reference, lock: LockType) -> Self {
        Self {
            reference,
            lock,
            cached: None,
            new_cached: None,
        }
    }

    pub fn cached(&self) -> Option<&Document> {
        self.new_cached.as_ref().or(self.cached.as_ref())
    }

    pub fn new_cached(&self) -> Option<Option<&Document>> {
        if self.cached.is_some() {
            if let Some(ref new) = self.new_cached {
                Some(Some(new))
            } else {
                None
            }
        } else {
            Some(None)
        }
    }

    pub fn cache(&mut self, document: Document) {
        self.new_cached = Some(document);
    }

    pub fn delete_cache(&mut self) {
        self.cached = None;
        self.new_cached = None;
    }
}
