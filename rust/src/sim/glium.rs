use std::f32::consts::PI;
use std::time::{Instant, Duration};
use std::{mem, ptr};

use glium::{self, DisplayBuild, Surface};

use palette::Rgb;

use {Spin, NLEDS};

const MM: f32 = 0.001;

const R0: f32 = 5.0*MM;
const LED: f32 = 2.8*MM;
const SPACE: f32 = 0.0*MM;

// const LED_DUR: u64 = 2200; // milliseconds
const NVERTS: usize = 120;

pub struct SimSpin {
    phi: f32,
    omega: f32,
    alpha: f32,

    update_time: Instant,
    last_draw: Instant,

    leds: [Rgb; NLEDS],

    display: glium::backend::glutin_backend::GlutinFacade,
    program: glium::Program,

    shapes: [Vec<Vertex>; NLEDS],
    vertex_buffers: [glium::VertexBuffer<Vertex>; NLEDS],
}

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
    creation_time: Instant,
}
implement_vertex!(Vertex, position, color);

impl Vertex {
    fn new(x: f32, y: f32, c: Rgb) -> Vertex {
        Vertex {
            position: [x, y],
            color: [c.red, c.green, c.blue],
            creation_time: Instant::now(),
        }
    }
}

impl SimSpin {
    pub fn new() -> SimSpin {
        let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
        let program = glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

        let (shapes, vertex_buffers) = unsafe {
            let mut shapes: [Vec<Vertex>; NLEDS] = mem::uninitialized();
            let mut vertex_buffers: [glium::VertexBuffer<Vertex>; NLEDS] = mem::uninitialized();

            for (shape, buffer) in shapes.iter_mut().zip(vertex_buffers.iter_mut()) {
                let new_shape = vec![Vertex::new(0.0, 0.0, Rgb::new(0.0, 0.0, 0.0)); NVERTS];
                ptr::write(shape, new_shape);
                let new_buffer = glium::VertexBuffer::dynamic(&display, &shape).unwrap();
                ptr::write(buffer, new_buffer);
            }
            (shapes, vertex_buffers)
        };

        let phi = PI/2.0;

        SimSpin {
            phi: phi,
            omega: 2.0*PI * 1.0,
            alpha: 0.0,
            update_time: Instant::now(),
            last_draw: Instant::now(),
            leds: [Rgb::new(1.0, 1.0, 1.0); NLEDS],

            display: display,
            program: program,
            shapes: shapes,
            vertex_buffers: vertex_buffers,
        }
    }
}

impl Spin for SimSpin {
    fn update(&mut self) {
        let now = Instant::now();
        let dur = now.duration_since(self.update_time);
        let dt: f32 = dur.as_secs() as f32 + (dur.subsec_nanos() as f32)/1.0e9;
        println!("{}", 2.0 * PI / (self.omega * dt));

        // fixme: do real stuff here
        self.phi += self.omega * dt;

        if self.phi > 2.0*PI { self.phi -= 2.0*PI; }

        for (i, (shape, led)) in
            self.shapes.iter_mut()
            .zip(self.leds.iter())
            .enumerate()
        {
            // shape.retain(|s| now.duration_from_earlier(s.creation_time) < Duration::from_millis(LED_DUR));
            let v1 = R0 + (i as f32)*(LED + SPACE);
            let v2 = v1 + LED;

            shape.remove(0);
            shape.remove(0);
            shape.push(Vertex::new(v1*self.phi.cos(), v1*self.phi.sin(), led.clone()));
            shape.push(Vertex::new(v2*self.phi.cos(), v2*self.phi.sin(), led.clone()));
        }

        let frame = Duration::new(0, (1.0e9/60.0) as u32);
        let since_last_draw = now.duration_since(self.last_draw);

        if since_last_draw > frame {
            let mut target = self.display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);

            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

            for (buffer, shape) in self.vertex_buffers.iter().zip(self.shapes.iter()) {
                buffer.write(shape);
                target.draw(buffer, &indices, &self.program, &glium::uniforms::EmptyUniforms,
                            &Default::default()).unwrap();
            }

            target.finish().unwrap();
            for ev in self.display.poll_events() {
                match ev {
                    glium::glutin::Event::Closed => return,
                    _ => (),
                }
            }
            self.last_draw = now;
        }

        self.update_time = now;
    }

    fn phi(&self) -> f32 {
        self.phi
    }
    fn omega(&self) -> f32 {
        self.omega
    }
    fn alpha(&self) -> f32 {
        self.alpha
    }

    fn leds(&mut self) -> &mut [Rgb; NLEDS] {
        &mut self.leds
    }
}


const VERTEX_SHADER: &'static str = r#"
#version 140

in vec2 position;
in vec3 color;

flat out vec3 v_color;

void main() {
    gl_Position = vec4(position*10, 0.0, 1.0);
    v_color = color;
}
"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 140

flat in vec3 v_color;
out vec4 f_color;

void main() {
    f_color = vec4(v_color, 0.1);
}
"#;

