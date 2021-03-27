use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext, WebGlProgram, WebGlUniformLocation};
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
    program: SurfaceProgram,
    mouse_pos: [f32; 2],
}

#[derive(Debug)]
struct SurfaceProgram {
    program: WebGlProgram,
    uniform_locations: SurfaceUniformLocations,
}

#[derive(Debug)]
struct SurfaceUniformLocations {
    time_location: Option<WebGlUniformLocation>,
    resolution_location: Option<WebGlUniformLocation>,
    mouse_location: Option<WebGlUniformLocation>,
}

impl Surface {
    fn setup_fs_program(gl: &WebGlRenderingContext, fs_code: &str) -> Result<SurfaceProgram, JsValue> {
        let program = graphics::create_program(gl, SAMPLE_VERTEX_SHADER, fs_code)?;
        gl.use_program(Some(&program));

        let buffer = gl.create_buffer().ok_or("failed to create web gl buffer")?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&SAMPLE_VERTICES);

            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        let pos_loc = gl.get_attrib_location(&program, "position");
        gl.vertex_attrib_pointer_with_i32(pos_loc as u32, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(pos_loc as u32);

        let uniform_locations = SurfaceUniformLocations {
            time_location: gl.get_uniform_location(&program, "iTime"),
            resolution_location: gl.get_uniform_location(&program, "iResolution"),
            mouse_location: gl.get_uniform_location(&program, "iMouse"),
        };
        let surface_program = SurfaceProgram {
            program,
            uniform_locations,
        };

        Ok(surface_program)
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
}

#[wasm_bindgen]
impl Surface {
    pub fn new(query_selector: &str, fs_code: &str) -> Result<Surface, JsValue> {
        let canvas = Surface::setup_canvas(query_selector)?;
        let gl = canvas.get_context("webgl")?
            .expect("web-gl must be enabled")
            .dyn_into::<WebGlRenderingContext>()?;
        let program = Surface::setup_fs_program(&gl, fs_code)?;

        Ok(Surface { canvas, gl, program, mouse_pos: [0.0, 0.0] })
    }

    pub fn update_mouse_pos(&mut self, x_pos: f32, y_pos: f32) {
        self.mouse_pos = [x_pos, y_pos];
    }

    pub fn render(&self, time: f32) -> Result<(), JsValue> {
        self.gl.uniform1f(self.program.uniform_locations.time_location.as_ref(), time);

        self.gl.uniform3fv_with_f32_array(
            self.program.uniform_locations.resolution_location.as_ref(),
            &[self.canvas.width() as f32, self.canvas.height() as f32, 1.0],
        );

        self.gl.uniform4fv_with_f32_array(
            self.program.uniform_locations.mouse_location.as_ref(),
            &[self.mouse_pos[0], self.mouse_pos[1], 0.0, 0.0],
        );

        // resizing canvas, firstly may resizing draw buffer width and height, then adjusting viewport
        graphics::resize_canvas(&self.canvas);
        self.gl.viewport(0, 0, self.canvas.width() as i32, self.canvas.height() as i32);

        // clear previously rendered image
        self.clear();

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
