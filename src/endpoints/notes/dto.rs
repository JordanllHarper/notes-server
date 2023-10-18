use crate::models::note::Note;

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
pub struct NotesResponseDto {
    notes: Vec<crate::models::note::Note>,
}

impl std::fmt::Debug for NotesResponseDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.notes.iter().for_each(|n| {
            let _ = f.write_str(&n.id.to_string());
            let _ = f.write_str(&n.contents);
        });
        Ok(())
    }
}

impl NotesResponseDto {
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

