use std::borrow::Borrow;
use std::collections::HashMap;
use std::iter::Map;
use bevy::reflect::serde::Serializable::Owned;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

pub fn handle(text: String) -> String {
    // efficiency is so yesterday
    let mut json = json::parse(&*text).unwrap();
    let binding = json["Type"].take();
    let typ: &str = binding.as_str().unwrap();

    match typ {
        "login" => {
            return json!({
                "Type": "loginresponse",
                "Data": {
                    "balance": 0,
                    "success": false
                }
            }).to_string();
        }
        "newAI" => {
            //let data: Data_Rec_Newai = serde_json::from_str(&json.Data).unwrap();
        }
        "AIresponse" => {
            //let data: Data_Res_Airesponse = serde_json::from_str(&json.Data).unwrap();
        }
        "bettinginfo" => {
            //let data: Data_Rec_Betinfo = serde_json::from_str(&json.Data).unwrap();
        }
        "race" => {
            //let data: Data_Res_Race = serde_json::from_str(&json.Data).unwrap();
        }


        _ => {println!("HAAAAAAAAAAAAAAA")}
    }
    "".to_owned()
}