use crate::state::MainState;
use tetra::ContextBuilder;

mod egui;
mod state;

fn main() -> tetra::Result {
    ContextBuilder::new("Example", 1280, 720)
        .show_mouse(true)
        .build()?
        .run(MainState::new)
}
