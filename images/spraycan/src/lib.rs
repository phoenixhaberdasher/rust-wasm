use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use std::f64::consts::PI;

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
    // Background
    ctx.set_fill_style(&"#ADD8E6".into()); // Light blue
    ctx.fill_rect(0.0, 0.0, width, height);

    // Border
    ctx.set_stroke_style(&"#000000".into()); // Black
    ctx.stroke_rect(0.0, 0.0, width, height);

    // Centered can
    let can_x = width / 2.0;
    let can_y = height / 2.0 + 40.0; // Slightly lower to fit spray

    let spray_origin_x = can_x;
    let spray_origin_y = can_y - 80.0 - 10.0 - 5.0;

    draw_spray_can(ctx, can_x, can_y);
    draw_cone(ctx, spray_origin_x, spray_origin_y, -PI / 6.0, PI / 8.0);
    draw_particles(ctx, spray_origin_x, spray_origin_y, PI / 8.0, 150, -PI / 6.0);
}

fn draw_spray_can(ctx: &CanvasRenderingContext2d, x: f64, y: f64) {
    let body_width = 30.0;
    let body_height = 80.0;
    let neck_height = 10.0;
    let radius = body_width / 2.0;

    // Bent body shape
    ctx.begin_path();
    ctx.move_to(x - radius, y);
    ctx.bezier_curve_to(x - radius - 5.0, y - body_height / 2.0, x - radius + 5.0, y - body_height, x - radius, y - body_height);
    ctx.line_to(x + radius, y - body_height);
    ctx.bezier_curve_to(x + radius + 5.0, y - body_height / 2.0, x + radius - 5.0, y, x + radius, y);
    ctx.close_path();
    ctx.set_fill_style(&"#800080".into()); // Purple body
    ctx.fill();

    // Rounded top
    ctx.begin_path();
    ctx.arc(x, y - body_height, radius, PI, 0.0).unwrap();
    ctx.set_fill_style(&"#9932CC".into()); // Lighter purple
    ctx.fill();

    // Neck
    ctx.begin_path();
    ctx.move_to(x - radius / 2.0, y - body_height);
    ctx.line_to(x - radius / 2.0, y - body_height - neck_height);
    ctx.line_to(x + radius / 2.0, y - body_height - neck_height);
    ctx.line_to(x + radius / 2.0, y - body_height);
    ctx.close_path();
    ctx.set_fill_style(&"#BA55D3".into()); // Even lighter purple
    ctx.fill();

    // Nozzle
    ctx.begin_path();
    ctx.arc(x, y - body_height - neck_height - 5.0, 4.0, 0.0, 2.0 * PI).unwrap();
    ctx.set_fill_style(&"#D8BFD8".into()); // Pale purple
    ctx.fill();
}

fn draw_cone(ctx: &CanvasRenderingContext2d, x: f64, y: f64, angle: f64, spread: f64) {
    let length = 100.0;
    let left_angle = angle - spread / 2.0;
    let right_angle = angle + spread / 2.0;

    let x1 = x + length * left_angle.cos();
    let y1 = y + length * left_angle.sin();
    let x2 = x + length * right_angle.cos();
    let y2 = y + length * right_angle.sin();

    ctx.begin_path();
    ctx.move_to(x, y);
    ctx.line_to(x1, y1);
    ctx.line_to(x2, y2);
    ctx.close_path();
    ctx.set_fill_style(&"rgba(128,0,128,0.2)".into()); // Transparent purple
    ctx.fill();
}

fn draw_particles(ctx: &CanvasRenderingContext2d, x: f64, y: f64, spread: f64, count: usize, angle: f64) {
    let max_distance = 100.0;
    for _ in 0..count {
        let rand_angle = angle + (js_sys::Math::random() - 0.5) * spread;
        let distance = js_sys::Math::random() * max_distance;
        let px = x + distance * rand_angle.cos();
        let py = y + distance * rand_angle.sin();

        ctx.begin_path();
        ctx.arc(px, py, 1.5, 0.0, 2.0 * PI).unwrap(); // Smaller particles
        ctx.set_fill_style(&"#DA70D6".into()); // Orchid purple
        ctx.fill();
    }
}
