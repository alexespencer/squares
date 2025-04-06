extern crate nannou;
use std::ops::Mul;

use nannou::{
    prelude::*,
    rand::{
        Rng, RngCore, SeedableRng,
        distributions::uniform::{SampleRange, SampleUniform},
        rngs::StdRng,
    },
};
use nannou_egui::{Egui, egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    /// How large the cubes are.
    cube_size: f32,
    /// How large the border is, padded top/bottom/left/right of the window.
    border_size: f32,
    /// Scaling factor for rotation.
    angle_noise: f32,
    /// Scaling factor for translation.
    translation_noise: f32,
}

struct Model {
    seed: u64,
    settings: Settings,
    egui: Egui,
}

fn model(app: &App) -> Model {
    // Create window
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    Model {
        seed: 42,
        settings: Settings {
            cube_size: 50.0,
            border_size: 30.0,
            angle_noise: 0.5,
            translation_noise: 15.0,
        },
        egui,
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Cube Size:");
        ui.add(egui::Slider::new(&mut settings.cube_size, 10.0..=200.0));

        ui.label("Angle noise:");
        ui.add(egui::Slider::new(
            &mut settings.angle_noise,
            0.01..=(PI / 2.0),
        ));

        ui.label("Translation noise:");
        ui.add(egui::Slider::new(
            &mut settings.translation_noise,
            0.01..=25.0,
        ));
    });
}

fn scaled_random_range<T, R>(rng: &mut impl Rng, scale: T, range: R) -> T
where
    T: SampleUniform + Zero + Mul<Output = T>,
    R: SampleRange<T>,
{
    if !scale.is_zero() && !range.is_empty() {
        scale * rng.gen_range(range)
    } else {
        T::zero()
    }
}

fn draw_row(draw: &Draw, area: &Rect, settings: &Settings, noise_limit: f32, rng: &mut impl Rng) {
    // Calculate the number of cubes that can fit in the area
    let num_cubes = ((area.w() - settings.border_size * 2.0) / settings.cube_size).floor() as i32;

    for i in 0..num_cubes {
        // Determine angle to rotate. Use model's angle_noise param * noise_limit
        // to create a range of angles.
        let random_angle = scaled_random_range(
            rng,
            noise_limit,
            -settings.angle_noise..settings.angle_noise,
        );

        // Shift the square up/down/left/right randomly
        let x_shift = scaled_random_range(
            rng,
            noise_limit,
            -settings.translation_noise..settings.translation_noise,
        );
        let y_shift = scaled_random_range(
            rng,
            noise_limit,
            -settings.translation_noise..settings.translation_noise,
        );

        // Create rect
        let rect = Rect::from_w_h(settings.cube_size, settings.cube_size)
            .top_left_of(*area)
            .shift_x(i as f32 * settings.cube_size)
            .shift_x(x_shift)
            .shift_y(y_shift);

        // Draw rect
        draw.rect()
            .wh(rect.wh())
            .xy(rect.xy())
            .stroke(BLACK)
            .stroke_weight(1.5)
            .rotate(random_angle);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);
    let window = app.window_rect().pad(model.settings.border_size);

    // Create rng
    let mut main_rng = StdRng::seed_from_u64(model.seed);

    // Prepare to draw.
    let draw = app.draw();

    // Calculate the number of rows that can fit in the area
    let num_rows =
        ((window.h() - model.settings.border_size * 2.0) / model.settings.cube_size).floor() as i32;
    for i in 0..num_rows {
        // Gen rng for this row
        let mut rng = StdRng::seed_from_u64(main_rng.next_u64());

        draw_row(
            &draw,
            &window.shift_y(-i as f32 * model.settings.cube_size),
            &model.settings,
            i as f32 / num_rows as f32,
            &mut rng,
        );
    }

    // Write to the window frame and draw the egui menu.
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
