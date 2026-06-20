use crate::data::AppState;
use cursive::Cursive;
use cursive::views::Dialog;

pub trait Section {
    const NAME: &'static str;
    fn create(app_state: &AppState) -> Dialog;

    fn update(_siv: &mut Cursive) {
        panic!("Trying to update section with no update method.");
    }
}
