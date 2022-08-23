#[derive(Clone, Default, PartialEq, Eq)]
pub struct ListData {
    pub id: usize,
    pub name: String,
}

impl ListData {
    pub fn new(id: usize, name: String) -> Self {
        Self { id, name }
    }
}
impl From<ListData> for String {
    fn from(list_data: ListData) -> String {
        format!("{}: {}", list_data.id, &list_data.name)
    }
}

impl std::fmt::Debug for ListData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}: {}", self.id, self.name)[..])
    }
}

impl std::fmt::Display for ListData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}: {}", self.id, self.name)[..])
    }
}
