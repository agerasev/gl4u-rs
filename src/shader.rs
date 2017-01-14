extern crate gl;

use gl::types::*;

enum Type {
	Vertex,
	Fragment,
}

struct Shader {
	pub id: GLuint;
}

impl Shader {
	fn new(t: Type) -> Self {
		let glt = match t {
			Vertex => gl::VERTEX_SHADER,
			Fragment => gl::FRAGMENT_SHADER,
		}
		Shader { id: gl::CreateShader(glt); }
	}

	fn load_str(&mut self, src: &str) -> Result<Self, String> {

	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		gl::DeleteShader(self.id);
	}
}