use std::collections::HashMap;
use std::convert::From;

#[derive(std::fmt::Debug)]
pub struct QueryString {
    data: HashMap<String, String>,
}

impl QueryString {
    pub fn get(&self, key: &String) -> Option<&String> {
        self.data.get(key)
    }
}

impl From<&str> for QueryString {
    fn from(s: &str) -> Self {
        let mut data = HashMap::new();

        //name=bond & age=18
        for c in s.split('&') {
            if let Some(i) = c.find('=') {
                let key = &c[..i];
                let value = &c[i + 1..];
                data.insert(key.to_string(), value.to_string());
            }
        }

        QueryString { data }
    }
}
