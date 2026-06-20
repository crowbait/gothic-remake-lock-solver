mod data;
mod solver;
mod ui;

use crate::data::AppState;

fn main() {
    let mut siv = ui::ui::init(AppState::default());

    siv.run();
}
