use crate::egui::{handle_event, render_ui};
use egui::{CtxRef, RawInput, Window};
use tetra::graphics::{clear, Color};
use tetra::time::get_delta_time;
use tetra::{Context, Event, State};

pub struct MainState {
    egui: CtxRef,
    input: RawInput,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> tetra::Result<Self> {
        let egui = CtxRef::default();
        let input = RawInput::default();

        Ok(MainState { egui, input })
    }
}

impl State for MainState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        clear(ctx, Color::rgb(0.8, 0.8, 0.95));

        let new = match &mut self.input.time {
            Some(prev) => Some(*prev + get_delta_time(ctx).as_secs_f64()),
            None => Some(get_delta_time(ctx).as_secs_f64()),
        };

        self.input.time = new;
        self.egui.begin_frame(self.input.take());
        self.input.time = new;

        // egui UI code goes here!
        Window::new("Hello World").show(&self.egui, |ui| {
            ui.label("I am inside of the window!");
        });
        // end

        let (_output, shapes) = self.egui.end_frame();
        render_ui(ctx, &mut self.egui, shapes);

        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> tetra::Result {
        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
        handle_event(ctx, &mut self.input, &event);
        Ok(())
    }
}
