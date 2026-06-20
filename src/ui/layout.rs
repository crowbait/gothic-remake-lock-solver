use crate::data::AppState;
use crate::ui::sections::section::Section;
use crate::ui::sections::section_pin_positions::SectionPinPositions;
use crate::ui::sections::section_plate_count::SectionPlateCount;
use crate::ui::sections::section_plate_links::SectionPlateLinks;
use crate::ui::sections::section_solution::SectionSolution;
use cursive::Cursive;
use cursive::align::HAlign;
use cursive::align::HAlign::Center;
use cursive::style::Effect::Dim;
use cursive::style::Style;
use cursive::utils::markup::StyledString;
use cursive::view::{Resizable, Scrollable};
use cursive::views::{Button, DummyView, LinearLayout, TextView};

pub fn create_layout(siv: &mut Cursive) {
    let mut title = StyledString::new();
    title.append_plain("---   GOTHIC REMAKE LOCK SOLVER   ---   ");
    title.append_styled(format!("v{}", env!("CARGO_PKG_VERSION")), Style::from(Dim));

    let state = siv
        .with_user_data(|app_state: &mut AppState| app_state.clone())
        .unwrap();

    siv.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(DummyView.fixed_height(1))
            .child(TextView::new(title).h_align(Center))
            .child(DummyView.fixed_height(2))
            // main content
            .child(
                LinearLayout::horizontal()
                    .child(DummyView.fixed_width(1))
                    // input
                    .child(
                        LinearLayout::vertical()
                            // number of plates
                            .child(SectionPlateCount::create(&state))
                            .child(DummyView.fixed_height(2))
                            // pin positions
                            .child(SectionPinPositions::create(&state))
                            .child(DummyView.fixed_height(2))
                            // plate links
                            .child(SectionPlateLinks::create(&state))
                            // input layout
                            .fixed_width(56)
                            .scrollable(),
                    )
                    .child(DummyView.fixed_width(3))
                    // solution
                    .child(SectionSolution::create(&state).fixed_width(32))
                    .full_height(),
            )
            .child(DummyView.fixed_height(1))
            // bottom bar
            .child(
                LinearLayout::horizontal()
                    .child(DummyView.fixed_width(1))
                    .child(
                        LinearLayout::vertical()
                            .child(TextView::new("←↑↓→ : Navigate     Space : Select").style(Dim))
                            .child(
                                TextView::new("... or use your mouse, like a normie.").style(Dim),
                            )
                            .child(
                                TextView::new(
                                    "For mouse scrolling, put the curser ON the scroll bar.",
                                )
                                .style(Dim),
                            )
                            .full_width(),
                    )
                    .child(
                        LinearLayout::vertical()
                            .child(
                                LinearLayout::horizontal()
                                    .child(Button::new("Solve", |siv| SectionSolution::update(siv)))
                                    .child(DummyView.fixed_width(16))
                                    .child(Button::new("Quit", |siv| siv.quit())),
                            )
                            .child(DummyView.fixed_height(1))
                            .child(
                                TextView::new("CC-BY-NC-ND-4.0    Ⓒ Traxx 🦊")
                                    .h_align(HAlign::Right)
                                    .style(Dim),
                            ),
                    ),
            )
            .fixed_width(92)
            .full_height(),
    )
}
