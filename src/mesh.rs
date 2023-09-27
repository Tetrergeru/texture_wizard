use gl::types::{GLint, GLsizeiptr, GLuint, GLvoid};

pub struct Mesh {
    _vbo: Vec<Vbo>,
    vao: Vao,
    vertex_count: usize,
}

impl Mesh {
    pub fn default_plain(reverse_y: bool) -> Self {
        let vao = Vao::new();

        let vec = vec![
            -1.0, -1.0, 0.0, //
            1.0, -1.0, 0.0, //
            -1.0, 1.0, 0.0, //
            -1.0, 1.0, 0.0, //
            1.0, -1.0, 0.0, //
            1.0, 1.0, 0.0, //
        ];

        let fake_positions = Vbo::new(&vec);
        let real_positions = Vbo::new(&vec);

        let mut uvs = vec![
            0.0, 1.0, //
            1.0, 1.0, //
            0.0, 0.0, //
            0.0, 0.0, //
            1.0, 1.0, //
            1.0, 0.0, //
        ];

        if reverse_y {
            for uv in uvs.iter_mut().skip(1).step_by(2) {
                *uv = -*uv;
            }
        }

        let uvs = Vbo::new(&uvs);
        vao.attach_vbo(&real_positions, 0, 3);
        vao.attach_vbo(&fake_positions, 1, 3);
        vao.attach_vbo(&uvs, 2, 2);
        Self {
            _vbo: vec![fake_positions, real_positions, uvs],
            vao,
            vertex_count: 6,
        }
    }

    pub fn draw(&self) {
        self.vao.bind();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count as i32);
        }
    }
}

struct Vbo {
    vbo: u32,
}

impl Vbo {
    pub fn new(data: &Vec<f32>) -> Self {
        let mut vbo = 0;
        unsafe {
            gl::CreateBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        Self { vbo }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

struct Vao {
    vao: gl::types::GLuint,
}

impl Vao {
    pub fn new() -> Self {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        Self { vao }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn attach_vbo(&self, vbo: &Vbo, idx: GLuint, size: i32) {
        vbo.bind();

        unsafe {
            gl::BindVertexArray(self.vao);

            gl::EnableVertexAttribArray(idx);

            // gl::VertexAttribPointer(0, 3, gl::FLOAT, 0, 3 * 4, std::ptr::null());
            gl::VertexAttribPointer(
                idx,
                size,
                gl::FLOAT,
                gl::FALSE,
                size * (std::mem::size_of::<f32>()) as GLint,
                std::ptr::null(),
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}

impl Default for Vao {
    fn default() -> Self {
        Self::new()
    }
}
