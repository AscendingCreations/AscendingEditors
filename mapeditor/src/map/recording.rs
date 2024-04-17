use crate::attributes::*;
use graphics::*;
use indexmap::IndexMap;

const MAX_CHANGE: usize = 500;

#[derive(Debug)]
pub enum RecordType {
    Layer,
    Attribute,
    Zone,
}

#[derive(Debug)]
pub struct ChangeData {
    pub record_type: RecordType,
    pub pos: Vec3,
    pub id: i64,
    pub data: Vec<InsertTypes>,
}

#[derive(Debug)]
pub struct Record {
    pub changes: IndexMap<String, ChangeData>,
}
pub struct Records {
    in_record: bool,
    pub undo: Vec<Record>,
    pub redo: Vec<Record>,
    last_index: Option<usize>,
}

impl Records {
    pub fn new() -> Self {
        Self {
            in_record: false,
            undo: Vec::new(),
            redo: Vec::new(),
            last_index: None,
        }
    }

    pub fn set_undo_record(&mut self) {
        if self.in_record {
            return;
        }

        self.in_record = true;
        let index = self.undo.len();
        self.last_index = Some(index);
        self.undo.push(Record {
            changes: IndexMap::new(),
        });
    }

    pub fn push_undo(
        &mut self,
        pos: Vec3,
        record_type: RecordType,
        id: i64,
        data: Vec<InsertTypes>,
    ) {
        if !self.in_record {
            return;
        }
        if self.undo.len() >= MAX_CHANGE {
            return;
        }

        if let Some(index) = self.last_index {
            let key_name = format!("{}_{}_{}", pos.x, pos.y, pos.z);
            if !self.undo[index].changes.contains_key(&key_name) {
                self.undo[index].changes.insert(
                    key_name,
                    ChangeData {
                        record_type,
                        pos,
                        id,
                        data,
                    },
                );
            }
        }
    }

    pub fn get_last_undo(&mut self) -> Option<Record> {
        self.undo.pop()
    }

    pub fn set_redo_record(&mut self) {
        if self.in_record {
            return;
        }

        self.in_record = true;
        let index = self.redo.len();
        self.last_index = Some(index);
        self.redo.push(Record {
            changes: IndexMap::new(),
        });
    }

    pub fn push_redo(
        &mut self,
        pos: Vec3,
        record_type: RecordType,
        id: i64,
        data: Vec<InsertTypes>,
    ) {
        if !self.in_record {
            return;
        }
        if self.redo.len() >= MAX_CHANGE {
            return;
        }

        if let Some(index) = self.last_index {
            let key_name = format!("{}_{}_{}", pos.x, pos.y, pos.z);
            if !self.redo[index].changes.contains_key(&key_name) {
                self.redo[index].changes.insert(
                    key_name,
                    ChangeData {
                        record_type,
                        pos,
                        id,
                        data,
                    },
                );
            }
        }
    }

    pub fn get_last_redo(&mut self) -> Option<Record> {
        self.redo.pop()
    }

    pub fn clear_redo(&mut self) {
        self.redo.clear();
    }

    pub fn stop_record(&mut self) {
        if !self.in_record {
            return;
        }
        self.in_record = false;
        self.last_index = None;
    }
}
