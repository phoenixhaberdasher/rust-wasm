use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, EventTarget, console, Element,
};
use std::f64::consts::PI;
use rand::Rng;


fn log(msg: &str) {
    console::log_1(&msg.into());
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

    // Initial draw
    resize_and_draw(&canvas, &ctx);

    // Set up resize listener
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

        resize_and_draw(&canvas, &ctx);
    }) as Box<dyn FnMut()>);

    let et: &EventTarget = win.as_ref();
    et.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())?;
    closure.forget();

    log("📡 Resize listener set");

    Ok(())
}

// fn resize_and_draw(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) {
//     let rect = canvas
//         .dyn_ref::<HtmlElement>() // ✅ Correct cast to HtmlElement
//         .unwrap()
//         .get_bounding_client_rect();

//     canvas.set_width(rect.width() as u32);
//     canvas.set_height(rect.height() as u32);

//     log(&format!("📐 Canvas resized to {}x{}", rect.width(), rect.height()));

//     draw(ctx, rect.width(), rect.height());
// }

// fn resize_and_draw(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) {
//     let rect = canvas
//         .unchecked_ref::<Element>() // ✅ Correct cast to Element
//         .get_bounding_client_rect();

//     canvas.set_width(rect.width() as u32);
//     canvas.set_height(rect.height() as u32);

//     log(&format!("📐 Canvas resized to {}x{}", rect.width(), rect.height()));

//     draw(ctx, rect.width(), rect.height());
// }

fn resize_and_draw(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) {
    let html_element = canvas.dyn_ref::<web_sys::HtmlElement>().unwrap();
    let rect = html_element.get_bounding_client_rect();

    canvas.set_width(rect.width() as u32);
    canvas.set_height(rect.height() as u32);

    log(&format!("📐 Canvas resized to {}x{}", rect.width(), rect.height()));

    draw(ctx, rect.width(), rect.height());
}

fn draw(ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
    log("🎨 Starting draw function");

    // Background
    ctx.set_fill_style(&JsValue::from_str("#ADD8E6"));
    ctx.fill_rect(0.0, 0.0, width, height);
    log("🟦 Background filled");

    // Border
    ctx.set_stroke_style(&JsValue::from_str("#000000"));
    ctx.stroke_rect(0.0, 0.0, width, height);
    log("⬛ Border drawn");

    // Corner squares
    let size = 5.0;
    ctx.set_fill_style(&JsValue::from_str("#000000"));
    ctx.fill_rect(0.0, 0.0, size, size);
    ctx.fill_rect(width - size, 0.0, size, size);
    ctx.fill_rect(0.0, height - size, size, size);
    ctx.fill_rect(width - size, height - size, size, size);
    log("🔲 Corner squares drawn");

    // Spray can center
    let can_x = width / 2.0;
    let can_y = height / 2.0;
    log(&format!("📍 Spray can center at ({}, {})", can_x, can_y));

    // Spray origin — slightly above the can
    let spray_origin_x = can_x;
    let spray_origin_y = can_y - 100.0;
    log(&format!("🎯 Spray origin at ({}, {})", spray_origin_x, spray_origin_y));

    // Spray origin dot
    ctx.set_fill_style(&JsValue::from_str("#FF0000"));
    ctx.begin_path();
    ctx.arc(spray_origin_x, spray_origin_y, 4.0, 0.0, 2.0 * PI).unwrap();
    ctx.fill();
    log("🔴 Spray origin dot drawn");

    draw_spray_can(ctx, can_x, can_y);
    log("🧴 Spray can drawn");

    draw_cone(ctx, spray_origin_x, spray_origin_y, 0.0, PI / 8.0);
    log("📐 Spray cone drawn");

    draw_particles(ctx, spray_origin_x, spray_origin_y, PI / 8.0, 150, 0.0);
    log("✨ Spray particles drawn");

    log("✅ Draw function complete");
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

fn draw_particles(ctx: &CanvasRenderingContext2d, x: f64, y: f64, spread: f64, count: usize, angle: f64) {
    let mut rng = rand::thread_rng();

    ctx.set_fill_style(&JsValue::from_str("#FF0000"));

    for _ in 0..count {
        let a = angle - spread / 2.0 + rng.gen::<f64>() * spread;
        let r = rng.gen::<f64>() * 100.0;
        let px = x + r * a.cos();
        let py = y + r * a.sin();
        ctx.fill_rect(px, py, 2.0, 2.0);
    }
}
