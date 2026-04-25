mod utils;
pub use utils::deserialized::Deserialized;
pub use utils::errors::DeserializationError;

mod deserializer;

pub use deserializer::deserialize;