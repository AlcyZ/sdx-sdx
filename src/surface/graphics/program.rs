use wasm_bindgen::prelude::*;
use web_sys::{WebGlShader, WebGlRenderingContext, WebGlProgram};

pub fn compile_vertex_shader(gl: &web_sys::WebGlRenderingContext, source: &str) -> Result<WebGlShader, JsValue> {
    compile_shader(&gl, WebGlRenderingContext::VERTEX_SHADER, source)
}

pub fn compile_fragment_shader(gl: &web_sys::WebGlRenderingContext, source: &str) -> Result<WebGlShader, JsValue> {
    compile_shader(&gl, WebGlRenderingContext::FRAGMENT_SHADER, source)
}

pub fn link_program(
    gl: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, JsValue> {
    let program = gl
        .create_program()
        .ok_or_else(|| JsValue::from("Unable to create shader object"))?;

    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")).into())
    }
}

fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, JsValue> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| JsValue::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")).into())
    }
}
