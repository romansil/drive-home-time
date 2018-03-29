extern crate reqwest;
extern crate time;

use reqwest::Url;
use std::env;
use time::Duration;


const URL: &'static str = "https://maps.googleapis.com/maps/api/distancematrix/json";
//const URL: &'static str = "http://httpbin.org/get";


#[derive(Debug)]
struct Way {
    src: String,
    dst: String,
}

impl Way {
    fn new(args: &[String]) -> Way {
        let src = args[1].clone();
        let dst = args[2].clone();

        Way { src, dst }
    }

    fn get_time(self) {

        let params = [
            ("units", "metric"),
            ("origins", self.src.as_str()),
            ("destinations", self.dst.as_str()),
            //("key", option_env!("API_KEY").unwrap())
        ];

        let url = Url::parse_with_params(URL, &params).unwrap();
        let res = reqwest::get(url).unwrap().text().unwrap();
        println!("res: {}", res);
    }
}


fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Usage: drive-home-time source destination");
    }

    let way = Way::new(&args);
    way.get_time();

    let t = Duration::seconds(1964);
    println!("t {:?}", t);
    println!("t {} min", t.num_minutes());
}
