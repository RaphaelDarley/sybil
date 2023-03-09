use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Item {
    Note,
    Person,
}

#[derive(Deserialize, Debug)]
pub struct Note {
    text: String,
}

#[derive(Deserialize, Debug)]
pub struct Person {
    name: String,
}
