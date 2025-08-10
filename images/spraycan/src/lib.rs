use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, EventTarget, console,
};
use std::f64::consts::PI;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

fn log(msg: &str) {
    console::log_1(&msg.into());
}

#[derive(Clone)]
struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    life: f64,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    log("🚀 Starting WASM module");

    let win = window().unwrap();
    let document = win.document().unwrap();
    let canvas = document.get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    log("✅ Canvas and context acquired");

    resize_canvas(&canvas);

    let particles = Rc::new(RefCell::new(Vec::new()));
    let particles_clone = particles.clone();
    let canvas_clone = canvas.clone();
    let ctx_clone = ctx.clone();

    let spray_origin_x = canvas.width() as f64 / 2.0;
    let spray_origin_y = canvas.height() as f64 / 2.0 - 100.0;

    let animate = Rc::new(RefCell::new(None));
    let animate_clone = animate.clone();

    *animate_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        update_particles(&ctx_clone, &canvas_clone, &mut particles_clone.borrow_mut(), spray_origin_x, spray_origin_y);
        request_animation_frame(animate.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(animate_clone.borrow().as_ref().unwrap());

    // Resize listener
    let closure = Closure::wrap(Box::new(move || {
        log("🔄 Window resized");

        let win = window().unwrap();
        let document = win.document().unwrap();
        let canvas = document.get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        resize_canvas(&canvas);
        draw_static_scene(&ctx, canvas.width() as f64, canvas.height() as f64);
    }) as Box<dyn FnMut()>);

    let et: &EventTarget = win.as_ref();
    et.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())?;
    closure.forget();

    log("📡 Resize listener set");

    draw_static_scene(&ctx, canvas.width() as f64, canvas.height() as f64);

    Ok(())
}

fn resize_canvas(canvas: &HtmlCanvasElement) {
    let rect = canvas
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .get_bounding_client_rect();

    canvas.set_width(rect.width() as u32);
    canvas.set_height(rect.height() as u32);

    log(&format!("📐 Canvas resized to {}x{}", rect.width(), rect.height()));
}

fn draw_static_scene(ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
    log("🎨 Drawing static scene");

    // Background
    ctx.set_fill_style(&JsValue::from_str("#ADD8E6"));
    ctx.fill_rect(0.0, 0.0, width, height);

    // Border
    ctx.set_stroke_style(&JsValue::from_str("#000000"));
    ctx.stroke_rect(0.0, 0.0, width, height);

    // Corner squares
    let size = 5.0;
    ctx.set_fill_style(&JsValue::from_str("#000000"));
    ctx.fill_rect(0.0, 0.0, size, size);
    ctx.fill_rect(width - size, 0.0, size, size);
    ctx.fill_rect(0.0, height - size, size, size);
    ctx.fill_rect(width - size, height - size, size, size);

    // Spray can
    let can_x = width / 2.0;
    let can_y = height / 2.0;
    draw_spray_can(ctx, can_x, can_y);

    // Spray cone
    let spray_origin_x = can_x;
    let spray_origin_y = can_y - 100.0;
    draw_cone(ctx, spray_origin_x, spray_origin_y, 0.0, PI / 8.0);
}

fn draw_spray_can(ctx: &CanvasRenderingContext2d, x: f64, y: f64) {
    ctx.set_fill_style(&JsValue::from_str("#808080"));
    ctx.fill_rect(x - 20.0, y - 80.0, 40.0, 80.0); // Can body
    ctx.set_fill_style(&JsValue::from_str("#000000"));
    ctx.fill_rect(x - 5.0, y - 90.0, 10.0, 10.0);  // Nozzle
}

fn draw_cone(ctx: &CanvasRenderingContext2d, x: f64, y: f64, angle: f64, spread: f64) {
    let length = 100.0;
    let left_angle = angle - spread / 2.0;
    let right_angle = angle + spread / 2.0;

    let x1 = x + length * left_angle.cos();
    let y1 = y + length * left_angle.sin();
    let x2 = x + length * right_angle.cos();
    let y2 = y + length * right_angle.sin();

    ctx.set_fill_style(&JsValue::from_str("rgba(255, 0, 0, 0.2)"));
    ctx.begin_path();
    ctx.move_to(x, y);
    ctx.line_to(x1, y1);
    ctx.line_to(x2, y2);
    ctx.close_path();
    ctx.fill();
}

fn update_particles(
    ctx: &CanvasRenderingContext2d,
    canvas: &HtmlCanvasElement,
    particles: &mut Vec<Particle>,
    origin_x: f64,
    origin_y: f64,
) {
    draw_static_scene(ctx, canvas.width() as f64, canvas.height() as f64);

    let mut rng = rand::thread_rng();

    // Spawn new particles
    for _ in 0..5 {
        let angle = rng.gen_range(-PI / 16.0..PI / 16.0);
        let speed = rng.gen_range(1.0..3.0);
        particles.push(Particle {
            x: origin_x,
            y: origin_y,
            vx: speed * angle.cos(),
            vy: speed * angle.sin(),
            life: 60.0,
        });
    }

    // Update and draw particles
    for p in particles.iter_mut() {
        p.x += p.vx;
        p.y += p.vy;
        p.life -= 1.0;

        let alpha = p.life / 60.0;
        let radius = alpha * 2.0;

        ctx.begin_path();
        ctx.arc(p.x, p.y, radius, 0.0, 2.0 * PI).unwrap();
        ctx.set_fill_style(&JsValue::from_str(&format!("rgba(255,0,0,{})", alpha)));
        ctx.fill();
    }

    particles.retain(|p| p.life > 0.0);
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window().unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}
