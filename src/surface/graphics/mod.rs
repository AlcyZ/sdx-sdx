use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlProgram, HtmlCanvasElement};

mod program;

pub fn create_program(gl: &WebGlRenderingContext, vertex_shader: &str, fragment_shader: &str) -> Result<WebGlProgram, JsValue> {
    let v_shader = program::compile_vertex_shader(&gl, vertex_shader)?;
    let f_shader = program::compile_fragment_shader(&gl, fragment_shader)?;

    let program = program::link_program(&gl, &v_shader, &f_shader)?;

    Ok(program)
}

pub fn resize_canvas(canvas: &HtmlCanvasElement) {
    let display_width = canvas.client_width() as u32;
    let display_height = canvas.client_height() as u32;

    let canvas_width = canvas.width();
    let canvas_height = canvas.height();

    let need_resize = canvas_width != display_width || canvas_height != display_height;

    if need_resize {
        canvas.set_width(display_width);
        canvas.set_height(display_height);
    }
}
