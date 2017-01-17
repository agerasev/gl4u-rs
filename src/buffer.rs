extern crate gl;

use std::mem;

use gl::types::*;

pub struct Buffer {
	pub id: GLuint,
	pub dim: i32
}

impl Buffer {
	pub fn new(dim: i32) -> Self {
		let mut buf = Buffer { id: 0, dim: dim };
		unsafe { gl::GenBuffers(1, &mut buf.id); }
		buf
	}

	pub fn bind(&self) {
		unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id); }
	}

	pub fn unbind() {
		unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0); }
	}

	pub fn load_float(&mut self, data: &[GLfloat]) {
		self.bind();
		unsafe { 
			gl::BufferData(
				gl::ARRAY_BUFFER, 
				(data.len()*mem::size_of::<GLfloat>()) as GLsizeiptr, 
				mem::transmute(&data[0]), 
				gl::STATIC_DRAW
			);
		}
		Self::unbind();
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		unsafe { gl::DeleteBuffers(1, &mut self.id); }
	}
}