use serde::{Deserialize, Deserializer, Serialize};

pub fn deserialize_float_as_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringFloatVisitor;

    impl<'de> serde::de::Visitor<'de> for StringFloatVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a floating number")
        }

        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_string())
        }

        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_string())
        }

        fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_string())
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_string())
        }
    }

    deserializer.deserialize_any(StringFloatVisitor)
}

// Quickly implment GetOwned trait
macro_rules! value_as_type {
    ($type_name: ident, $value_type: ty, $value: expr) => {
        pub struct $type_name;

        impl crate::utils::GetOwned<$value_type> for $type_name {
            fn get_owned() -> $value_type {
                $value
            }
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UntaggeBinary<First, Second> {
    Primary(First),
    Secondary(Second),
}

pub(crate) use value_as_type;

pub trait GetRef<'a, Item> {
    fn get_ref() -> &'a Item;
}

pub trait GetOwned<Item> {
    fn get_owned() -> Item;
}
