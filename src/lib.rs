pub use details::*;

pub mod proto {
    tonic::include_proto!("google.rpc");
}

mod details;
