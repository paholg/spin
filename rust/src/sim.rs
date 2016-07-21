use std::f32::consts::PI;
use std::time::{Instant, Duration};
use std::{mem, ptr};

use glium;

use NLEDS;
use color::Rgb;

// Eventually, we will use dimensioned for real units
#[allow(non_upper_case_globals)]
const mm: f32 = 0.001;

// distance to first led
const R0: f32 = 0.2 * mm;
const LED_SIZE: f32 = 2.8 * mm;
const SPACE: f32 = 0.2 * mm;


const ALPHA0: f32 = 2.0 * PI * 10.0; // rad/s^2
const OMEGA0: f32 = 0.0;
const OMEGA_MAX: f32 = 2.0 * PI * 30.0; // rad/s

const DPHIMAX: f32 = 2.0 * PI / 180.0;

const NVERTS: usize = (2.0 * PI / DPHIMAX * 4.0) as usize;

pub struct Spin {
    pub leds: [Rgb; NLEDS],

    phi: f32,
    omega: f32,
    alpha: f32,

    update_time: Instant,
    last_draw: Instant,

    display: glium::backend::glutin_backend::GlutinFacade,
    program: glium::Program,

    shapes: [Vec<Vertex>; NLEDS],
    vertex_buffers: [glium::VertexBuffer<Vertex>; NLEDS],
}

#[derive(Copy, Clone, Debug)]
struct Vertex {
    pub creation_time: Instant,
    pub phi: f32,
    position: [f32; 2],
    color: [u8; 3],
}
implement_vertex!(Vertex, position, color);

impl Vertex {
    fn new(phi: f32, x: f32, y: f32, c: Rgb) -> Vertex {
        Vertex {
            phi: phi,
            creation_time: Instant::now(),
            position: [x, y],
            color: [c.r, c.g, c.b],
        }
    }
}

impl Spin {
    pub fn new() -> Spin {
        use glium::DisplayBuild;
        let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
        let program = glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

        let (shapes, vertex_buffers) = unsafe {
            let mut shapes: [Vec<Vertex>; NLEDS] = mem::uninitialized();
            let mut vertex_buffers: [glium::VertexBuffer<Vertex>; NLEDS] = mem::uninitialized();

            for (shape, buffer) in shapes.iter_mut().zip(vertex_buffers.iter_mut()) {
                let new_shape = vec![Vertex::new(0.0, 0.0, 0.0, Rgb::new(0, 0, 0)); NVERTS];
                ptr::write(shape, new_shape);
                let new_buffer = glium::VertexBuffer::dynamic(&display, &shape).unwrap();
                ptr::write(buffer, new_buffer);
            }
            (shapes, vertex_buffers)
        };

        let phi = PI/2.0;

        Spin {
            phi: phi,
            omega: OMEGA0,
            alpha: ALPHA0,
            update_time: Instant::now(),
            last_draw: Instant::now(),
            leds: [Rgb::new(255, 255, 255); NLEDS],

            display: display,
            program: program,
            shapes: shapes,
            vertex_buffers: vertex_buffers,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let dur = now.duration_since(self.update_time);
        let dt: f32 = dur.as_secs() as f32 + (dur.subsec_nanos() as f32)/1.0e9;
        let dphi = self.omega * dt;

        println!("fps: {:6.1}, omega: {:6.1}, dphi: {:6.2} deg",
                 1.0/dt, self.omega, dphi * 180. / PI);

        // Move the disc
        if self.omega.abs() > OMEGA_MAX {
            self.alpha = -self.alpha;
        }

        self.omega += self.alpha * dt;
        self.phi += dphi;

        while self.phi >= 2.0*PI { self.phi -= 2.0*PI; }

        // if we've moved too much, we need to fill with some extra trapezoids
        // (2 triangles = 1 trapezoid)
        let ntraps = ::std::cmp::min((dphi.abs() / DPHIMAX).ceil() as u32, NVERTS as u32/2);

        // update leds
        for (i, (shape, led)) in
            self.shapes.iter_mut()
            .zip(self.leds.iter())
            .enumerate()
        {
            let v1 = R0 + (i as f32)*(LED_SIZE + SPACE);
            let v2 = v1 + LED_SIZE;

            for _ in 0..2*ntraps {
                shape.remove(0);
            }
            for i in 0..ntraps {
                let mini_dphi = dphi / ntraps as f32;
                let phi = self.phi - mini_dphi*(ntraps - i - 1) as f32;
                shape.push(Vertex::new(phi, v1*phi.cos(), v1*phi.sin(), *led));
                shape.push(Vertex::new(phi, v2*phi.cos(), v2*phi.sin(), *led));
            }

        }

        let frame_dur = Duration::new(0, (1.0e9/60.0) as u32);
        let since_last_draw = now.duration_since(self.last_draw);

        if since_last_draw > frame_dur {
            let mut target = self.display.draw();

            use glium::Surface;
            target.clear_color(0.0, 0.0, 0.0, 1.0);

            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

            for buf in self.vertex_buffers.iter_mut() {
                buf.invalidate();
            }

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
        ::std::thread::sleep(Duration::new(0, 10_000));
        self.update_time = now;
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
    f_color = vec4(v_color / 255, 1.0);
}
"#;
