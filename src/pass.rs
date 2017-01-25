extern crate gl;

use std::{ffi, ptr};
use std::collections::{LinkedList};

use gl::types::*;

use buffer::Buffer;

pub enum Prim {
	Triangles,
	Quads
}

pub struct Pass {
	pub id: GLuint,
	pub attribs: LinkedList<GLint>,
	pub first: i32,
	pub count: i32,
	pub prim: Prim,
}

impl Pass {
	pub fn new(id: GLuint) -> Pass {
		unsafe { gl::UseProgram(id); }
		Pass { id: id, attribs: LinkedList::<GLint>::new(), first: 0, count: 0, prim: Prim::Triangles }
	}

	fn get_uniform_location(&self, name: &str) -> Result<GLint, String> {
		let id = unsafe { gl::GetUniformLocation(self.id, ffi::CString::new(name.as_bytes()).unwrap().as_ptr()) };
		if id != -1 { Ok(id) } else { Err(String::new() + "uniform '" + name + "' location error") }
	}

	fn get_attribute_location(&self, name: &str) -> Result<GLint, String> {
		let id = unsafe { gl::GetAttribLocation(self.id, ffi::CString::new(name.as_bytes()).unwrap().as_ptr()) };
		if id != -1 { Ok(id) } else { Err(String::new() + "attribute '" + name + "' location error") }
	}

	pub fn uniform_scalar(self, name: &str, value: GLfloat) -> Result<Pass, String> {
		match self.get_uniform_location(name) {
			Ok(id) => {
				unsafe{ gl::Uniform1fv(id, 1, &value); }
				Ok(self)
			},
			Err(err) => Err(err),
		}
	}

	pub fn uniform_vector(self, name: &str, data: &[GLfloat]) -> Result<Pass, String> {
		match self.get_uniform_location(name) {
			Ok(id) => {
				unsafe {
					match data.len() {
						1 => { gl::Uniform1fv(id, 1, &data[0]); },
						2 => { gl::Uniform2fv(id, 1, &data[0]); },
						3 => { gl::Uniform3fv(id, 1, &data[0]); },
						4 => { gl::Uniform4fv(id, 1, &data[0]); },
						_ => { return Err(String::new() + "vector size is not between 1 and 4"); },
					}
				}
				Ok(self)
			},
			Err(err) => Err(err),
		}
	}

	pub fn uniform_matrix(self, name: &str, data: &[GLfloat]) -> Result<Pass, String> {
		match self.get_uniform_location(name) {
			Ok(id) => {
				unsafe {
					match data.len() {
						4 => { gl::UniformMatrix2fv(id, 1, gl::TRUE, &data[0]); },
						9 => { gl::UniformMatrix3fv(id, 1, gl::TRUE, &data[0]); },
						16 => { gl::UniformMatrix4fv(id, 1, gl::TRUE, &data[0]); },
						_ => { return Err(String::new() + "matrix size is not between 2 and 4"); },
					}
				}
				Ok(self)
			},
			Err(err) => Err(err),
		}
	}

	pub fn attribute(mut self, name: &str, buffer: &Buffer) -> Result<Pass, String> {
		match self.get_attribute_location(name) {
			Ok(id) => {
				unsafe {
					self.attribs.push_back(id);
					gl::EnableVertexAttribArray(id as GLuint);
					buffer.bind();
					gl::VertexAttribPointer(id as GLuint, buffer.dim, gl::FLOAT, gl::FALSE, 0, ptr::null());
					Buffer::unbind();
				}
				Ok(self)
			},
			Err(err) => Err(err),
		}
	}

	pub fn range(mut self, first: i32, count: i32) -> Self {
		self.first = first;
		self.count = count;
		self
	}

	pub fn primitive(mut self, prim: Prim) -> Self {
		self.prim = prim;
		self
	}

	#[allow(unused_mut)]
	pub fn draw(mut self) -> Result<(), String> {
		unsafe {
			let glprim = match self.prim {
				Prim::Triangles => gl::TRIANGLES,
				Prim::Quads => gl::QUADS,
			};
			gl::DrawArrays(glprim, self.first, self.count);

			for id in self.attribs {
				gl::DisableVertexAttribArray(id as GLuint);
			}

			gl::UseProgram(0);
		}
		Ok(())
	}
}
