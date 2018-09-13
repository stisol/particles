use super::FieldProvider;

use std::f32;
use bincode::deserialize;

const TEST_DATA: &[u8] = include_bytes!("out2.bincode");  // include_bytes!("brain.bincode");
const WIDTH:     usize = 38;  // 148;
const HEIGHT:    usize = 39;  // 190;
const DEPTH:     usize = 40;  // 160;

type Vector3 = (f32, f32, f32);

#[derive(Serialize, Deserialize)]
struct VectorField {
    vectors: Vec<Vec<Vec<Vector3>>>,
}

fn lerpf(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

fn lerp((ax, ay, az): Vector3, (bx, by, bz): Vector3, t: f32) -> Vector3 {
    (lerpf(ax, bx, t), lerpf(ay, by, t), lerpf(az, bz, t))
}

fn lerp2d(lxly: Vector3, lxuy: Vector3, uxly: Vector3, uxuy: Vector3, t1: f32, t2: f32) -> Vector3 {
    let s = lerp(uxly, lxuy, t1);
    let v = lerp(lxly, uxuy, t2);
    lerp(s, v, t2)
}

#[allow(unknown_lints, too_many_arguments)]
fn lerp3d(
    v1: Vector3,
    v2: Vector3,
    v3: Vector3,
    v4: Vector3,
    v5: Vector3,
    v6: Vector3,
    v7: Vector3,
    v8: Vector3,
    t1: f32,
    t2: f32,
    t3: f32,
) -> Vector3 {
    let s = lerp2d(v1, v2, v3, v4, t1, t2);
    let v = lerp2d(v5, v6, v7, v8, t1, t2);
    lerp(s, v, t3)
}

pub struct SphereFieldProvider {
    data: Vec<(f32, f32, f32)>,
}

impl SphereFieldProvider {
    fn get_vec(&self, (fx, fy, fz): (usize, usize, usize)) -> (f32, f32, f32) {
        let fx = fx.min(WIDTH);
        let fy = fy.min(HEIGHT);
        let fz = fz.min(DEPTH);
        let index = fz + fy * WIDTH + fx * WIDTH * HEIGHT;
        self.data[index]
        //(0.0, 0.0, 0.0)
    }
}

impl FieldProvider for SphereFieldProvider {
    fn new() -> Self {
        let mut data = Vec::new();
        let x : VectorField = deserialize(TEST_DATA).unwrap();
        for plane in x.vectors {
            for row in plane {
                for elem in row {
                    data.push(elem);
                }
            }
        }
        SphereFieldProvider { data }
    }

    fn delta(&self, (x, y, z): (f32, f32, f32)) -> (f32, f32, f32) {
        let x = x * (WIDTH as f32) + (WIDTH as f32)/2.0;
        let y = y * (HEIGHT as f32) + (HEIGHT as f32)/2.0;
        let z = z * (DEPTH as f32) + (DEPTH as f32)/2.0;
        let lx = x.floor() as usize;
        let ly = y.floor() as usize;
        let lz = z.floor() as usize;
        let ux = x.ceil() as usize;
        let uy = y.ceil() as usize;
        let uz = z.ceil() as usize;
        let v1 = self.get_vec((lx, ly, lz));
        let v2 = self.get_vec((lx, ly, uz));
        let v3 = self.get_vec((lx, uy, lz));
        let v4 = self.get_vec((lx, uy, uz));
        let v5 = self.get_vec((ux, ly, lz));
        let v6 = self.get_vec((ux, ly, uz));
        let v7 = self.get_vec((ux, uy, lz));
        let v8 = self.get_vec((ux, uy, uz));

        use std::f32;
        // remove noise
        if  v1 == (0.0,0.0,0.0) &&
            v2 == (0.0,0.0,0.0) &&
            v3 == (0.0,0.0,0.0) &&
            v4 == (0.0,0.0,0.0) &&
            v5 == (0.0,0.0,0.0) &&
            v6 == (0.0,0.0,0.0) && 
            v7 == (0.0,0.0,0.0) &&
            v8 == (0.0,0.0,0.0) {
            return (f32::NAN,f32::NAN,f32::NAN);
        }

        let t1 = x - x.floor();
        let t2 = y - y.floor();
        let t3 = z - z.floor();

        lerp3d(v1, v2, v3, v4, v5, v6, v7, v8, t1, t2, t3)
    }
}
