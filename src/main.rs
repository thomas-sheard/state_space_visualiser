use nannou::{color::FromColor, prelude::*};
use image;

// state stores a single double pendula's information
#[derive(Copy, Clone)]
struct State {
    theta_1: f32,
    theta_2: f32,
    dot_theta_1: f32,
    dot_theta_2: f32,
}

// implement vector scalar multiplication, division, and addition on State (for rk4)
impl std::ops::Mul<f32> for State {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            theta_1: self.theta_1 * rhs,
            theta_2: self.theta_2 * rhs,
            dot_theta_1: self.dot_theta_1 * rhs,
            dot_theta_2: self.dot_theta_2 * rhs,
        }
    }
}

impl std::ops::Div<f32> for State {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            theta_1: self.theta_1 / rhs,
            theta_2: self.theta_2 / rhs,
            dot_theta_1: self.dot_theta_1 / rhs,
            dot_theta_2: self.dot_theta_2 / rhs,
        }
    }
}

impl std::ops::Add<State> for State {
    type Output = Self;

    fn add(self, other: State) -> Self {
        Self {
            theta_1: self.theta_1 + other.theta_1,
            theta_2: self.theta_2 + other.theta_2,
            dot_theta_1: self.dot_theta_1 + other.dot_theta_1,
            dot_theta_2: self.dot_theta_2 + other.dot_theta_2,
        }
    }
}

fn derivatives(state: &State) -> State {
    
    // cache reused values
    let dot_theta_1 = state.dot_theta_1;
    let dot_theta_2 = state.dot_theta_2;

    let dtheta = state.theta_1 - state.theta_2;

    let sin_theta_1 = state.theta_1.sin();
    let sin_theta_2 = state.theta_2.sin();

    let sin_dtheta = dtheta.sin();
    let cos_dtheta = dtheta.cos();

    let denominator = 1.0 + (M_RATIO * sin_dtheta * sin_dtheta);

    // equations from uni edinburgh (page 30-31):
    // https://www2.ph.ed.ac.uk/~dmarendu/MVP/DoublePendulumTutorial.pdf
    let num_1 = (M_RATIO_PLUS * GAMMA * sin_theta_1) + (M_RATIO * L_RATIO * dot_theta_2 * dot_theta_2 * sin_dtheta) + (M_RATIO * cos_dtheta * (dot_theta_1 * dot_theta_1 * sin_dtheta - GAMMA * sin_theta_2));

    // pendulum 1 acceleration
    let ddot_theta_1 = - num_1 / denominator;

    let num_2 = M_RATIO * (dot_theta_1 * dot_theta_1 * sin_dtheta - GAMMA * sin_theta_2) + cos_dtheta * (M_RATIO_PLUS * GAMMA * sin_theta_1 + M_RATIO * L_RATIO * dot_theta_2 * dot_theta_2 * sin_dtheta);

    // pendulum 2 acceleration
    let ddot_theta_2 = num_2 / (L_RATIO * denominator);

    // return new values from initial state
    State { 
        theta_1: dot_theta_1,
        theta_2: dot_theta_2,
        dot_theta_1: ddot_theta_1,
        dot_theta_2: ddot_theta_2,
    }
}

fn rk4(state: &State, dt: f32) -> State {
    
    // rk4 implementation

    // k1 is equivalent to euler's method (poor performance)
    let k1 = derivatives(state) * dt;

    // we then take the new state estimate that k1 gives and make another state estimate for what the derivatives would look like after half of the time step
    let k2_state = *state + k1 * 0.5;
    let k2 = derivatives(&k2_state) * dt;

    // and then use the k2 estimate for k3... etc
    let k3_state = *state + k2 * 0.5;
    let k3 = derivatives(&k3_state) * dt;
    
    let k4_state = *state + k3;
    let k4 = derivatives(&k4_state) * dt;

    // return new state from weighted averages
    *state + (k1 + k2 * 2.0 + k3 * 2.0 + k4) / 6.0
}

// constants

// render dimensions
const RESOLUTION: f32 = 1024.0;

// constant model params
const GRAVITY: f32 = 10.0; // gravity (m/s)
const M1: f32 = 1.0; // bob 1 mass (kg)
const M2: f32 = 1.0; // bob 2 mass (kg)
const L1: f32 = 1.0; // arm 1 length (m)
const L2: f32 = 1.0; // arm 2 length (m)

