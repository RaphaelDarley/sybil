use std::collections::BTreeMap;

use serde::Deserialize;
use surrealdb::sql::{Strand, Value};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Item {
    Note(Note),
    Person(Person),
}

impl Item {
    fn to_map(&self) -> BTreeMap<String, Value> {
        match self {
            Item::Note(n) => n.to_map(),
            Item::Person(p) => p.to_map(),
        }
    }

    pub fn add_to_vars(&self, vars: &mut BTreeMap<String, Value>) {
        vars.append(&mut self.to_map());
    }
}

trait ToMap {
    fn to_map(&self) -> BTreeMap<String, Value>;
}

#[derive(Deserialize, Debug)]
pub struct Note {
    text: String,
}

impl ToMap for Note {
    fn to_map(&self) -> BTreeMap<String, Value> {
        let mut map = BTreeMap::new();
        map.insert(
            "type".to_string(),
            surrealdb::sql::Value::Strand(Strand::from("note")),
        );
        map.insert(
            "text".to_string(),
            surrealdb::sql::Value::Strand(Strand::from(self.text.clone())),
        );
        map
    }
}

#[derive(Deserialize, Debug)]
pub struct Person {
    name: String,
    nickname: String,
}

impl ToMap for Person {
    fn to_map(&self) -> BTreeMap<String, Value> {
        let mut map = BTreeMap::new();
        map.insert(
            "type".to_string(),
            surrealdb::sql::Value::Strand(Strand::from("person")),
        );
        map.insert(
            "name".to_string(),
            surrealdb::sql::Value::Strand(Strand::from(self.name.clone())),
        );
        map.insert(
            "nickname".to_string(),
            surrealdb::sql::Value::Strand(Strand::from(self.nickname.clone())),
        );
        map
    }
}
