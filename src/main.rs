extern crate nannou;
use nannou::{
    prelude::*,
    rand::{Rng, RngCore, SeedableRng, rngs::StdRng},
};

fn main() {
    // Set nannou random seed
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    cube_size: f32,
    border_size: f32,
    seed: u64,
    angle_noise: f32,
    translation_limit: f32,
}

fn model(_app: &App) -> Model {
    Model {
        cube_size: 50.0,
        border_size: 30.0,
        seed: 42,
        angle_noise: 0.5,
        translation_limit: 15.0,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn draw_row(draw: &Draw, area: &Rect, model: &Model, noise_limit: f32, rng: &mut impl Rng) {
    // Calculate the number of cubes that can fit in the area
    let num_cubes = ((area.w() - model.border_size * 2.0) / model.cube_size).floor() as i32;

    for i in 0..num_cubes {
        // Determine angle to rotate. Use model's angle_noise param * noise_limit
        // to create a range of angles.
        let random_angle = if noise_limit > 0.0 {
            rng.gen_range((-model.angle_noise * noise_limit)..(model.angle_noise * noise_limit))
        } else {
            0.0
        };

        // Shift the square up/down/left/right randomly
        let x_shift = if noise_limit > 0.0 {
            rng.gen_range(
                (-model.translation_limit * noise_limit)..(model.translation_limit * noise_limit),
            )
        } else {
            0.0
        };
        let y_shift = if noise_limit > 0.0 {
            rng.gen_range(
                (-model.translation_limit * noise_limit)..(model.translation_limit * noise_limit),
            )
        } else {
            0.0
        };

        // Create rect
        let rect = Rect::from_w_h(model.cube_size, model.cube_size)
            .top_left_of(*area)
            .shift_x(i as f32 * model.cube_size)
            .shift_x(x_shift)
            .shift_y(y_shift);

        // Draw rect
        draw.rect()
            .wh(rect.wh())
            .xy(rect.xy())
            .stroke(BLACK)
            .stroke_weight(1.0)
            .rotate(random_angle);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);
    let window = app.window_rect().pad(model.border_size);

    // Create rng
    let mut main_rng = StdRng::seed_from_u64(model.seed);

    // Prepare to draw.
    let draw = app.draw();

    // Calculate the number of rows that can fit in the area
    let num_rows = ((window.h() - model.border_size * 2.0) / model.cube_size).floor() as i32;
    for i in 0..num_rows {
        // Gen rng for this row
        let mut rng = StdRng::seed_from_u64(main_rng.next_u64());

        draw_row(
            &draw,
            &window.shift_y(-i as f32 * model.cube_size),
            model,
            i as f32 / num_rows as f32,
            &mut rng,
        );
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
