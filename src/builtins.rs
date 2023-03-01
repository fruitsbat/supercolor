use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Debug)]
pub enum Builtins {
    Catpeek,
    LoveMyPuter,
}

impl Builtins {
    pub fn get(&self) -> &'static str {
        match self {
            Builtins::Catpeek => include_str!("art/catpeek.txt"),
            Builtins::LoveMyPuter => include_str!("art/love_my_puter.txt"),
        }
    }
}
