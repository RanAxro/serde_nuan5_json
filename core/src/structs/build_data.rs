use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BuildData{
  #[serde(rename = "NeedRestore")]
  pub need_restore: bool,

  #[serde(rename = "PlaceInfo")]
  pub place_info: Vec<PlaceInfo>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorMap{
  Map(HashMap<String, i32>),
  Vec(Vec<Option<i32>>),
}

#[derive(Serialize, Deserialize)]
pub struct PlaceInfo{
  #[serde(rename = "InstanceID")]
  pub instance_id: i32,

  #[serde(rename = "GroupID")]
  pub group_id: i32,

  #[serde(rename = "ItemID")]
  pub item_id: i32,

  #[serde(rename = "AttachID")]
  pub attach_id: i32,

  #[serde(rename = "ColorMap")]
  pub color_map: ColorMap,

  #[serde(rename = "TempInfo")]
  pub temp_info: TempInfo,

  #[serde(rename = "Location")]
  pub location: Location,

  #[serde(rename = "Rotation")]
  pub rotation: Rotation,

  #[serde(rename = "Scale")]
  pub scale: Scale,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TempInfo{
  #[serde(skip_serializing_if = "Option::is_none")]
  pub points: Option<Vec<Vec<f64>>>,
}

#[derive(Serialize, Deserialize)]
pub struct Location{
  #[serde(rename = "X")]
  pub x: f32,

  #[serde(rename = "Y")]
  pub y: f32,

  #[serde(rename = "Z")]
  pub z: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Rotation{
  #[serde(rename = "Pitch")]
  pub pitch: f32,

  #[serde(rename = "Yaw")]
  pub yaw: f32,

  #[serde(rename = "Roll")]
  pub roll: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Scale{
  #[serde(rename = "X")]
  pub x: f32,

  #[serde(rename = "Y")]
  pub y: f32,

  #[serde(rename = "Z")]
  pub z: f32,
}