use std::collections::{BTreeMap};
use std::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

pub const TOKEN: &str = "IdMap";

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