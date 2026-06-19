use crate::data::{AppState, LockData};
use crate::ui;
use crate::ui::sections::section::Section;
use crate::ui::sections::section_pin_positions::SectionPinPositions;
use crate::ui::sections::section_plate_links::SectionPlateLinks;
use cursive::view::Nameable;
use cursive::views::{Dialog, LinearLayout, RadioGroup, TextView};
use cursive::Cursive;


pub struct SectionPlateCount {}
impl Section for SectionPlateCount {
    const NAME: &'static str = "section_plate_count";

    fn create() -> Dialog {
        let mut group = RadioGroup::<u8>::new();
        let mut group_layout = LinearLayout::horizontal();

        for i in 0..4 {
            let lbl = if i < 3 {
                format!("{}   ", 4 + i)
            } else {
                format!("{}", 4 + i)
            };
            group_layout.add_child(group.button(4 + i, lbl));
        }

        group.set_on_change(|siv: &mut Cursive, v: &u8| {
            siv.with_user_data(|app_state: &mut AppState| {
                app_state.lock = LockData::from_num_plates(v);
            });
            SectionPinPositions::update(siv);
            SectionPlateLinks::update(siv);
        });

        let section = LinearLayout::vertical()
            .child(TextView::new(
                "Changing this will reset all other options.\n\n",
            ))
            .child(group_layout);

        ui::util::wrap_section(section.with_name(Self::NAME), "1. Number of Plates")
    }
}
