use serde::Deserialize;
use serde_derive::Serialize;

#[derive(Deserialize)]
struct Top_Layer {
    Type: String,
    Data: String
}

#[derive(Deserialize)]
struct Data_Rec_Login {
    token: String
}

#[derive(Serialize)]
struct Data_Res_Login {
    balance: i32,
    success: bool
}

#[derive(Deserialize)]
struct Data_Rec_Newai {
    alpha: f64,
    gamma: f64,
    colour: String,
    name: String
}

#[derive(Serialize)]
struct Data_Res_Airesponse {
    balance: i32,
    horsename: Vec<String>,
    success: bool
}

#[derive(Deserialize)]
struct Data_Rec_Betinfo {
    bet: i32,
    horsename: String
}

#[derive(Serialize)]
struct Data_Res_Race {
    balance: i32
}

pub fn handle(text: String) -> String {
    let json: Top_Layer = serde_json::from_str(&text).unwrap();
    match json.Type.as_str() {
        "login" => {
            let data: Data_Rec_Login = serde_json::from_str(&json.Data).unwrap();

            let obj: Data_Res_Login = Data_Res_Login {
                balance: 0,
                success: true
            };

            return serde_json::to_string(&obj).unwrap();
        }
        "login response" => {
            //let data: Data_Res_Login = serde_json::from_str(&json.Data).unwrap();
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