use std::collections::HashMap;
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
    let events: Events = match serde_json::from_reader(rsp_reader) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // Display the fetched activity in the terminal.

    // Output:
    // - Pushed 3 commits to kamranahmedse/developer-roadmap
    // - Opened a new issue in kamranahmedse/developer-roadmap
    // - Starred kamranahmedse/developer-roadmap
    // - ...

    let mut push_repo_map: HashMap<String, i64> = HashMap::with_capacity(events.len());

    for event in events {
        if !(event.r#type == "PushEvent") {
            continue;
        }

        if let Some(r) = push_repo_map.get_mut(&event.repo.name) {
            *r = *r + 1
        } else {
            push_repo_map.insert(event.repo.name, 1);
        }
    }

    for (k, v) in push_repo_map.iter() {
        println!("Pushed {} commits to {}", v, k);
    }
}
