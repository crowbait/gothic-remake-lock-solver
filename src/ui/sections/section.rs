use cursive::views::Dialog;
use cursive::Cursive;


pub trait Section {
    const NAME: &'static str;

    fn create() -> Dialog;

    fn update(siv: &mut Cursive) {
        panic!("Trying to update section with no update method.");
    }
}
