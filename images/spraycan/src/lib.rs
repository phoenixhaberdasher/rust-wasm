use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, window};
use js_sys::Math;
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    resize_canvas(&canvas);
    let particles = Rc::new(RefCell::new(Vec::new()));
    let particles_clone = particles.clone();

    let closure = Rc::new(RefCell::new(None));
    let closure_clone = closure.clone();

    *closure_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        update_particles(&ctx, &mut particles_clone.borrow_mut());
        request_animation_frame(closure.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(closure_clone.borrow().as_ref().unwrap());

    // Simulate spraying every 100ms
    let spray_closure = Closure::wrap(Box::new(move || {
        spawn_particles(&canvas, &mut particles.borrow_mut());
    }) as Box<dyn FnMut()>);

    window.set_interval_with_callback_and_timeout_and_arguments_0(
        spray_closure.as_ref().unchecked_ref(),
        100,
    )?;

    spray_closure.forget(); // Keep it alive

    Ok(())
}

fn resize_canvas(canvas: &HtmlCanvasElement) {
    let html_element = canvas.dyn_ref::<HtmlElement>().unwrap();
    let rect = html_element.get_bounding_client_rect();
    canvas.set_width(rect.width() as u32);
    canvas.set_height(rect.height() as u32);
}

struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    life: f64,
}

fn spawn_particles(canvas: &HtmlCanvasElement, particles: &mut Vec<Particle>) {
    let x = canvas.width() as f64 / 2.0;
    let y = canvas.height() as f64 / 2.0;

    for _ in 0..20 {
        let angle = Math::random() * std::f64::consts::PI * 2.0;
        let speed = Math::random() * 2.0;
        particles.push(Particle {
            x,
            y,
            vx: angle.cos() * speed,
            vy: angle.sin() * speed,
            life: 60.0,
        });
    }
}

fn update_particles(ctx: &CanvasRenderingContext2d, particles: &mut Vec<Particle>) {
    ctx.clear_rect(0.0, 0.0, ctx.canvas().unwrap().width() as f64, ctx.canvas().unwrap().height() as f64);

    for p in particles.iter_mut() {
        p.x += p.vx;
        p.y += p.vy;
        p.life -= 1.0;

        ctx.begin_path();
        ctx.arc(p.x, p.y, 1.5, 0.0, std::f64::consts::PI * 2.0).unwrap();
        ctx.set_fill_style(&JsValue::from_str("rgba(0,0,0,0.5)"));
        ctx.fill();
    }

    particles.retain(|p| p.life > 0.0);
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window().unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}
