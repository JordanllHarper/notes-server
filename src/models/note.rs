#[derive(rocket::serde::Deserialize, rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Note {
    pub id: i32,
    pub contents: String,
}

impl Note {
    pub fn new(id: i32, contents: String) -> Note {
        Note { id, contents }
    }
}
