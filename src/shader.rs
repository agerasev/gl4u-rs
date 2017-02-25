extern crate gl;

use std::{ffi, ptr};
use std::error::Error as StdError;
use std::io::{Read};
use std::fs::{File};

use gl::types::*;

use error::Error;

pub enum Type {
	Vertex,
	Fragment,
}

pub struct RawShader {
	pub id: GLuint,
	pub name: String,
}

impl RawShader {
	pub fn new(t: GLenum) -> Self {
		RawShader { id: unsafe { gl::CreateShader(t) }, name: String::new() }
	}
}

impl Drop for RawShader {
	fn drop(&mut self) {
		unsafe { gl::DeleteShader(self.id) };
	}
}

pub struct ShaderNew {
	pub raw: RawShader,
}

pub struct ShaderLoaded {
	pub raw: RawShader,
}

pub struct ShaderCompiled {
	pub raw: RawShader
}

pub type Shader = ShaderCompiled;

impl ShaderCompiled {
	pub fn new(t: Type) -> ShaderNew {
		let glt = match t {
			Type::Vertex => gl::VERTEX_SHADER,
			Type::Fragment => gl::FRAGMENT_SHADER,
		};
		ShaderNew { raw: RawShader::new(glt) }
	}

	pub fn name(&self) -> &str {
		&self.raw.name
	}

	pub fn id(&self) -> GLuint {
		self.raw.id
	}
}

impl ShaderNew {
	#[allow(unused_mut)]
	pub fn load_str(mut self, src: &str) -> ShaderLoaded {
		unsafe { gl::ShaderSource(self.raw.id, 1, &ffi::CString::new(src.as_bytes()).unwrap().as_ptr(), ptr::null()) };
		ShaderLoaded { raw: self.raw }
	}

	pub fn load_file(mut self, filename: &str) -> Result<ShaderLoaded, Error> {
		match File::open(filename) {
			Ok(f) => {
				self.raw.name = String::from(filename);
				let mut file = f;
				let mut content = String::new();
				file.read_to_string(&mut content).unwrap();
				Ok(self.load_str(content.as_str()))
			},
			Err(e) => Err(Error::new(String::new() + filename + ": " + e.description()))
		}
	}
}

impl ShaderLoaded {
	#[allow(unused_mut)]
	pub fn compile(mut self) -> Result<(ShaderCompiled, String), (Error, String)> {
		unsafe {
			let id = self.raw.id;
			gl::CompileShader(id);
			
			let mut status = gl::FALSE as GLint;
			gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);

			let mut len = 0;
			gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);

			let log = if len > 1 {
				let mut buf = Vec::<u8>::new();
				buf.resize((len-1) as usize, 0);
				gl::GetShaderInfoLog(id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
				self.raw.name.clone() + ": " + String::from_utf8(buf).unwrap().as_str()
			} else {
				String::new()
			};

			if status != (gl::TRUE as GLint) {
				Err((Error::new("Error compile shader `".to_string() + &self.raw.name + "`"), log))
			} else {
				Ok((ShaderCompiled { raw: self.raw }, log))
			}
		}
	}
}
