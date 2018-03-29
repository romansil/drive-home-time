extern crate reqwest;

use std::io::Read;

const URL: &'static str = "https://maps.googleapis.com/maps/api/distancematrix/json";

fn main() {
    let key = option_env!("API_KEY").unwrap();

    let params = [
        ("units", "imperial"),
        ("origins", "Washington,DC"),
        ("destinations", "New+York+City,NY"),
        ("key", key)
    ];
    let client = reqwest::Client::new();
    let mut res = client.get(URL)
        .form(&params)
        .send()
        .unwrap();

    //println!("res: {:?}", res);
    let mut buf = String::new();
    res.read_to_string(&mut buf).expect("Failed to read response");
    println!("{}", buf);
}
