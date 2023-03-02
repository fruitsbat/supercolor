use strum::EnumIter;

#[derive(EnumIter, Debug)]
pub enum Builtins {
    Catpeek,
    LoveMyPuter,
    Gamebtw,
    Hashbang,
}

impl Builtins {
    pub fn get(&self) -> &'static str {
        match self {
            Builtins::Catpeek => include_str!("art/catpeek.txt"),
            Builtins::LoveMyPuter => include_str!("art/love_my_puter.txt"),
            Builtins::Gamebtw => include_str!("art/gamebtw.txt"),
            Builtins::Hashbang => include_str!("art/hashbang.txt"),
        }
    }
}
