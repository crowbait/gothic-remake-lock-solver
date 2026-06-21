use crate::common::solver::Move;
use cursive::reexports::enumset::__internal::EnumSetTypeRepr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkState {
    Same,
    Opposite,
    Unlinked,
}

#[derive(Clone, Debug)]
pub struct LockData {
    pub num_plates: u8,
    pub pin_positions: Vec<u8>,
    pub links: Vec<Vec<LinkState>>,
}

impl LockData {
    pub fn from_num_plates(num_plates: &u8) -> Self {
        LockData {
            num_plates: *num_plates,
            pin_positions: vec![3; num_plates.to_usize()],
            links: vec![vec![LinkState::Unlinked; num_plates.to_usize()]; num_plates.to_usize()],
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub lock: LockData,
    pub solution: Option<Vec<Move>>,
    pub plate_order_as_in_game: bool,
    pub group_steps: bool,
}

impl AppState {
    pub fn default() -> Self {
        AppState {
            lock: LockData::from_num_plates(&4),
            solution: None,
            plate_order_as_in_game: true,
            group_steps: true,
        }
    }
}
