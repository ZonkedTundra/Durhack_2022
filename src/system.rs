use std::borrow::Borrow;
use std::collections::HashMap;
use std::iter::Map;
use bevy::reflect::serde::Serializable::Owned;
use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use crate::betting_maths::{add_bet, add_horse, name_horses};
use crate::CONFIG;
use crate::database::{balance_get, get_player, player_new};

pub async fn handle(text: String) -> Option<String> {
    // efficiency is so yesterday
    let mut json = json::parse(&*text).unwrap();
    if json["Type"].is_null() {
        return Option::from("go away".to_owned());
    }

    return match json["Type"].as_str().unwrap() {
        "login" => {
            return Option::from(if !json["token"].is_null() && !get_player(json["token"].as_str().unwrap()).await.is_none() {
                let balance: i32 = balance_get(json["token"].as_str().unwrap()).await;
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
            return Option::from(if json["Data"]["alpha"].is_null() || json["Data"]["gamma"].is_null() || json["Data"]["omega"].is_null() || json["Data"]["colour"].is_null() || json["Data"]["name"].is_null() {
                add_horse(json["Data"]["name"].as_str().unwrap().to_owned(), json["Data"]["colour"].as_str().unwrap().to_owned(), json["Data"]["alpha"].as_f32().unwrap().to_owned(), json["Data"]["beta"].as_f32().unwrap().to_owned(), json["Data"]["gamma"].as_f32().unwrap().to_owned());
                json!({
                    "Type": "AIresponse",
                    "Data": {
                        "success": false,
                        "balance": 0,
                        "horsename": []
                    }
                })
            } else {
                add_horse(json["Data"]["name"].as_str().unwrap().to_owned(), json["Data"]["colour"].as_str().unwrap().to_owned(),json["Data"]["alpha"].as_f32().unwrap().to_owned(), json["Data"]["gamma"].as_f32().unwrap().to_owned(), json["Data"]["omega"].as_f32().unwrap().to_owned());
                let balance: i32 = balance_get(json["token"].as_str().unwrap()).await;
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
            if !json["Data"]["bet"].is_null() && !json["Data"]["horsename"].is_null() && !json["token"].is_null() {
                add_bet(json["token"].as_str().unwrap().to_owned(), json["Data"]["horsename"].as_usize().unwrap(), json["Data"]["bet"].as_i32().unwrap());
            } else {
                println!("malformed packet with token: {}", json["token"].as_str().unwrap());
            }
            return None
        }
        "race" => {
            None
            //let data: Data_Res_Race = serde_json::from_str(&json.Data).unwrap();
        }
        "requestbetting" => {
            return Some(if !json["token"].is_null() {
                let balance: i32 = balance_get(json["token"].as_str().unwrap()).await;
                let horselist: Vec<String> = name_horses();
                json!({
                    "Type": "AIresponse",
                    "Data": {
                        "success": true,
                        "balance": balance,
                        "horsename": horselist
                    }
                })
            } else {
                json!({
                    "Type": "AIresponse",
                    "Data": {
                        "success": false,
                        "balance": 0,
                        "horsename": []
                    }
                })
            }.to_string())
        }
        "newtoken" => {
            if !json["token"].is_null() {
                // hardcoded admin password (budget cuts)
                if json["token"] == "admin22" {
                    let new_token: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
                    println!("{}", new_token);
                    player_new(&new_token).await;
                    return Some(json!({
                        "Type": "alert",
                        "Data": {
                            "value": new_token
                        }
                    }).to_string());
                }
            }
            return None;
        }
        _ => {None}
    }
}