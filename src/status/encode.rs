use prost::{bytes::Bytes, Message};
use prost_types::Any;
use tonic::{Code, Status};

use crate::{proto, to_any, TypeURL};

#[derive(Debug)]
pub struct StatusEncoder {
    details: Vec<Any>,
}

impl StatusEncoder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            details: Vec::new(),
        }
    }

    #[must_use]
    pub fn details_len(&self) -> usize {
        self.details.len()
    }

    pub fn push_details<M>(&mut self, message: &M)
    where
        M: Message + TypeURL,
    {
        self.details.push(to_any(message));
    }

    #[must_use]
    pub fn status(self, code: Code, message: &str) -> Status {
        let status = proto::Status {
            code: code.into(),
            message: message.into(),
            details: self.details,
        };

        let mut buf = Vec::new();

        match status.encode(&mut buf) {
            Ok(()) => Status::with_details(code, message, Bytes::from(buf)),
            Err(_) => Status::internal("unhandled error"),
        }
    }
}

impl Default for StatusEncoder {
    fn default() -> Self {
        Self::new()
    }
}
