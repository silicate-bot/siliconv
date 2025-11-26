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
    /// Create a new metadata field from an encoding function.
    pub fn new<F>(encode_fn: F) -> Self
    where
        F: Fn() -> String + 'a,
    {
        MetaField {
            encode_fn: Box::new(encode_fn),
        }
    }

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

// Implementations for common types

macro_rules! impl_meta_for_num {
    ($($t:ty),*) => {
        $(
            impl MetaEncodable for $t {
                fn encode(&self) -> String {
                    self.to_string()
                }
            }

            impl MetaDecodable for $t {
                fn decode(encoded: String) -> Option<Self> {
                    encoded.parse::<$t>().ok()
                }
            }
        )*
    };
}

impl_meta_for_num!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

impl MetaEncodable for String {
    fn encode(&self) -> String {
        self.clone()
    }
}

impl MetaDecodable for String {
    fn decode(encoded: String) -> Option<Self> {
        Some(encoded)
    }
}

impl MetaEncodable for () {
    fn encode(&self) -> String {
        String::new()
    }
}

impl MetaDecodable for () {
    fn decode(_encoded: String) -> Option<Self> {
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestMeta {
        pub tps: f64,
    }

    impl Meta for TestMeta {
        fn fields(&self) -> HashMap<String, MetaField<'_>> {
            let mut map = HashMap::new();
            let tps_field = MetaField {
                encode_fn: Box::new(|| self.tps.encode()),
            };
            map.insert("tps".to_string(), tps_field);
            map
        }

        fn from_fields(fields: HashMap<String, MetaField<'_>>) -> Self {
            let tps = fields
                .get("tps")
                .map_or(240.0, |t| t.decode::<f64>(t.encode()).unwrap_or(240.0));
            TestMeta { tps }
        }
    }

    struct OtherTestMeta {
        pub tps: f32,
        pub seed: u64,
    }

    impl Meta for OtherTestMeta {
        fn fields(&self) -> HashMap<String, MetaField<'_>> {
            let mut map = HashMap::new();
            let tps_field = MetaField {
                encode_fn: Box::new(|| (f64::from(self.tps)).encode()),
            };
            let seed_field = MetaField {
                encode_fn: Box::new(|| self.seed.to_string()),
            };
            map.insert("tps".to_string(), tps_field);
            map.insert("seed".to_string(), seed_field);
            map
        }

        fn from_fields(fields: HashMap<String, MetaField<'_>>) -> Self {
            #[allow(clippy::cast_possible_truncation)]
            let tps = fields.get("tps").map_or(240.0, |t| {
                t.decode::<f64>(t.encode()).unwrap_or(240.0) as f32
            });
            let seed = fields
                .get("seed")
                .map_or(2137, |t| t.decode::<u64>(t.encode()).unwrap_or(2137));
            OtherTestMeta { tps, seed }
        }
    }

    #[test]
    fn test_meta_field_encode_decode() {
        let value: f64 = 60.0;
        let meta_field = MetaField {
            encode_fn: Box::new(|| value.encode()),
        };

        let encoded = meta_field.encode();

        let decoded: f64 = meta_field.decode(encoded).unwrap();
        assert!(decoded.eq(&value));
    }

    #[test]
    fn test_same_meta() {
        let original_meta = TestMeta { tps: 60.0 };
        let fields = original_meta.fields();
        let reconstructed_meta = TestMeta::from_fields(fields);

        assert!(original_meta.tps.eq(&reconstructed_meta.tps));
    }

    #[test]
    fn test_different_meta() {
        let original_meta = TestMeta { tps: 60.0 };
        let other_meta = OtherTestMeta::from_fields(original_meta.fields());

        assert!(other_meta.tps.eq(&60.0));
        assert!(other_meta.seed.eq(&2137));
    }

    #[test]
    fn test_different_meta_to_original() {
        let original_meta = OtherTestMeta {
            tps: 75.0,
            seed: 1234,
        };

        let reconstructed_meta = TestMeta::from_fields(original_meta.fields());
        assert!(reconstructed_meta.tps.eq(&75.0));
    }
}
