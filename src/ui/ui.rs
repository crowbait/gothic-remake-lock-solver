use crate::auto_update::get_available_version;
use crate::data::AppState;
use crate::ui::layout::create_layout;
use crate::ui::sections::section::Section;
use crate::ui::sections::section_pin_positions::SectionPinPositions;
use crate::ui::sections::section_plate_links::SectionPlateLinks;
use crate::ui::theme;
use cursive::CursiveRunnable;
use cursive::views::TextView;

pub fn init(initial_app_state: AppState) -> CursiveRunnable {
    let mut siv = cursive::default();
    siv.set_theme(theme::get_theme());

    siv.set_user_data(initial_app_state);

    siv.add_layer(TextView::new("Checking for updates..."));
    let update = get_available_version();
    siv.pop_layer();

    create_layout(&mut siv, &update.ok().flatten());

    // component initialization
    SectionPinPositions::update(&mut siv);
    SectionPlateLinks::update(&mut siv);

    siv
}
