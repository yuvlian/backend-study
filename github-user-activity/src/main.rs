use std::env;

mod models;
use models::Events;

fn main() {
    let mut args = env::args();

    if !(args.len() == 2) {
        println!("Usage: github-user-activity <username>");
        return;
    }

    let username = args.nth(1).unwrap();

    if username.is_empty() {
        println!("Usage: github-user-activity <username>");
        return;
    }

    let req = match ureq::get(format!("https://api.github.com/users/{}/events", username)).call() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let rsp_reader = req.into_body().into_reader();
    let rsp: Events = match serde_json::from_reader(rsp_reader) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // i aint doin allat
    println!("{:#?}", rsp);
}
