extern crate selenium_rs;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
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
use std::{
    thread,
    time::Duration,
    fs::File,
    io::prelude::*,
    process::Command,
};
mod action;
use action::{
    Action,
    ActionManager,
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

fn main() {
    let app = init_clap();
    let matches = app.get_matches();
    let file_name = matches.value_of("file").unwrap();

    let mut file = File::open(file_name).unwrap();

    let mut actions: ActionManager = serde_yaml::from_reader(file).unwrap();
    actions.replace_vars();

    Command::new("chromedriver")
        .arg("--port=4444")
        .arg("--url-base=wd/hub")
        .spawn();

    let mut driver = WebDriver::new(Browser::Chrome);

    we!(driver.start_session());
    for action in actions.main {
        use Action::*;
        match action {
            Navigate {url} => {
                we!(driver.navigate(&url));
            },
            Click {dom, count} => {
                let count = count.unwrap_or(1);
                for _ in 0..count {
                    let element = we!(driver.query_element(Selector::CSS, &dom));
                    we!(element.click());
                }
            },
            Input {dom, value, enter} => {
                let element = we!(driver.query_element(Selector::CSS, &dom));
                we!(element.type_text(&value));
                if enter.unwrap_or(true) {
                    we!(element.type_text("\u{E007}"));
                }
            },
            Sleep {time} => {
                if time == -1 {
                    thread::sleep(Duration::from_millis(std::u64::MAX));
                } else {
                    thread::sleep(Duration::from_millis(time as u64 * 1000));
                }
            },
        }
    }
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