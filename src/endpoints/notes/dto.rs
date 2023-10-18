use crate::models::note::Note;
#[derive(rocket::serde::Serialize, rocket::serde::Deserialize, Debug)]
pub struct GetNotesRequestDto {
    pub api_key: String,
}

impl GetNotesRequestDto {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
    pub fn dummy(api_key: String) -> Self {
        let dummy_data = "12345IAmAPIKey".to_string();
        Self {
            api_key: dummy_data,
        }
    }
}

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
pub struct GetNotesResponseDto {
    notes: Vec<crate::models::note::Note>,
}

impl std::fmt::Debug for GetNotesResponseDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.notes.iter().for_each(|n| {
            let _ = f.write_str(&n.id.to_string());
            let _ = f.write_str(&n.contents);
        });
        Ok(())
    }
}

impl GetNotesResponseDto {
    pub fn new(notes: Vec<crate::models::note::Note>) -> Self {
        Self { notes }
    }

    pub fn dummy() -> Self {
        let dummy_data = vec![
            Note::new(1, String::from("I am a test note")),
            Note::new(2, String::from("I am another test note!")),
            Note::new(3, String::from("I am yet another test note!")),
        ];
        Self { notes: dummy_data }
    }
}

