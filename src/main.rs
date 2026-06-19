mod data;
mod solver;
mod ui;


use crate::data::{AppState, LockData};
use cursive::traits::{Nameable, Resizable, Scrollable};


fn main() {
    let mut siv = ui::ui::init(AppState {
        lock: LockData::from_num_plates(&4),
        plate_order_as_in_game: true,
    });

    siv.run();
}
