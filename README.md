# gl4u

## TODO:
### Required:
- [ ] Context
- [x] Buffer (Vertex Buffer)
- [x] Shader
- [x] Program
- [ ] Texture
- [ ] Framebuffer

### Safety:
- [x] store immutable reference to shaders in program (to prevent dropping shaders before program)
- [ ] think how to forse Pass cannot break by other OpenGL calls (maybe borrow Context or put it in lambda or give all params to one function).
      (Pass is the complete execution step of program that includes binding buffers, uniforms and running program)

### Refactor:
- [ ] use Path instead of String for shader loading
- [x] different types for different stages of Shader and Program initialization
- [ ] refactor error message concatenation
- [ ] use Error instead of String in result of methods
