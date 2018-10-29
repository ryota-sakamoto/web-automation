use yaml_rust::{
    Yaml,
};

#[derive(Debug, Clone)]
pub enum Action {
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
pub struct Sleep {
    pub before: i64,
    pub after: i64,
}

impl<'a> From<&'a Yaml> for Action {
    fn from(y: &Yaml) -> Action {
        use self::Action::*;
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
