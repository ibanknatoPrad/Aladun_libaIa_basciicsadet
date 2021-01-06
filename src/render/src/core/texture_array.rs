use web_sys::WebGl2RenderingContext;

use crate::image_fmt::FormatImageType;
use crate::WebGl2Context;

pub struct Texture2DArray {
    gl: WebGl2Context,

    pub textures: Vec<Texture2D>,
}

use super::{Texture2D, Texture2DBound};
use crate::JsValue;
impl Texture2DArray {
    pub fn create_empty(
        gl: &WebGl2Context,
        // The weight of the individual textures
        width: i32,
        // Their height
        height: i32,
        // How many texture slices it contains
        num_slices: i32,
        tex_params: &'static [(u32, u32)],
        // Texture format
        format: FormatImageType,
    ) -> Result<Texture2DArray, JsValue> {
        let mut textures = vec![];
        for _slice_idx in 0..num_slices {
            let texture = Texture2D::create_empty(gl, width, height, tex_params, format)?;
            textures.push(texture);
        }

        let gl = gl.clone();
        Ok(Texture2DArray { gl, textures })
    }

    pub fn bind_texture_slice(&self, idx_texture: i32) -> Texture2DBound {
        let texture = &self.textures[idx_texture as usize];
        texture.bind()
    }
}

use crate::shader::SendUniforms;
use crate::shader::ShaderBound;
impl SendUniforms for Texture2DArray {
    fn attach_uniforms<'a>(&self, shader: &'a ShaderBound<'a>) -> &'a ShaderBound<'a> {
        let num_tex = self.textures.len();

        let mut textures_bound = Vec::with_capacity(num_tex);
        for texture_idx in 0..num_tex {
            let texture_bound = self.bind_texture_slice(texture_idx as i32);
            textures_bound.push(texture_bound.get_idx_sampler());
        }

        shader
            .attach_uniform("tex[0]", &textures_bound.as_slice())
            .attach_uniform("num_tex", &(num_tex as i32));

        shader
    }
}
