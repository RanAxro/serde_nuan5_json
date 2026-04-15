use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

pub type IdMap<T> = BTreeMap<i64, T>;

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