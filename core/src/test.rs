use serde::__private228::de::Content::String;
use serde::de::Unexpected::Str;
use crate::ext_type::*;
use crate::{de, ser};
use crate::structs::image_custom_data::*;
use crate::structs::share_code::*;

#[test]
fn test_1(){
  let mut t_m = IdMap::new();
  t_m.insert(27, false);
  t_m.insert(2, true);

  let r_t = NikkiPhotoCustomData{
    edit_photo_handler: Option::from(EditPhotoHandler{
      edit_state: true,
      has_sticker: false,
      has_text: true,
    }),
    // interactive_photo: None,
    interactive_photo: Option::from(t_m),
    photo_wall_plugin: None,
    portrait_mode_handler: None,
    puzzle_game_plugin: None,
    risk_photo: None,
    social_photo: Option::from(SocialPhoto{
      camera_params: "".to_string(),
      carrier_info: None,
      da_miao_info: OptionMap::None{},
      giant_state: None,
      interactions: AdaptiveArray::Empty{},
      local_transforms: "".to_string(),
      mount_info: None,
      photo_info: PhotoInfo{
        nikki_loc_x: 0.0,
        nikki_loc_y: 0.0,
        nikki_loc_z: 0.0,
        nikki_scale_x: 0.0,
        nikki_scale_y: 0.0,
        nikki_scale_z: 0.0,
        nikki_rot_yaw: 0.0,
        nikki_rot_pitch: 0.0,
        nikki_rot_roll: 0.0,
        camera_component_loc_x: 0.0,
        camera_component_loc_y: 0.0,
        camera_component_loc_z: 0.0,
        camera_component_rot_yaw: 0.0,
        camera_component_rot_pitch: 0.0,
        camera_component_rot_roll: 0.0,
        camera_actor_loc_x: 0.0,
        camera_actor_loc_y: 0.0,
        camera_actor_loc_z: 0.0,
        camera_actor_rot_yaw: 0.0,
        camera_actor_rot_pitch: 0.0,
        camera_actor_rot_roll: 0.0,
        aperture_section: 0,
        camera_focal_length: 0.0,
        filter_id: "".to_string(),
        filter_strength: 0.0,
        light_id: "".to_string(),
        light_strength: 0.0,
        magicball_color_ids: None,
        nikki_clothes: None,
        nikki_diy: AdaptiveArray::Empty{},
        nikki_hidden: false,
        nikki_weapon_tag_name: None,
        pose_id: 0,
        vignette_intensity: 0.0,
      },
      static_infos: OptionMap::None{},
      time: Time{
        day: 0,
        hour: 0,
        min: 0,
        sec: 0.0,
      },
      weapon_snap_shot: None,
      weather_type: 0,
    }),
  };
  let t = ser::to_string_pretty(&r_t).unwrap();
  print!("{}", t);
  let v = de::from_str::<NikkiPhotoCustomData>(&t).unwrap();
  match v.social_photo {
    Some(social_photo) => {
      println!("{}", social_photo.photo_info.camera_actor_loc_x);
      println!("{}", social_photo.camera_params);
    }
    _ => {}
  }
  match v.interactive_photo{
    Some(interactive_photo) => {
      println!("{:?}", interactive_photo.get(&27));
    }
    _ => {}
  }
  // println!("{}", v);
}



#[test]
fn test_2(){

  let t = "[{\"RoleID\":\"108328049\",\"TimeStamp\":1776261333.6336,\"ShareCode\":\"18otHv5nilc#\"},{\"RoleID\":\"108328049\",\"TimeStamp\":1776261317.6222,\"ShareCode\":\"18otfOV8T0o#\"},{\"RoleID\":\"108328049\",\"TimeStamp\":1776261259.1727,\"ShareCode\":\"18ot6FlYaao#\"},{\"RoleID\":\"108328049\",\"TimeStamp\":1772101678.5011,\"ShareCode\":\"1z0h9nkpKxV#\"},{\"RoleID\":\"108328049\",\"TimeStamp\":1772101618.9272,\"ShareCode\":\"1z0XlZCthe1#\"},{\"RoleID\":\"108328049\",\"TimeStamp\":1759042841.9844,\"ShareCode\":\"1KD94miaNxV#\"},{\"RoleID\":\"108328049\",\"TimeStamp\":1759042772.1247,\"ShareCode\":\"1KDTllkoJMZ#\"},{\"RoleID\":\"108328049\",\"TimeStamp\":1754713723.2667,\"ShareCode\":\"1ROS4VKliAH#\"},{\"RoleID\":\"108328049\",\"TimeStamp\":1776326096.8062,\"ShareCode\":\"186cGjd87IR#\"}]";

  let s: DiyHistoryShareCodeBox = de::from_str(&t).unwrap();
  let st = ser::to_string_pretty(&s).unwrap();

  println!("{}", st);
}