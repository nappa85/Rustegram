use std::collections::HashMap;
use std::cmp;

use serde_json::value::Value;

/// Session here is seen like an event store
/// where session value is given by the aggregate of single session values
pub struct Session {
    vars: HashMap<String, Vec<Value>>,
}

impl Session {
    /// constructor
    pub fn new() -> Session {
        Session {
            vars: HashMap::new(),
        }
    }

    /// retrieves a session aggregate
    pub fn get(&self, key: &str) -> Option<Value> {
        match self.vars.get(key) {
            Some(v) => {
                if v.len() == 0 {
                    return None;
                }

                let mut val = json!(null);
                for x in v {
                    val = Session::aggregate(&val, x);
                }
                Some(val)
            },
            None => None,
        }
    }

    /// replaces a session
    pub fn set(&mut self, key: &str, value: Value) {
        self.vars.insert(key.to_string(), vec![value]);
    }

    /// appends a value to a session
    pub fn add(&mut self, key: &str, value: Value) {
        if self.vars.contains_key(key) {
            match self.vars.get_mut(key) {
                Some(v) => {
                    v.push(value);
                    return;
                },
                None => {},
            }
        }

        self.set(key, value);
    }

    fn aggregate(a: &Value, b: &Value) -> Value {
        if a.is_array() && b.is_array() {
            let mut v = a.clone();
            {
                let arr_a = v.as_array_mut().unwrap();
                let arr_b = b.as_array().unwrap();
                let min = cmp::min(arr_a.len(), arr_b.len());
                for i in 0..min {
                    arr_a[i] = Session::aggregate(&arr_a[i], &arr_b[i]);
                }

                if arr_a.len() < arr_b.len() {
                    for i in min..arr_b.len() {
                        arr_a[i] = arr_b[i].clone();
                    }
                }
            }
            v
        }
        else if a.is_object() && b.is_object() {
            let mut v = a.clone();
            {
                let obj_a = v.as_object_mut().unwrap();
                let obj_b = b.as_object().unwrap();

                let mut keys: Vec<String> = Vec::new();
                for i in obj_a.keys() {
                    keys.push(i.to_string());
                }
                for i in obj_b.keys() {
                    if !keys.contains(&i) {
                        keys.push(i.to_string());
                    }
                }

                for i in &keys {
                    if obj_a.contains_key(i) && obj_b.contains_key(i) {
                        let temp = Session::aggregate(&obj_a[i], &obj_b[i]);
                        obj_a.insert(i.to_string(), temp);
                    }
                    else if obj_b.contains_key(i) {
                        obj_a.insert(i.to_string(), obj_b[i].clone());
                    }
                }
            }
            v
        }
        else {
            b.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::Session;

    #[test]
    fn it_works() {
        let mut session = Session::new();
        let key = "test";
        session.add(key, json!({"a": 123}));
        session.add(key, json!({"b": 456}));
        session.add(key, json!({"a": 456}));
        assert_eq!(session.get(key).unwrap(), json!({"a": 456, "b": 456}));
        session.add(key, json!({"b": {"c": {"d": true}}}));
        session.add(key, json!({"b": {"c": {"e": 12.25}}}));
        session.add(key, json!({"b": {"c": {"d": "false"}}}));
        assert_eq!(session.get(key).unwrap(), json!({"a": 456, "b": {"c": {"d": "false", "e": 12.25}}}));
        session.add(key, json!({"f": [1, 2, 3]}));
        session.add(key, json!({"f": [4, 5, 6]}));
        assert_eq!(session.get(key).unwrap(), json!({"a": 456, "b": {"c": {"d": "false", "e": 12.25}}, "f": [4, 5, 6]}));
    }
}
