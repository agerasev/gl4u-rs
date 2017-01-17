extern crate gl;

use std::{ffi, ptr};
use std::error::{Error};
use std::io::{Read};
use std::fs::{File};

use gl::types::*;

pub enum Type {
	Vertex,
	Fragment,
}

pub struct Shader {
	pub id: GLuint,
	pub name: String,
}

impl Shader {
	pub fn new(t: Type) -> Self {
		let glt = match t {
			Type::Vertex => gl::VERTEX_SHADER,
			Type::Fragment => gl::FRAGMENT_SHADER,
		};
		Shader { id: unsafe { gl::CreateShader(glt) }, name: String::new() }
	}

	#[allow(unused_mut)]
	pub fn load_str(mut self, src: &str) -> Self {
		unsafe { gl::ShaderSource(self.id, 1, &ffi::CString::new(src.as_bytes()).unwrap().as_ptr(), ptr::null()) };
		self
	}

	pub fn load_file(mut self, filename: &str) -> Result<Self, String> {
		match File::open(filename) {
			Ok(f) => {
				self.name = String::from(filename);
				let mut file = f;
				let mut content = String::new();
				file.read_to_string(&mut content).unwrap();
				Ok(self.load_str(content.as_str()))
			},
			Err(e) => Err(String::new() + filename + ": " + e.description())
		}
	}

	#[allow(unused_mut)]
	pub fn compile(mut self) -> Result<(Self, String), String> {
		unsafe {
			gl::CompileShader(self.id);
			
			let mut status = gl::FALSE as GLint;
			gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut status);

			let mut len = 0;
			gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut len);

			let log = if len > 0 {
				let mut buf = Vec::<u8>::new();
				buf.resize((len-1) as usize, 0);
				gl::GetShaderInfoLog(self.id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
				self.name.clone() + ": " + String::from_utf8(buf).unwrap().as_str()
			} else {
				String::new()
			};

			if status != (gl::TRUE as GLint) {
				Err(log)
			} else {
				Ok((self, log))
			}
		}
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe { gl::DeleteShader(self.id) };
	}
}