#[macro_use]
extern crate glium;

// Based on the code at http://tomaka.github.io/glium/tutorials/03-colors.html

fn main() {
    use glium::{DisplayBuild, Surface};

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);

    let vertex1 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex { position: [ 0.0,  0.5], tex_coords: [0.0, 1.0] };
    let vertex3 = Vertex { position: [ 0.5, -0.25], tex_coords: [1.0, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;
        uniform sampler2D tex2;

        void main() {
            color = vec4(texture(tex, v_tex_coords).xy, texture(tex2, v_tex_coords).zw);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut x = 0.0;
    let tex1 = glium::texture::Texture2d::new(&display, vec![vec![(1.0, 1.0, 1.0)]]).unwrap();
    loop {
        x = if x >= 1.0 { 0.0 } else { x + 0.05 };

        // Generating new textures causes glium to go through all texture units,
        // eventually overflowing an [32 * 6] array in the AMD driver
        let tex2 = glium::texture::Texture2d::new(&display, vec![vec![(x, x, x)]]).unwrap();
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // Using any samplers will trigger the crash, as there is some sampler data
        // right after the [32 * 6] array
        let uniforms = uniform! {
            tex: tex1.sampled(),
            tex2: &tex2,
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
