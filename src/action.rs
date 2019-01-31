use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionManager {
    vars: HashMap<String, String>,
    pub main: Vec<Action>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Navigate {
        url: String,
    },
    Click {
        dom: String,
        count: Option<u64>,
    },
    Input {
        dom: String,
        value: String,
        enter: Option<bool>,
    },
    Sleep {
        time: i64,
    },
}

impl ActionManager {
    pub fn replace_vars(&mut self) {
        use Action::*;
        self.main = self.main.iter().map(|action| {
            match action {
                Navigate {url} => {
                    Navigate {
                        url: self.replace_value(url),
                    }
                },
                Click {dom, count} => {
                    Click {
                        dom: self.replace_value(dom),
                        count: count.to_owned(),
                    }
                },
                Input {dom, value, enter} => {
                    Input {
                        dom: self.replace_value(dom),
                        value: self.replace_value(value),
                        enter: enter.to_owned(),
                    }
                },
                Sleep {time} => {
                    Sleep {
                        time: time.to_owned(),
                    }
                },
            }
        }).collect();
    }

    fn replace_value(&self, value: &str) -> String {
        let mut value = value.to_string();
        for (k, v) in &self.vars {
            value = value.replace(&format!("${}", k), v);
        }
        value
    }
}