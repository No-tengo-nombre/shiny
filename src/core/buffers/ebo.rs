use crate::assert_gl_is_loaded;
use gl;
use gl::types::*;
use std::mem::size_of;

/// An OpenGL Element Buffer Object.
#[derive(Copy, Clone)]
pub struct EBO {
    _id: u32,
    _count: u32,
}

impl EBO {
    /// Generates a new instance of an Element Buffer Object containing the given
    /// indices, asuming the usage `GL_STATIC_DRAW`.
    pub fn new<T>(indices: &[T], count: u32) -> EBO {
        return EBO::new_with_usage(indices, count, gl::STATIC_DRAW);
    }

    /// Generates a new instance of an Element Buffer Object containing the given
    /// indices, allowing the user to specify the usage.
    pub fn new_with_usage<T>(indices: &[T], count: u32, usage: GLenum) -> EBO {
        assert_gl_is_loaded();
        let mut ebo = 0;
        unsafe {
            // Generate the VBO
            gl::GenBuffers(1, &mut ebo);
            assert_ne!(ebo, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            // Buffer the vertices
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * size_of::<T>()) as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                usage,
            );
        }
        return EBO {
            _id: ebo,
            _count: count,
        };
    }

    pub fn get_count(&self) -> u32 {
        return self._count;
    }

    pub fn bind(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self._id);
        }
    }

    pub fn unbind(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn del(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::DeleteBuffers(1, &self._id);
        }
    }
}
