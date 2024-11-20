use prost::Message;
use prost_types::Any;

pub trait TypeURL {
    fn type_url() -> &'static str;
}

pub fn to_any<M>(message: &M) -> Any
where
    M: Message + TypeURL,
{
    Any {
        type_url: M::type_url().to_owned(),
        value: message.encode_to_vec(),
    }
}
