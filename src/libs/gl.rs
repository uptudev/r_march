use gl33::global_loader as glob;
use gl33::*;
use std::os::raw::c_uint;

pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        glob::glClearColor(red, green, blue, alpha);
    }
}

pub fn buffer_data(ty: BufferType, data: &[u8], usage: GLenum) {
    unsafe {
        glob::glBufferData(
            GLenum(ty as u32),
            data.len().try_into().unwrap(),
            data.as_ptr().cast(),
            usage,
        );
    }
}

pub struct VertexArray(pub c_uint);
pub struct Buffer(pub c_uint);
pub struct Shader(pub c_uint);
pub struct ShaderProgram(pub c_uint);
pub enum BufferType {
    Array = GL_ARRAY_BUFFER.0 as isize,
    ElementArray = GL_ELEMENT_ARRAY_BUFFER.0 as isize,
}

pub enum ShaderType {
    Vertex = GL_VERTEX_SHADER.0 as isize,
    Fragment = GL_FRAGMENT_SHADER.0 as isize,
}

impl VertexArray {
    /// Creates a new VertexArray object
    pub fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe { glob::glGenVertexArrays(1, &mut vao) };
        if vao != 0 {
            Some(Self(vao))
        } else {
            None
        }
    }

    /// Bind this VertexArray as the current VAO
    pub fn bind(&self) {
        glob::glBindVertexArray(self.0)
    }

    /// Clear the current VAO binding
    pub fn clear_binding() {
        glob::glBindVertexArray(0)
    }
}

/// Basic wrapper for a [Buffer
/// Object](https://www.khronos.org/opengl/wiki/Buffer_Object).
impl Buffer {
    /// Makes a new vertex buffer
    pub fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe {
            glob::glGenBuffers(1, &mut vbo);
        }
        if vbo != 0 {
            Some(Self(vbo))
        } else {
            None
        }
    }

    /// Bind this vertex buffer for the given type
    pub fn bind(&self, ty: BufferType) {
        unsafe { glob::glBindBuffer(GLenum(ty as u32), self.0) }
    }

    /// Clear the current vertex buffer binding for the given type.
    pub fn clear_binding(ty: BufferType) {
        unsafe { glob::glBindBuffer(GLenum(ty as u32), 0) }
    }
}

/// A handle to a [Shader
/// Object](https://www.khronos.org/opengl/wiki/GLSL_Object#Shader_objects)
impl Shader {
    /// Makes a new shader.
    ///
    /// Prefer the [`Shader::from_source`](Shader::from_source) method.
    ///
    /// Possibly skip the direct creation of the shader object and use
    /// [`ShaderProgram::from_vert_frag`](ShaderProgram::from_vert_frag).
    pub fn new(ty: ShaderType) -> Option<Self> {
        let shader = glob::glCreateShader(GLenum(ty as u32));
        if shader != 0 {
            Some(Self(shader))
        } else {
            None
        }
    }

    /// Assigns a source string to the shader.
    ///
    /// Replaces any previously assigned source.
    pub fn set_source(&self, src: &str) {
        unsafe {
            glob::glShaderSource(
                self.0,
                1,
                &(src.as_bytes().as_ptr().cast()),
                &(src.len().try_into().unwrap()),
            );
        }
    }

    // Compiles the shader based on the current source.
    pub fn compile(&self) {
        glob::glCompileShader(self.0)
    }

    /// Checks if the last compile was successful or not.
    pub fn compile_success(&self) -> bool {
        let mut compiled = 0;
        unsafe { glob::glGetShaderiv(self.0, GL_COMPILE_STATUS, &mut compiled) };
        compiled == GL_TRUE.0 as i32
    }

    /// Gets the info log for the shader.
    ///
    /// Usually you use this to get the compilation log when a compile failed.
    pub fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { glob::glGetShaderiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_len) };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            glob::glGetShaderInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    ///Marks a shader for deletion.
    ///
    /// Note: This _does not_ immediately delete the shader. It only marks it
    /// for deletion. If the shader has been previouslt attached to a program,
    /// then the shader will stay allocated until it is unattached.
    pub fn delete(self) {
        glob::glDeleteShader(self.0)
    }

    /// Takes a shader type and source string and produces either the compiled
    /// shader or an error message.
    ///
    /// Prefer [`ShaderProgram::from_vert_frag`](ShaderProgram::from_vert_frag),
    /// it makes a complete program from the vertex and fragment sources all at
    /// once.
    pub fn from_source(ty: ShaderType, source: &str) -> Result<Self, String> {
        let id = Self::new(ty).ok_or_else(|| "Couldn't allocate new shader".to_string())?;
        id.set_source(source);
        id.compile();
        if id.compile_success() {
            Ok(id)
        } else {
            let out = id.info_log();
            id.delete();
            Err(out)
        }
    }
}

/// A handle to a [Program
/// Object](https://www.khronos.org/opengl/wiki/GLSL_Object#Program_objects)
impl ShaderProgram {
    /// Allocates a new program object.
    ///
    /// Prefer [`ShaderProgram::from_vert_frag`](ShaderProgram::from_vert_frag),
    /// it makes a complete program from the vertex and fragment sources all at
    /// once.
    pub fn new() -> Option<Self> {
        let prog = glob::glCreateProgram();
        if prog != 0 {
            Some(Self(prog))
        } else {
            None
        }
    }

    /// Takes a vertex shader source string and a fragment shader source string
    /// and either gets you a working program object or gets you an error message.
    ///
    /// This is the preferred way to create a simple shader program in the common
    /// case. It's just less error prone than doing all the steps yourself.
    pub fn from_vert_frag(vert: &str, frag: &str) -> Result<Self, String> {
        let p = Self::new().ok_or_else(|| "Couldn't allocate a program".to_string())?;
        let v = Shader::from_source(ShaderType::Vertex, vert)
            .map_err(|e| format!("Vertex Compile Error: {}", e))?;
        let f = Shader::from_source(ShaderType::Fragment, frag)
            .map_err(|e| format!("Fragment Compile Error: {}", e))?;
        p.attach_shader(&v);
        p.attach_shader(&f);
        p.link_program();
        v.delete();
        f.delete();
        if p.link_success() {
            Ok(p)
        } else {
            let out = format!("Program Link Error: {}", p.info_log());
            p.delete();
            Err(out)
        }
    }

    /// Attaches a shader object to this program object.
    pub fn attach_shader(&self, shader: &Shader) {
        glob::glAttachShader(self.0, shader.0);
    }

    /// Links the various attached, compiled shader objects into a usable program.
    pub fn link_program(&self) {
        glob::glLinkProgram(self.0);
    }

    /// Checks if the last linking operation was successful.
    pub fn link_success(&self) -> bool {
        let mut success = 0;
        unsafe { glob::glGetProgramiv(self.0, GL_LINK_STATUS, &mut success) };
        success == GL_TRUE.0 as i32
    }

    /// Gets the log data for this program.
    ///
    /// This is usually used to check the message when a program failed to link.
    pub fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { glob::glGetProgramiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_len) };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            glob::glGetProgramInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    /// Sets the program as the program to use when drawing.
    pub fn use_program(&self) {
        glob::glUseProgram(self.0);
    }

    /// Marks the program for deletion.
    ///
    /// Note: This _does not_ immediately delete the program. If the program is
    /// currently in use it won't be deleted until it's not the active program.
    /// When a program is finally deleted and attached shaders are unattached.
    pub fn delete(self) {
        glob::glDeleteProgram(self.0);
    }
}
