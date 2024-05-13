use serde::{Deserialize, Serialize};

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
