use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DuoMotionPlayType{
  Default = 0,
  AsInviter = 1,
  AsInvitee = 2,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PhotoMode{
  Normal = 0,
  TimePause = 1,
  MultiPlayer = 2,
  GameScreenshot = 3,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum WindIntensity{
  Invalid = -1,
  None = 0,
  Small = 1,
  Middle = 2,
  Big = 3,
  VeryBig = 4
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum WindDirection{
  Invalid = -1,
  East = 0,
  South = 1,
  West = 2,
  North = 3,
  EastSouth = 4,
  WestSouth = 5,
  EastNorth = 6,
  WestNorth = 7,
  None = 8
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum WeatherType{
  Invalid = -1,
  Sunny = 0,
  Cloudy = 1,
  Rainy = 2,
  Thunderstorm = 3,
  Rainbow = 4,
  WaterspoutCloudy = 5,
  WaterspoutRainy = 6,
  StarSea = 7
}