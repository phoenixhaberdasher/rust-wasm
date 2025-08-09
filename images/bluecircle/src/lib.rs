use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    // Set canvas size to match its display size
    let width = canvas.client_width() as u32;
    let height = canvas.client_height() as u32;
    canvas.set_width(width);
    canvas.set_height(height);

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Calculate center and radius
    let center_x = width as f64 / 2.0;
    let center_y = height as f64 / 2.0;
    let radius = center_x.min(center_y) - 5.0;

    // Draw centered circle
    context.begin_path();
    context.arc(center_x, center_y, radius, 0.0, std::f64::consts::PI * 2.0)?;
    context.set_fill_style(&JsValue::from_str("blue"));
    context.fill();

    Ok(())
}
