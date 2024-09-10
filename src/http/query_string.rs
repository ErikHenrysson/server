use std::vec;
use std::collections::HashMap;

// Structure of the QueryString object. Translates the query string (Everything after path and before protocol) and stores its values in a HashMap
// If multiple values have the same key it is stored as a vector at the key-location in the HashMap, otherwise a Single value is stored.
#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

// Structure of the value stored in the HashMap. Stored as &str if the key only has one value. Stored as Vec<&str> if the key has multiple values.
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>), 
}

// Implementation of getter for a specific key
impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        return self.data.get(key);
    }

}

// Implementation of unfailable From function for the QueryString
// Takes a &str input containing the full querystring, splits it and ads it to the correct location in the HashMap
// Returns a new populated QueryString 
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        // Split the input at the correct location (&)
        for sub_str in s.split('&'){
            let mut key = sub_str;
            let mut val = "";
            // Finds the location of a key and value pair
            if let Some(i) = sub_str.find('='){
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            // Enters the key-value pair into the HashMap
            // If the key already has a value, change it to the "Multiple" variant of the Value enumeration and add all values to the key. 
            data.entry(key)
            .and_modify(|existing: &mut Value| match existing {
                Value::Single(prev_val) => {
                    *existing = Value::Multiple(vec![prev_val,val]);
                }
                Value::Multiple(vec) => vec.push(val)
                
            })
            .or_insert(Value::Single(val));
        }
        // Return a QueryString with the created HashMap
        QueryString { data }
    }    
}