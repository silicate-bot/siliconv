//! Metadata handling for different replay formats.

use std::collections::HashMap;

/// A metadata field.
pub struct MetaField<'a> {
    /// Function to encode the field into a string.
    encode_fn: Box<dyn Fn() -> String + 'a>,
}

/// Trait for types that can be encoded into metadata fields.
pub trait MetaEncodable {
    /// Encode the type into a string.
    fn encode(&self) -> String;
}

/// Trait for types that can be decoded from metadata fields.
pub trait MetaDecodable {
    /// Decode the type from a string.
    fn decode(encoded: String) -> Option<Self>
    where
        Self: Sized;
}

impl<'a> MetaField<'a> {
    #[must_use]
    /// Create a new metadata field from an encodable value.
    pub fn encode(&'a self) -> String {
        (self.encode_fn)()
    }

    #[must_use]
    /// Decode a metadata field into a decodable type.
    pub fn decode<T: MetaDecodable>(&self, encoded: String) -> Option<T> {
        T::decode(encoded)
    }
}

/// Generic metadata trait for different replays.
pub trait Meta {
    /// Fetch all fields in a metadata object.
    fn fields(&self) -> HashMap<String, MetaField<'_>>
    where
        Self: Sized;

    /// Create a metadata object from fields.
    fn from_fields(fields: HashMap<String, MetaField<'_>>) -> Self
    where
        Self: Sized;
}
