extern crate gl;

use std::collections::LinkedList;
use std::ptr;
use std::rc::Rc;

use gl::types::*;

use shader::Shader;
use pass::Pass;
use error::Error;

pub struct RawProgram {
	pub id: GLuint,
	pub name: String,
	pub shaders: LinkedList<Rc<Shader>>,
}

impl RawProgram {
	pub fn new() -> Self {
		RawProgram { id: unsafe { gl::CreateProgram() }, name: String::new(), shaders: LinkedList::new() }
	}

	pub fn attach_shader(&mut self, shrc: Rc<Shader>) {
		unsafe { gl::AttachShader(self.id, shrc.id()); }
		self.shaders.push_back(shrc);
	}
}

impl Drop for RawProgram {
	fn drop(&mut self) {
		let id = self.id;
		for ref shader in &self.shaders {
			unsafe { gl::DetachShader(id, shader.id()); }
		}
		unsafe { gl::DeleteProgram(self.id); }
	}
}

pub struct ProgramNew {
	pub raw: RawProgram,
}

pub struct ProgramLinked {
	pub raw: RawProgram,
}

pub type Program = ProgramLinked;

impl ProgramLinked {
	pub fn new() -> ProgramNew {
		ProgramNew { raw: RawProgram::new() }
	}

	pub fn use_(&self) -> Result<Pass, Error> {
		Ok(Pass::new(self.id()))
	}

	pub fn name(&self) -> &str {
		&self.raw.name
	}

	pub fn set_name(&mut self, name: &str) {
		self.raw.name = name.to_string()
	}

	pub fn id(&self) -> GLuint {
		self.raw.id
	}
}

impl ProgramNew {
	pub fn attach(mut self, shader: Shader) -> Self {
		self.raw.attach_shader(Rc::new(shader));
		self
	}

	pub fn attach_rc(mut self, shader_rc: Rc<Shader>) -> Self {
		self.raw.attach_shader(shader_rc);
		self
	}

	#[allow(unused_mut)]
	pub fn link(mut self) -> Result<ProgramLinked, Error> {
		unsafe {
			let id = self.raw.id;
			gl::LinkProgram(id);
			
			let mut status = gl::FALSE as GLint;
			gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);

			if status != (gl::TRUE as GLint) {
				let mut len: GLint = 0;
				gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
				if len > 0 {
					let mut buf = Vec::<u8>::new();
					buf.resize((len-1) as usize, 0);
					gl::GetProgramInfoLog(id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
					Err(Error::new(self.raw.name.clone() + ": " + String::from_utf8(buf).unwrap().as_str()))
				} else {
					Err(Error::new(String::new()))
				}
			} else {
				Ok(ProgramLinked { raw: self.raw })
			}
		}
	}
}