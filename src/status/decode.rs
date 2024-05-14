use prost::{DecodeError, Message};
use prost_types::Any;
use tonic::{Code, Status};

use crate::proto;

#[derive(Debug)]
pub struct StatusDecoder {
    status: proto::Status,
}

impl StatusDecoder {
    pub fn from_status(status: Status) -> Result<Self, DecodeError> {
        let status = proto::Status::decode(status.details())?;
        Ok(Self { status })
    }

    pub fn code(&self) -> Code {
        Code::from_i32(self.status.code)
    }

    pub fn message(&self) -> &str {
        &self.status.message
    }

    pub fn details(&self) -> &[Any] {
        &self.status.details
    }
}
