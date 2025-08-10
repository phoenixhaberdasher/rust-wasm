use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window, Document};
use js_sys::Math;
use std::f64::consts::PI;
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let window_clone = window.clone();
    let document: Document = window.document().unwrap();
    let canvas: HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        draw_spray(&context);
        let _ = window_clone
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref());
    }) as Box<dyn FnMut()>));

    window
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}

fn draw_spray(ctx: &CanvasRenderingContext2d) {
    let width = ctx.canvas().unwrap().width() as f64;
    let height = ctx.canvas().unwrap().height() as f64;

    ctx.set_fill_style(&"white".into());
    ctx.fill_rect(0.0, 0.0, width, height);

    // Spray angle and spread
    let spray_angle = 0.0; // Rightward (3 o'clock)
    let spread = 60.0;
    let particle_count = 200;

    // Spray can position (left side, vertically centered)
    let can_x = width * 0.25;
    let can_y = height * 0.5;

    draw_spray_can(ctx, can_x, can_y);
    draw_cone(ctx, can_x, can_y, spray_angle, spread);
    draw_particles(ctx, can_x, can_y, spread, particle_count, spray_angle);
}

fn draw_spray_can(ctx: &CanvasRenderingContext2d, x: f64, y: f64) {
    let body_width = 20.0;
    let body_height = 60.0;
    let bend_offset = 10.0;

    // Body with bend
    ctx.begin_path();
    ctx.move_to(x, y);
    ctx.line_to(x, y - body_height);
    ctx.line_to(x + body_width, y - body_height + bend_offset);
    ctx.line_to(x + body_width, y + bend_offset);
    ctx.close_path();
    ctx.set_fill_style(&"#333".into());
    ctx.fill();

    // Nozzle
    ctx.begin_path();
    ctx.arc(x + body_width + 5.0, y - body_height / 2.0, 5.0, 0.0, 2.0 * PI).unwrap();
    ctx.set_fill_style(&"#666".into());
    ctx.fill();
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
    ctx.set_fill_style(&"lightgray".into());
    ctx.fill();
}

fn draw_particles(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    spread: f64,
    count: usize,
    angle: f64,
) {
    let cone_angle = PI / 6.0;
    for _ in 0..count {
        let offset = random_range(-cone_angle, cone_angle);
        let distance = random_range(0.0, spread);
        let px = x + distance * (angle + offset).cos();
        let py = y + distance * (angle + offset).sin();
        ctx.set_fill_style(&random_color().into());
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
