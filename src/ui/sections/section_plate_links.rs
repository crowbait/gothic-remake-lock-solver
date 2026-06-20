use crate::data::{AppState, LinkState, LockData};
use crate::ui;
use crate::ui::sections::section::Section;
use cursive::Cursive;
use cursive::reexports::enumset::__internal::EnumSetTypeRepr;
use cursive::view::Nameable;
use cursive::views::{Dialog, LinearLayout, RadioGroup, TextView};

pub struct SectionPlateLinks {}
impl Section for SectionPlateLinks {
    const NAME: &'static str = "section_plate_links";
    fn create(_app_state: &AppState) -> Dialog {
        let plates = LinearLayout::vertical().with_name(Self::NAME);

        let section = LinearLayout::vertical()
            .child(TextView::new(
                "For each plate, select which plates move with it in the same direction, \
                 the opposite direction, or not at all.\n\
                 Note that \"Plate 1\" is the bottommost plate in the game.\n\n\
                 ⏺️ = Unlinked (doesn't move)\n\
                 ✅ = Same (moves in the same direction)\n\
                 ❌ = Opposite (moves in the other direction)",
            ))
            .child(plates);

        ui::util::wrap_section(section, "3. Plate Links")
    }

    fn update(siv: &mut Cursive) {
        // retrieve number of plates from app state
        let num_plates = siv
            .with_user_data(|app_state: &mut AppState| {
                let num = app_state.lock.num_plates;
                // reset pin positions
                app_state.lock.links = LockData::from_num_plates(&num).links;
                num
            })
            .unwrap();

        // rewrite and populate section layout
        siv.call_on_name(Self::NAME, |section: &mut LinearLayout| {
            section.clear();

            // plate boxes
            for i in 0..num_plates {
                let mut rows = LinearLayout::vertical();

                for j in 0..num_plates {
                    if j != i {
                        let mut link = RadioGroup::<LinkState>::new();

                        link.set_on_change(move |siv_cb, v| {
                            siv_cb.with_user_data(|app_state: &mut AppState| {
                                app_state.lock.links[i.to_usize()][j.to_usize()] = *v;
                            });
                        });

                        let row = LinearLayout::horizontal()
                            .child(TextView::new(format!("   Plate {}:  ", j + 1)))
                            .child(link.button(LinkState::Unlinked, "⏺️   "))
                            .child(link.button(LinkState::Same, "✅   "))
                            .child(link.button(LinkState::Opposite, "❌"));

                        rows.add_child(row);
                    } else {
                        rows.add_child(TextView::new(format!("   Plate {}:  ---", j + 1)));
                    };
                }

                let plate_box = LinearLayout::vertical()
                    .child(TextView::new(format!("\nPlate {}:", i + 1)))
                    .child(rows);
                section.add_child(plate_box);
            }
        });
    }
}