// calculated constants that are used in rk4
const M_RATIO: f32 = M1 / M2;
const M_RATIO_PLUS: f32 = M_RATIO + 1.0;
const L_RATIO: f32 = L1 / L2;
const GAMMA: f32 = GRAVITY / L1;

fn main() {

    println!("Instantiating scene with {} pendula...", (RESOLUTION * RESOLUTION) as u32);

    // for each pendulum p_ij in scene set initial position to be ij
    let mut scene: Vec<Vec<State>> = (0..RESOLUTION as usize).map(|i| {
        (0..RESOLUTION as usize).map(|j| State {
            theta_1: map_range(i as f32, 0.0, RESOLUTION, -PI, PI),
            theta_2: map_range(j as f32, 0.0, RESOLUTION, -PI, PI),
            dot_theta_1: 0.0,
            dot_theta_2: 0.0
        }).collect()
    }).collect();

    println!("Done.");

    let duration: f32 = 2.0; // duration to render in seconds

    // separating ups and fps lets us maintain simulation accuracy at lower rendering cost
    let ups: f32 = 120.0; // updates per second (simulation steps)
    let fps: f32 = 30.0; // frames per second (steps to actually render)

    let repeat_each = (ups / fps) as u32;

    // timestep for simulation
    let dt: f32 = 1.0 / ups;

    // number of sim steps to run
    let iterations = (duration / dt) as u32;

    // number of frames to render
    let frames = iterations / repeat_each;

    // for each of the frames to be rendered
    for frame in 1..=frames {
        // iterate the physics by the number of physics steps per frame, and
        for _update in 0..repeat_each as u32 {
            // for each pixel (pendulum) in the scene
            for i in 0..RESOLUTION as usize {
                for j in 0..RESOLUTION as usize {
                    // simulate that many times
                    scene[i][j] = rk4(&scene[i][j], dt);
                }
            }
        }
        // then render every repeat_each'th frame
        println!("Rendering frame {}/{} ({:.1}%)", frame, frames+1, (frame as f32 / (frames+1) as f32) * 100.0);
        render_scene(&scene, frame);
    }

    println!("Rendering complete! Run\nffmpeg -framerate {} -pattern_type glob -i \"output/*.png\" -c:v libx264 -pix_fmt yuv420p render.mp4\nto concatenate into video.", fps as u32);
    
}

fn render_scene(scene: &Vec<Vec<State>>, iteration: u32) {

    let res = RESOLUTION as usize;
    let mut raw_colours = Vec::with_capacity(res * res * 3);

    // for each pixel / pendulum, convert its current position to RGB
    // rows need reversing because images start from the top, sim starts from the bottom
    for i in (0..res).rev() {
        for j in 0..res {
            //let y = res - j;
            let colour = colourmap(&scene[i][j]);
            //let colour = debug_colormap(i as u32, j as u32);
            raw_colours.extend_from_slice(&colour);
        }
    }
 
    // export as png
    //println!("{:?}", raw_colours);
    //println!("{:?}", raw_colours.len());

    let filename = format!("output/frame_{:04}.png", iteration);

    image::save_buffer(
        filename,
        &raw_colours,
        res as u32,
        res as u32,
        image::ColorType::Rgb8,
    ).expect("Render failed.");

}

//fn debug_colormap(x: u32, y: u32) -> [u8; 3] {
//    if (x + y) % 2 == 0 { [255, 255, 255] } else { [0, 0, 0] }
//}

fn colourmap(state: &State) -> [u8; 3] {

    // any mapping from [-PI, PI) -> Z^3 will work here
    // but non-unique mappings give less visually interesting

    let x = state.theta_2;
    let y = state.theta_1;
    let scale = 1.0 * 2.0 * PI;

    let hue = map_range(x, -scale / 2.0, scale / 2.0, 0.0, 1.0);

    let saturation = map_range(y, -scale / 2.0, scale / 2.0, 0.0, TAU).sin() / 3.0 + 0.66;

    let value = map_range(y, -scale / 2.0, scale / 2.0, 0.0, TAU).cos() / 3.0 + 0.6;

    let rgb = Rgb::from_hsv(hsv(hue, saturation, value));

    let r: u8 = map_range(rgb.red, 0.0, 1.0, 0, 255);
    let g: u8 = map_range(rgb.green, 0.0, 1.0, 0, 255);
    let b: u8 = map_range(rgb.blue, 0.0, 1.0, 0, 255);

    [r, g, b]
}
