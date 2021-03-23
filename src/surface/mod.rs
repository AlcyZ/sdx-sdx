use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext, WebGlProgram};
use crate::util;

mod graphics;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

const SAMPLE_VERTEX_SHADER: &str = r#"
attribute vec4 position;
void main() {
    gl_Position = position;
}
"#;


// renders two triangle sending the whole surface pixels to the fragment shader
const SAMPLE_VERTICES: [f32; 12] = [
    -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, // first triangle
    -1.0, -1.0, 1.0, -1.0, 1.0, 1.0 // second triangle
];

#[wasm_bindgen]
#[derive(Debug)]
pub struct Surface {
    canvas: HtmlCanvasElement,
    gl: WebGlRenderingContext,
    program: Option<WebGlProgram>,
}

#[wasm_bindgen]
impl Surface {
    pub fn new(query_selector: &str) -> Result<Surface, JsValue> {
        let canvas = setup_canvas(query_selector)?;
        let gl = canvas.get_context("webgl")?
            .expect("web-gl must be enabled")
            .dyn_into::<WebGlRenderingContext>()?;

        Ok(Surface { canvas, gl, program: None })
    }

    pub fn setup_fs_program(&mut self, fs_code: &str) -> Result<(), JsValue> {
        self.program = Some(graphics::create_program(&self.gl, SAMPLE_VERTEX_SHADER, fs_code)?);
        self.gl.use_program(Some(&self.program.as_ref().unwrap()));

        let buffer = self.gl.create_buffer().ok_or("failed to create web gl buffer")?;
        self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&SAMPLE_VERTICES);

            self.gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        let pos_loc = self.gl.get_attrib_location(&self.program.as_ref().unwrap(), "position");
        self.gl.vertex_attrib_pointer_with_i32(pos_loc as u32, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(pos_loc as u32);

        Ok(())
    }

    pub fn render(&self, time: f32) -> Result<(), JsValue> {
        let time_loc = self.gl.get_uniform_location(&self.program.as_ref().unwrap(), "iTime");
        self.gl.uniform1f(time_loc.as_ref(), time);

        let resolution_loc = self.gl.get_uniform_location(&self.program.as_ref().unwrap(), "iResolution");
        self.gl.uniform3fv_with_f32_array(resolution_loc.as_ref(), &[self.canvas.width() as f32, self.canvas.height() as f32, 1.0]);

        // resizing canvas, firstly may resizing draw buffer width and height, then adjusting viewport
        graphics::resize_canvas(&self.canvas);
        self.gl.viewport(0, 0, self.canvas.width() as i32, self.canvas.height() as i32);

        // clear previously rendered image
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        // render current program
        self.gl.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (SAMPLE_VERTICES.len() / 2) as i32,
        );

        Ok(())
    }

    pub fn clear(&self) {
        // clear previously rendered image
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }
}

fn setup_canvas(query_selector: &str) -> Result<HtmlCanvasElement, JsValue> {
    let canvas = util::document()
        .query_selector(query_selector)?
        .ok_or_else(|| JsValue::from(
            &format!("could not find query selector '{}'", query_selector)
        ))?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    Ok(canvas)
}
