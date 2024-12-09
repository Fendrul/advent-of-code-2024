use std::fmt::{Debug, Formatter};

pub struct FileBlock {
    bloc_size: i32,
    bloc_type: FileBlockType,
}

impl FileBlock {
    pub fn new(bloc_size: i32, bloc_type: FileBlockType) -> Self {
        Self {
            bloc_size,
            bloc_type,
        }
    }

    pub fn get_size(&self) -> i32 {
        self.bloc_size
    }

    pub fn get_type(&self) -> FileBlockType {
        self.bloc_type
    }
}

#[derive(Copy, Clone)]
pub enum FileBlockType {
    Number(usize),
    Void,
}

impl Debug for FileBlockType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileBlockType::Number(number) => write!(f, "{}", number),
            FileBlockType::Void => write!(f, "."),
        }
    }
}
