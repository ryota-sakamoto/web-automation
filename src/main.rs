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
};

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

#[derive(Debug, Clone)]
enum Action {
    Navigate {
        url: String,
        sleep: Sleep,
    },
    Click {
        dom: String,
        count: u64,
        wait: u64,
    },
    Input {
        dom: String,
        value: String,
        enter: bool,
    },
    Invalid,
}

#[derive(Debug, Clone)]
struct Sleep {
    before: i64,
    after: i64,
}

impl<'a> From<&'a Yaml> for Action {
    fn from(y: &Yaml) -> Action {
        use Action::*;
        let h = y.as_hash().unwrap();
        h.iter().map(|kv| {
            let k = kv.0;
            let v = kv.1;

            let s = k.as_str().unwrap();
            match s {
                "navigate" => {
                    Navigate {
                        url: v["url"].as_str().unwrap().to_string(),
                        sleep: Sleep {
                            before: v["sleep"]["before"].as_i64().unwrap_or(0),
                            after: v["sleep"]["after"].as_i64().unwrap_or(0),
                        },
                    }
                },
                "click" => {
                    Click {
                        dom: v["dom"].as_str().unwrap().to_string(),
                        count: v["count"].as_i64().unwrap_or(1) as u64,
                        wait: v["wait"].as_i64().unwrap_or(1) as u64,
                    }
                },
                "input" => {
                    Input {
                        dom: v["dom"].as_str().unwrap().to_string(),
                        value: v["value"].as_str().unwrap().to_string(),
                        enter: v["enter"].as_bool().unwrap_or(false),
                    }
                },
                _ => {
                    Invalid
                }
            }
        }).collect::<Vec<Action>>().get(0).unwrap_or(&Invalid).clone()
    }
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