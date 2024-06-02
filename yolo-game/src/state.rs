use super::*;

pub type Bytes<'a> = &'a [u8];

pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");

pub const SCORE_BOARD: Item<ScoreBoard> = Item::new("score_board");

pub const DENOM: Item<String> = Item::new("denom");
