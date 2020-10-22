//! Serialize/deserialize Option<T> type.
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Deserialize Option<T>
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

/// Serialize Option<T>
pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Default + PartialEq + Serialize,
{
    if value == &T::default() {
        // Todo: Cleanup if works.
        //let whatevs: Option<T> = None;
        return serializer.serialize_none();
        //return whatevs.serialize(serializer);
    }
    value.serialize(serializer)
}
