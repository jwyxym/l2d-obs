use serde_json::{from_str, to_string};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Error};

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Model {
	Version: i8,
    FileReferences: ModelFileReferences, 
    Groups: Vec<ModelGroup>
}

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ModelFileReferences {
	Moc: String,
	Textures: Vec<String>,
	Physics: String,
	DisplayInfo: String
}

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ModelGroup {
	Target: String,
	Name: String,
	Ids: Vec<String>
}

impl Model {
	pub fn from_json (str: &str) -> Result<Model, Error> {
		let mut model: Model = from_str::<Model>(str)?;
		let param_mouth_open_y: String = String::from("ParamMouthOpenY");
		let param_eye_l_open: String = String::from("ParamEyeLOpen");
		let param_eye_r_open: String = String::from("ParamEyeROpen");
		if let Some(lipsync) = model.Groups.iter_mut().find(|i|i.Name == String::from("LipSync")) {
			if !lipsync.Ids.contains(&param_mouth_open_y) {
				lipsync.Ids.push(param_mouth_open_y);
			}
		} else {
			model.Groups.push(ModelGroup {
				Target: String::from("Parameter"),
				Name: String::from("LipSync"),
				Ids: vec![param_mouth_open_y]
			});
		}
		if let Some(lipsync) = model.Groups.iter_mut().find(|i|i.Name == String::from("EyeBlink")) {
			if !lipsync.Ids.contains(&param_eye_l_open) {
				lipsync.Ids.push(param_eye_l_open);
			}
			if !lipsync.Ids.contains(&param_eye_r_open) {
				lipsync.Ids.push(param_eye_r_open);
			}
		} else {
			model.Groups.push(ModelGroup {
				Target: String::from("Parameter"),
				Name: String::from("EyeBlink"),
				Ids: vec![param_eye_l_open, param_eye_r_open]
			});
		}
		Ok(model)
	}
	pub fn to_string (&self) -> Result<String, Error> {
		let str: String = to_string(&self)?;
		Ok(str)
	}
}
