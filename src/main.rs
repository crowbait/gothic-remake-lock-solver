mod auto_update;
mod common;
mod data;
mod ui;

use crate::data::AppState;

fn main() {
    let mut siv = ui::ui::init(AppState::default());

    siv.run();
}
