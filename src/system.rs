use std::borrow::Borrow;
use std::collections::HashMap;
use std::iter::Map;
use bevy::reflect::serde::Serializable::Owned;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use crate::betting_maths::{add_bet, add_horse, name_horses};
use crate::database::{balance_get, get_player};

pub async fn handle(text: String) -> Option<String> {
    // efficiency is so yesterday
    let mut json = json::parse(&*text).unwrap();
    if json["Type"].is_null() {
        return Option::from("go away".to_owned());
    }

    match json["Type"].as_str().unwrap() {
        "login" => {
            return Option::from(if !json["Data"]["token"].is_null() && !get_player(json["Data"]["token"].as_str().unwrap()).await.is_none() {
                let balance: i32 = balance_get(json["Data"]["token"].as_str().unwrap()).await;
                json!({
                    "Type": "loginresponse",
                    "Data": {
                        "balance": balance,
                        "success": true
                    }
                })
            } else {
                json!({
                    "Type": "loginresponse",
                    "Data": {
                        "balance": 0,
                        "success": false
                    }
                })
            }.to_string())
        }
        "newAI" => {
            return Some(if json["Data"]["alpha"].is_null() || json["Data"]["gamma"].is_null() || json["Data"]["omega"].is_null() || json["Data"]["colour"].is_null() || json["Data"]["name"].is_null() {
                json!({
                    "Type": "AIresponse",
                    "Data": {
                        "success": false,
                        "balance": 0,
                        "horsename": []
                    }
                })

            } else {
                add_horse(json["Data"]["name"].as_str().unwrap().to_owned(), json["Data"]["colour"].as_str().unwrap().to_owned(),0, 0., String::new(), json["Data"]["alpha"].as_f32().unwrap().to_owned(), json["Data"]["beta"].as_f32().unwrap().to_owned(), json["Data"]["gamma"].as_f32().unwrap().to_owned());
                let balance: i32 = balance_get(json["Data"]["token"].as_str().unwrap()).await;
                let horselist: Vec<String> = name_horses();
                json!({
                    "Type": "AIresponse",
                    "Data": {
                        "success": true,
                        "balance": balance,
                        "horsename": horselist
                    }
                })


            }.to_string())
        }

        "bettinginfo" => {
            if !json["Data"]["bet"].is_null() && !json["Data"]["horsename"].is_null() && !json["Data"]["token"].is_null() {
                add_bet(json["Data"]["token"].as_str().unwrap().to_owned(), json["Data"]["horsename"].as_usize().unwrap(), json["Data"]["bet"].as_i32().unwrap());
            } else {
                println!("malformed packet from token: {}", json["Data"]["token"].as_str().unwrap());
            }
            return None
        }
        "race" => {
            //let data: Data_Res_Race = serde_json::from_str(&json.Data).unwrap();
        }


        _ => {println!("HAAAAAAAAAAAAAAA")}
    }
    Option::from("".to_owned())
}