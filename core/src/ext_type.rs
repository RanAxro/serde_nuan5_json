use std::collections::{BTreeMap};
use std::ops::{Deref, DerefMut};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::DeserializeOwned;

pub const ID_MAP_TOKEN: &str = "IdMap";

#[derive(Serialize, Deserialize)]
pub struct IdMap<T>(BTreeMap<i64, T>);

impl<T> IdMap<T>{
  pub fn new() -> Self{
    IdMap(BTreeMap::new())
  }
}

impl<T> Deref for IdMap<T>{
  type Target = BTreeMap<i64, T>;
  fn deref(&self) -> &Self::Target{
    &self.0
  }
}

impl<T> DerefMut for IdMap<T>{
  fn deref_mut(&mut self) -> &mut Self::Target{
    &mut self.0
  }
}


#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdaptiveArray<T>{
  Array(Vec<T>),
  Item(T),
  Empty {},
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum OptionMap<T>{
  None {},
  Some(T),
}


pub enum OptionValue<T: Serialize + DeserializeOwned>{
  Value(T),
  None,
}

impl<T: Serialize + DeserializeOwned> Serialize for OptionValue<T>{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer
  {
    match self{
      OptionValue::Value(v) => v.serialize(serializer),
      OptionValue::None => "".serialize(serializer),
    }
  }
}

impl<'de, T: Serialize + DeserializeOwned> Deserialize<'de> for OptionValue<T>{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>
  {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum TemporaryOptionValue<T>{
      Value(T),
      None(String),
    }
    let value: TemporaryOptionValue<T> = de::Deserialize::deserialize(deserializer)?;
    Ok(match value{
      TemporaryOptionValue::Value(v) => {
        OptionValue::Value(v)
      },
      TemporaryOptionValue::None(_) => {
        OptionValue::None
      },
    })
  }
}