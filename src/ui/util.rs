use cursive::views::Dialog;
use cursive::View;


pub fn wrap_section(section: impl View, title: &str) -> Dialog {
    Dialog::around(section).title(title).padding_top(1)
}
