extern crate reqwest;

use std::io::Read;
use reqwest::Client;
use reqwest::Url;

const URL: &'static str = "https://maps.googleapis.com/maps/api/distancematrix/json";
//const URL: &'static str = "http://httpbin.org/get";

fn main() {

    let params = [
        ("units", "metric"),
        ("origins", "New York"),
        ("destinations", "Washington DC"),
        //("key", option_env!("API_KEY").unwrap())
    ];

    let url = Url::parse_with_params(URL, &params).unwrap();

    let client = Client::new();
    let mut res = client.get(url).send().unwrap();

    //println!("res: {:?}", res);
    let mut res_txt = String::new();
    res.read_to_string(&mut res_txt).unwrap();
    println!("{}", res_txt);
}
