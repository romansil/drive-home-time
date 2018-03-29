extern crate reqwest;

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

    let res = reqwest::get(url).unwrap().text().unwrap();
    println!("res: {}", res);
}
