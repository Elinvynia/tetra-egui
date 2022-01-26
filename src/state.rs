use crate::egui::{handle_event, render_ui, EguiRenderer};
use egui::{CtxRef, InputState, Window};
use tetra::graphics::{clear, Color};
use tetra::time::get_delta_time;
use tetra::{Context, Event, State};

pub struct MainState {
    egui: CtxRef,
    egui_renderer: EguiRenderer,
    input: InputState,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> tetra::Result<Self> {
        let egui = CtxRef::default();
        let input = InputState::default();
        let egui_renderer = EguiRenderer::default();

        Ok(MainState {
            egui,
            egui_renderer,
            input,
        })
    }
}

impl State for MainState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        clear(ctx, Color::rgb(0.8, 0.8, 0.95));

        let new = match &mut self.input.raw.time {
            Some(prev) => Some(*prev + get_delta_time(ctx).as_secs_f64()),
            None => Some(get_delta_time(ctx).as_secs_f64()),
        };

        self.input.raw.time = new;
        self.egui.begin_frame(self.input.raw.take());
        self.input.raw.time = new;

        // egui UI code goes here!
        // you should most likely make a separate function
        // that gets called here
        Window::new("Hello World").show(&self.egui, |ui| {
            ui.label("I am inside of the window!");
        });
        // end

        let (_output, shapes) = self.egui.end_frame();
        render_ui(ctx, &mut self.egui, &mut self.egui_renderer, shapes);

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
