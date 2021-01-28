use json::{object::Object, JsonValue};
use std::{collections::HashMap, convert::TryInto};

use decent_serde_json_derive_alternative::derive_from_json_to_json_for_tuple;

pub use decent_serde_json_derive_alternative::{FromJson, ToJson};

pub trait FromJson: Sized {
    fn from_json(json: &JsonValue) -> Option<Self>;
}

pub trait ToJson {
    fn to_json(&self) -> JsonValue;
}

macro_rules! impl_to_json_numbers {
    ($x: ty) => {
        impl ToJson for $x {
            fn to_json(&self) -> JsonValue {
                JsonValue::Number((*self).into())
            }
        }
    };
}

impl_to_json_numbers!(u8);
impl_to_json_numbers!(u16);
impl_to_json_numbers!(u32);
impl_to_json_numbers!(u64);
impl_to_json_numbers!(usize);

impl_to_json_numbers!(i8);
impl_to_json_numbers!(i16);
impl_to_json_numbers!(i32);
impl_to_json_numbers!(i64);
impl_to_json_numbers!(isize);

impl_to_json_numbers!(f32);
impl_to_json_numbers!(f64);

impl ToJson for String {
    fn to_json(&self) -> JsonValue {
        JsonValue::String(self.clone())
    }
}

impl ToJson for bool {
    fn to_json(&self) -> JsonValue {
        JsonValue::Boolean(*self)
    }
}

impl<T> ToJson for Vec<T>
where
    T: ToJson,
{
    fn to_json(&self) -> JsonValue {
        JsonValue::Array(self.iter().map(|x| x.to_json()).collect())
    }
}

impl<V> ToJson for HashMap<String, V>
where
    V: ToJson,
{
    fn to_json(&self) -> JsonValue {
        let mut o = Object::new();
        for (key, value) in self {
            o.insert(key, value.to_json())
        }
        JsonValue::Object(o)
    }
}

impl<T> ToJson for Option<T>
where
    T: ToJson,
{
    fn to_json(&self) -> JsonValue {
        match self {
            Some(x) => x.to_json(),
            None => JsonValue::Null,
        }
    }
}
impl<T> ToJson for Box<T>
where
    T: ToJson,
{
    fn to_json(&self) -> JsonValue {
        self.as_ref().to_json()
    }
}

macro_rules! impl_from_json_numbers {
    ($x: ty) => {
        impl FromJson for $x {
            fn from_json(json: &JsonValue) -> Option<Self> {
                if let JsonValue::Number(x) = json {
                    (*x).try_into().ok()
                } else {
                    None
                }
            }
        }
    };
}

impl_from_json_numbers!(u8);
impl_from_json_numbers!(u16);
impl_from_json_numbers!(u32);
impl_from_json_numbers!(u64);
impl_from_json_numbers!(usize);

impl_from_json_numbers!(i8);
impl_from_json_numbers!(i16);
impl_from_json_numbers!(i32);
impl_from_json_numbers!(i64);
impl_from_json_numbers!(isize);

impl_from_json_numbers!(f32);
impl_from_json_numbers!(f64);

impl FromJson for String {
    fn from_json(json: &JsonValue) -> Option<Self> {
        if let JsonValue::String(x) = json {
            Some(x.clone())
        } else if let JsonValue::Short(x) = json {
            Some(x.to_string())
        } else {
            None
        }
    }
}

impl FromJson for bool {
    fn from_json(json: &JsonValue) -> Option<Self> {
        if let JsonValue::Boolean(x) = json {
            Some(*x)
        } else {
            None
        }
    }
}

impl<T> FromJson for Vec<T>
where
    T: FromJson,
{
    fn from_json(json: &JsonValue) -> Option<Self> {
        if let JsonValue::Array(xs) = json {
            let mut res = vec![];
            for x in xs.into_iter() {
                if let Some(x) = T::from_json(x) {
                    res.push(x);
                } else {
                    return None;
                }
            }
            Some(res)
        } else {
            None
        }
    }
}

impl<V> FromJson for HashMap<String, V>
where
    V: FromJson,
{
    fn from_json(json: &JsonValue) -> Option<Self> {
        if let JsonValue::Object(o) = json {
            let mut res = HashMap::new();
            for (key, value) in o.iter() {
                if let Some(value) = V::from_json(value) {
                    res.insert(key.to_string(), value);
                } else {
                    return None;
                }
            }
            Some(res)
        } else {
            None
        }
    }
}

impl<T> FromJson for Option<T>
where
    T: FromJson,
{
    fn from_json(json: &JsonValue) -> Option<Self> {
        if let JsonValue::Null = json {
            Some(None)
        } else if let Some(value) = T::from_json(json) {
            Some(Some(value))
        } else {
            None
        }
    }
}

impl<T> FromJson for Box<T>
where
    T: FromJson,
{
    fn from_json(json: &JsonValue) -> Option<Self> {
        T::from_json(json).map(Box::new)
    }
}

derive_from_json_to_json_for_tuple!(2);
derive_from_json_to_json_for_tuple!(3);
derive_from_json_to_json_for_tuple!(4);
derive_from_json_to_json_for_tuple!(5);
derive_from_json_to_json_for_tuple!(6);
derive_from_json_to_json_for_tuple!(7);
derive_from_json_to_json_for_tuple!(8);
derive_from_json_to_json_for_tuple!(9);
derive_from_json_to_json_for_tuple!(10);
