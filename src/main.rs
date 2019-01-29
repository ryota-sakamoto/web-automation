extern crate selenium_rs;
extern crate yaml_rust;
extern crate clap;

use selenium_rs::webdriver::{
    Browser,
    WebDriver,
    Selector,
};
use clap::{
    App,
    Arg,
};
use yaml_rust::{
    YamlLoader,
    Yaml,
};
use std::{
    thread,
    time::Duration,
    fs::File,
    io::prelude::*,
    process::Command,
};
mod action;
use action::Action;

macro_rules! we {
    ($t:expr) => {
        match $t {
            Ok(r) => r,
            Err(e) => {
                println!("{:?}", e);
                panic!();
            },
        }
    };
}

fn main() {
    let app = init_clap();
    let matches = app.get_matches();
    let file_name = matches.value_of("file").unwrap();

    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let mut docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];
    let mut actions = Vec::new();

    for ref v in doc["main"].as_vec() {
        for a in v.iter() {
            let action = Action::from(a);
            actions.push(action);
        }
    }

    Command::new("chromedriver")
        .arg("--port=4444")
        .arg("--url-base=wd/hub")
        .spawn();

    let mut driver = WebDriver::new(Browser::Chrome);

    we!(driver.start_session());
    for action in actions {
        use Action::*;
        match action {
            Navigate {url, sleep} => {
                thread::sleep(Duration::from_millis((sleep.before * 1000) as u64));
                we!(driver.navigate(&url));
                thread::sleep(Duration::from_millis((sleep.after * 1000) as u64))
            },
            Click {dom, count, wait} => {
                for _ in 0..count {
                    thread::sleep(Duration::from_millis(wait * 1000));
                    let element = we!(driver.query_element(Selector::CSS, &dom));
                    we!(element.click());
                }
            },
            Input {dom, value, enter} => {
                let element = we!(driver.query_element(Selector::CSS, &dom));
                we!(element.type_text(&value));
                if enter {
                    we!(element.type_text("\u{E007}"));
                }
            },
            _ => {
                println!("Not Implement: {:?}", action);
            }
        }
    }

    thread::sleep(Duration::from_millis(10000));
}

fn init_clap<'a, 'b>() -> App<'a, 'b> {
    App::new("web-automation")
        .arg(
            Arg::with_name("file")
                .help("test file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
        )
}