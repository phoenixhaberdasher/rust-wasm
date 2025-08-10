use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window, Document};
use js_sys::Math;
use std::f64::consts::PI;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document: Document = window.document().unwrap();
    let canvas: HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    // Animation loop
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        draw_spray(&context);
        let _ = window
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref());
    }) as Box<dyn FnMut()>));

    window
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}

use std::cell::RefCell;
use std::rc::Rc;

fn draw_spray(ctx: &CanvasRenderingContext2d) {
    let width = ctx.canvas().unwrap().width() as f64;
    let height = ctx.canvas().unwrap().height() as f64;

    ctx.set_fill_style(&JsValue::from_str("white"));
    ctx.fill_rect(0.0, 0.0, width, height);

    let nozzle_x = width / 2.0;
    let nozzle_y = height - 50.0;
    let spread = 100.0;
    let particle_count = 300;

    draw_cone(ctx, nozzle_x, nozzle_y, -PI / 2.0, spread);
    draw_particles(ctx, nozzle_x, nozzle_y, spread, particle_count);
}

fn draw_cone(ctx: &CanvasRenderingContext2d, x: f64, y: f64, angle: f64, spread: f64) {
    let cone_angle = PI / 6.0;
    ctx.begin_path();
    ctx.move_to(x, y);
    ctx.line_to(
        x + spread * (angle - cone_angle).cos(),
        y + spread * (angle - cone_angle).sin(),
    );
    ctx.line_to(
        x + spread * (angle + cone_angle).cos(),
        y + spread * (angle + cone_angle).sin(),
    );
    ctx.close_path();
    ctx.set_fill_style(&JsValue::from_str("lightgray"));
    ctx.fill();
}

fn draw_particles(ctx: &CanvasRenderingContext2d, x: f64, y: f64, spread: f64, count: usize) {
    let cone_angle = PI / 6.0;
    for _ in 0..count {
        let angle = random_range(-cone_angle, cone_angle) - PI / 2.0;
        let distance = random_range(0.0, spread);
        let px = x + distance * angle.cos();
        let py = y + distance * angle.sin();
        ctx.set_fill_style(&JsValue::from_str(random_color()));
        ctx.fill_rect(px, py, 2.0, 2.0);
    }
}

fn random_range(min: f64, max: f64) -> f64 {
    Math::random() * (max - min) + min
}

fn random_color() -> &'static str {
    let colors = ["#FF0000", "#00FF00", "#0000FF", "#FFFF00", "#FF00FF"];
    let index = (Math::floor(Math::random() * colors.len() as f64)) as usize;
    colors[index]
}

