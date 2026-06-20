use crate::data::{AppState, LockData};
use crate::ui;
use crate::ui::sections::section::Section;
use cursive::Cursive;
use cursive::reexports::enumset::__internal::EnumSetTypeRepr;
use cursive::style::Effect::Dim;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Checkbox, Dialog, DummyView, LinearLayout, RadioGroup, TextView};

pub struct SectionPinPositions {}
impl Section for SectionPinPositions {
    const NAME: &'static str = "section_pin_positions";
    fn create(app_state: &AppState) -> Dialog {
        let rows = LinearLayout::vertical().with_name(Self::NAME);

        let mut header = LinearLayout::horizontal().child(TextView::new("          1"));
        for slot in 2..8 {
            header.add_child(TextView::new(format!("    {}", slot)));
        }

        let mut chk_plate_order = Checkbox::new().on_change(|siv, checked| {
            siv.with_user_data(|app_state: &mut AppState| {
                app_state.plate_order_as_in_game = checked;
            });
            Self::update(siv);
        });
        if app_state.plate_order_as_in_game {
            chk_plate_order.check();
        }

        let section = LinearLayout::vertical()
            .child(
                LinearLayout::horizontal().child(chk_plate_order).child(
                    LinearLayout::vertical()
                        .child(TextView::new(" Plate order as in-game (1 = bottom)"))
                        .child(TextView::new(" Changing this resets pin positions.").style(Dim)),
                ),
            )
            .child(DummyView.fixed_height(1))
            .child(header)
            .child(rows);

        ui::util::wrap_section(section, "2. Pin Positions")
    }

    fn update(siv: &mut Cursive) {
        // retrieve number of plates from app state
        let (num_plates, plate_order_as_in_game) = siv
            .with_user_data(|app_state: &mut AppState| {
                let num = app_state.lock.num_plates;
                // reset pin positions
                app_state.lock.pin_positions = LockData::from_num_plates(&num).pin_positions;
                (num, app_state.plate_order_as_in_game)
            })
            .unwrap();

        let plate_indices: Box<dyn Iterator<Item = u8>> = if plate_order_as_in_game {
            // collect -> Vec, needed because rev is not same type as base iteration
            Box::new((0..num_plates).rev())
        } else {
            Box::new(0..num_plates)
        };

        // rewrite and populate rows layout
        siv.call_on_name(Self::NAME, |section: &mut LinearLayout| {
            section.clear();
            for i in plate_indices {
                let mut new_group = RadioGroup::<u8>::new();
                let mut row = LinearLayout::horizontal();
                row.add_child(TextView::new(format!("Plate {}  ", i + 1)));

                for j in 0..7 {
                    let mut button = new_group.button(j, " ");
                    if j == 3 {
                        button.select();
                    }

                    row.add_child(button);
                }

                new_group.set_on_change(move |siv_cb, v| {
                    siv_cb.with_user_data(|app_state: &mut AppState| {
                        app_state.lock.pin_positions[i.to_usize()] = *v;
                    });
                });

                section.add_child(row);
            }
        });
    }
}
