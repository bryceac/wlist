pub struct Note {
    pub id: u32,
    pub note: String
}

impl Note {
    pub fn from(id: u32, note: &str) -> Self {
        Self {
            id,
            note: note.to_owned()
        }
    }
}