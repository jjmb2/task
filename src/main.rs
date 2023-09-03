use cursive::views::{LinearLayout, ListView, Panel, TextView};
use cursive::{Cursive, CursiveExt};

/// Create a task panel with the given title.
fn task_panel(title: &str) -> Panel<ListView> {
    Panel::new(ListView::new()).title(title)
}

fn main() {
    let mut siv = Cursive::new();

    let bag = task_panel("Task Bag");
    let today = task_panel("Today");

    // both task views
    let tasks = LinearLayout::horizontal().child(bag).child(today);

    let screen = LinearLayout::vertical()
        .child(tasks)
        .child(Panel::new(TextView::new("q: quit ")));

    siv.add_layer(screen);
    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}
