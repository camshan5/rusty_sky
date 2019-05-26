// Rusty Sky is a weather fetching API app writen in Rust,
// using the Dark Sky API. More info @ https://darksky.net/dev/docs

// URL options for Forecast Requests:
// -> https://api.darksky.net/forecast/[key]/[latitude],[longitude]

extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate url;

use std::io::Read;
use url::Url;
use serde_json::{Value, Error};

// disables unused value warning
#[allow(unused_must_use)]
fn main() {
    // TODO: replace lat & lon to accept input or use current location
    get_temp("37.0000", "-122.0000");
}

#[allow(unused_must_use)]
fn get_temp(lat: &str, lon: &str) -> Result<(), Error> {

    // TODO: Add secret key from toml file
    let key = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    let url = format!("https://api.darksky.net/forecast/{}/{},{}", key, lat, lon);
    let mut resp = reqwest::get(Url::parse(&url).unwrap()).unwrap();

    assert!(resp.status().is_success());

    // create a new string & a mutable reference to 'content'
    let mut content = String::new();
    resp.read_to_string(&mut content);

    let v: Value = serde_json::from_str(content.as_str())?;

    // TODO: Replace with input for the key value?
    //  E.g. ["currently"], ["minutely"]["summary"], ["data"]["temperature"],

    let ref current = v["hourly"]["summary"];

    // E.g. response: "Weather Update: 'Light rain starting in 30 min.'"
    println!("Weather Update: {}", current);

    Ok(())
}