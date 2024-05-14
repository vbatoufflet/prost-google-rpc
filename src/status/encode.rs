use prost::{bytes::Bytes, Message};
use prost_types::Any;
use tonic::{Code, Status};

use crate::{proto, to_any, TypeURL};

#[derive(Debug)]
pub struct StatusEncoder {
    details: Vec<Any>,
}

impl StatusEncoder {
    pub fn new() -> Self {
        Self {
            details: Vec::new(),
        }
    }

    pub fn details_len(&self) -> usize {
        self.details.len()
    }

    pub fn push_details<M>(&mut self, message: M)
    where
        M: Message + TypeURL,
    {
        self.details.push(to_any(message));
    }

    pub fn status(self, code: Code, message: &str) -> Status {
        let status = proto::Status {
            code: code.into(),
            message: message.into(),
            details: self.details,
        };

        let mut buf = Vec::new();

        match status.encode(&mut buf) {
            Ok(_) => Status::with_details(code, message, Bytes::from(buf)),
            Err(_) => Status::internal("unhandled error"),
        }
    }
}
