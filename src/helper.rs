use crate::cache::Cache;
use regex::Regex;
use serde_json::value::Value;

pub struct Helper{
    re_cache: Cache<String, Regex>
}
impl Helper {
    pub fn new() -> Self {
        Helper{
            re_cache: Cache::new(Box::new(|x|Regex::new(x).unwrap()))
        }
    }
    pub fn regex_match(&self, pattern: &Value, string: &Value) -> bool {
        match (pattern, string) {
            (Value::String(pattern), Value::String(string)) => {
                let re = self.re_cache.get(pattern);
                re.is_match(string)
            }
            _ => false
        }

    }
}

