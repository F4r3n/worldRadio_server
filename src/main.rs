#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use dns_lookup::lookup_host;
use dns_lookup::lookup_addr;
use std::result::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json;
#[get("/yolo", rank = 1)]
fn yolo() -> &'static str {
    "Hello, world!"
}

struct RadioBrowser {
    server_list : Vec<String>
}

#[derive(Deserialize, Serialize)]
struct RadioData {
    url : String,
    icon : String,
    name: String
}

impl RadioBrowser {

    pub fn new() -> RadioBrowser {
        RadioBrowser {
            server_list : Vec::new(),
        }
    }

    pub fn update_server_list(& mut self) {
        self.server_list.clear();

        let hostname = "all.api.radio-browser.info";
        let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();
        for ip in ips.iter() {
            let ip = match lookup_addr(&ip) {
                Ok(i) => i,
                Err(error) => panic!("Problem dns lookup: {:?}", error),
            };
            self.server_list.push(ip);
        }
    }

    fn write(&self, in_content : &String) {
        let path = Path::new("test.json");
        let mut file = match File::create(&path) {
            Err(err) => panic!("Could not open file {}", err),
            Ok(file) => file
        };

        match file.write(in_content.as_bytes()) {
            Err(err) => panic!("Could not write {}", err),
            Ok(_) => println!("yolo")
        }
    }

    pub fn get_list_per_language(&self, in_language : String, in_country : String) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let body = rt.block_on(self.get_list_per_language_async(&self.server_list[0], in_language, in_country));
        let _body = match body {
            Ok(r) => r,
            Err(err) => panic!("{}", err)
        };
        let items = match _body.as_array() {
            Some(value) => Some(value),
            None => None
        };
        let mut radio_datas : Vec<RadioData> = Vec::new();

        if items.is_some() {
            for item in items.unwrap() {
                let data = RadioData {
                    url : String::from(item["name"].to_string()),
                    icon : String::from(item["name"].to_string()),
                    name: String::from(item["name"].to_string())
                };
                radio_datas.push(data);
            }
        }

        let json = serde_json::to_string(&radio_datas);
        println!("{}", &json.unwrap());
    }

    async fn get_list_per_language_async(&self, in_url : &String, in_language : String, in_country : String) -> Result<serde_json::Value, Box<dyn std::error::Error>>{
        //https://de1.api.radio-browser.info/json/stations/search?language=japanese&country=Japan&languageExact=true
        let url = format!("http://{}/json/stations/search?language={}&country={}&languageExact=true", in_url, in_language, in_country);

        let body = reqwest::get(url)
        .await?
        .json::<serde_json::Value>()
        .await?;
        Ok(body)
    }
}


fn main() {
    let mut radio_browser = RadioBrowser::new();
    radio_browser.update_server_list();
    radio_browser.get_list_per_language(String::from("japanese"), String::from("Japan"));
    /*
    rocket::ignite()
    .mount("/", routes![yolo])
    .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")).rank(10))
    .launch();
    */
}