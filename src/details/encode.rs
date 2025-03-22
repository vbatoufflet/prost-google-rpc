use prost::{bytes::Bytes, Message, Name};
use prost_types::Any;
use tonic::{Code, Status};

use crate::proto;

#[derive(Debug)]
pub struct DetailsEncoder {
    details: Vec<Any>,
}

impl DetailsEncoder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            details: Vec::new(),
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.details.len()
    }

    pub fn push<M>(&mut self, message: &M)
    where
        M: Name,
    {
        self.details.push(Any {
            type_url: M::type_url(),
            value: message.encode_to_vec(),
        });
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

impl Default for DetailsEncoder {
    fn default() -> Self {
        Self::new()
    }
}
