use crate::common::solver::{Move, solve};
use crate::data::AppState;
use crate::ui;
use crate::ui::sections::section::Section;
use cursive::Cursive;
use cursive::view::{Nameable, Resizable, Scrollable};
use cursive::views::{Checkbox, Dialog, DummyView, LinearLayout, TextView};

pub struct SectionSolution {}
impl Section for SectionSolution {
    const NAME: &'static str = "section_solution";
    fn create(app_state: &AppState) -> Dialog {
        let mut chk_group_steps = Checkbox::new().on_change(|siv, checked| {
            siv.with_user_data(|app_state: &mut AppState| {
                app_state.group_steps = checked;
            });
            Self::update_display(siv);
        });

        if app_state.group_steps {
            chk_group_steps.check();
        }

        ui::util::wrap_section(
            LinearLayout::vertical()
                .child(
                    LinearLayout::horizontal()
                        .child(chk_group_steps)
                        .child(TextView::new(" Group identical")),
                )
                .child(DummyView.fixed_height(1))
                .child(TextView::empty().with_name(Self::NAME).scrollable()),
            "Solution",
        )
    }

    fn update(siv: &mut Cursive) {
        // get lock data
        let lock_data = siv
            .with_user_data(|app_state: &mut AppState| app_state.lock.clone())
            .unwrap();

        let solution = solve(&lock_data);
        siv.with_user_data(|app_state: &mut AppState| {
            app_state.solution = solution;
        });

        Self::update_display(siv);
    }
}

impl SectionSolution {
    fn update_display(siv: &mut Cursive) {
        let (solution, group_steps) = siv
            .with_user_data(|app_state: &mut AppState| {
                (app_state.solution.clone(), app_state.group_steps)
            })
            .unwrap();

        let mut result_str = String::new();

        if let Some(mvs) = solution {
            result_str.push_str(&format!("Solution: {} moves\n\n", mvs.len()));

            // group moves into "streaks" of consecutive moves
            let mut streak_start_step = 1;
            let mut streak_count = 0;
            let mut current: Option<Move> = None;

            // This actually evaluates the "last" ("current") move (the one before the current loop iteration), by comparing
            // the "last" move to the "new" one. It only writes the new one to be "current" at the end of the iteration.
            for mv in mvs {
                if !group_steps {
                    Self::append_move(&mv, &mut result_str, streak_start_step, 1);
                    streak_start_step += 1;
                    continue;
                }

                match &current {
                    // Some picks only on first move, if guard (as it is not within a `=> {}` block) additionally
                    // makes this branch only execute if the move is the same as the last.
                    Some(cur) if cur.plate == mv.plate && cur.direction == mv.direction => {
                        streak_count += 1;
                    }
                    // first run or guard failed
                    _ => {
                        // `take()` takes the value out of an option; leaves `None` behind
                        // runs only after first iteration = "current" is already set
                        if let Some(cur) = current.take() {
                            Self::append_move(
                                &cur,
                                &mut result_str,
                                streak_start_step,
                                streak_count,
                            );
                        }

                        streak_start_step += streak_count;
                        streak_count = 1;
                        current = Some(mv);
                    }
                }
            }

            // last move is not evaluated in loop, append it
            if let Some(cur) = current {
                Self::append_move(&cur, &mut result_str, streak_start_step, streak_count);
            }

        // no solution found
        } else {
            result_str = String::from("No solution.");
        }

        // rewrite and populate solution text
        siv.call_on_name(Self::NAME, |section: &mut TextView| {
            section.set_content(result_str);
        });
    }

    fn append_move(mv: &Move, result_str: &mut String, streak_start_step: i32, streak_count: i32) {
        result_str.push_str(&format!(
            "#{:>2}   Plate {}: {}",
            streak_start_step,
            mv.plate + 1,
            mv.direction.to_str()
        ));

        if streak_count > 1 {
            result_str.push_str(&format!("   x{}", streak_count));
        }

        if Self::lines_since_last_empty(result_str) == 4 {
            result_str.push('\n');
        }
        result_str.push('\n');
    }
    fn lines_since_last_empty(input: &str) -> usize {
        let mut count = 0;

        for line in input.lines().rev() {
            if line.is_empty() {
                break;
            }
            count += 1;
        }

        count
    }
}
