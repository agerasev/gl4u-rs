extern crate gl;

use std::collections::LinkedList;
use std::ptr;

use gl::types::*;

use shader::Shader;
use pass::Pass;

pub struct Program {
	pub id: GLuint,
	pub shaders: LinkedList<Shader>,
}

impl Program {
	pub fn new() -> Self {
		Program { id: unsafe { gl::CreateProgram() }, shaders: LinkedList::new() }
	}

	pub fn attach(mut self, shader: Shader) -> Self {
		unsafe { gl::AttachShader(self.id, shader.id); }
		self.shaders.push_back(shader);
		self
	}

	#[allow(unused_mut)]
	pub fn link(mut self) -> Result<Self, String> {
		unsafe {
			gl::LinkProgram(self.id);
			
			let mut status = gl::FALSE as GLint;
			gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut status);

			if status != (gl::TRUE as GLint) {
				let mut len: GLint = 0;
				gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
				if len > 0 {
					let mut buf = Vec::<u8>::new();
					buf.resize((len-1) as usize, 0);
					gl::GetProgramInfoLog(self.id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
					Err(String::from_utf8(buf).unwrap())
				} else {
					Err(String::new())
				}
			} else {
				Ok(self)
			}
		}
	}

	pub fn use_(&self) -> Result<Pass, String> {
		Ok(Pass::new(self.id))
	}
}

impl Drop for Program {
	fn drop(&mut self) {
		unsafe {
			let id = self.id;
			for ref shader in &self.shaders {
				gl::DetachShader(id, shader.id);
			}
			gl::DeleteProgram(self.id);
		}
	}
}
