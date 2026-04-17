use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdMap<T>{
  inner: BTreeMap<i64, T>,
}

impl<T> Deref for IdMap<T>{
  type Target = BTreeMap<i64, T>;

  fn deref(&self) -> &Self::Target{
    &self.inner
  }
}

impl<T> DerefMut for IdMap<T>{
  fn deref_mut(&mut self) -> &mut Self::Target{
    &mut self.inner
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