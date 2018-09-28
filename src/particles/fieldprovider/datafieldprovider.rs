use super::FieldProvider;

use bincode::deserialize;
use std::f32;

use resources::fields::TEST_DATA;

type Vector3 = (f32, f32, f32);

#[derive(Serialize, Deserialize)]
struct VectorField {
    width: usize,
    height: usize,
    depth: usize,
    vectors: Vec<Vec<Vec<Vector3>>>,
}

fn lerpf(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

fn lerp((ax, ay, az): Vector3, (bx, by, bz): Vector3, t: f32) -> Vector3 {
    (lerpf(ax, bx, t), lerpf(ay, by, t), lerpf(az, bz, t))
}

fn lerp2d(lxly: Vector3, lxuy: Vector3, uxly: Vector3, uxuy: Vector3, t1: f32, t2: f32) -> Vector3 {
    let s = lerp(lxuy, uxuy, t1);
    let v = lerp(lxly, uxly, t1);
    lerp(s, v, t2)
}

#[allow(unknown_lints, too_many_arguments)]
fn lerp3d(
    // naming scheme: face <n> <lower|upper>x <lower|upper>y
    f1lxly: Vector3,
    f1lxuy: Vector3,
    f1uxly: Vector3,
    f1uxuy: Vector3,
    f2lxly: Vector3,
    f2lxuy: Vector3,
    f2uxly: Vector3,
    f2uxuy: Vector3,
    t1: f32,
    t2: f32,
    t3: f32,
) -> Vector3 {
    let s = lerp2d(f1lxly, f1lxuy, f1uxly, f1uxuy, t1, t2);
    let v = lerp2d(f2lxly, f2lxuy, f2uxly, f2uxuy, t1, t2);
    lerp(s, v, t3)
}

pub struct DataFieldProvider {
    width: usize,
    height: usize,
    depth: usize,
    data: Vec<(f32, f32, f32)>,
}

impl DataFieldProvider {
    fn get_vec(&self, pos: (usize, usize, usize)) -> (f32, f32, f32) {
        match pos {
            (fx, fy, fz) if fx >= self.width() || fy >= self.height() || fz >= self.depth() => {
                (0.0, 0.0, 0.0)
            }
            (fx, fy, fz) => self.get(fx, fy, fz),
        }
    }
}

impl FieldProvider for DataFieldProvider {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn depth(&self) -> usize {
        self.depth
    }

    fn new() -> Self {
        let mut data = Vec::new();
        let x: VectorField = deserialize(TEST_DATA).unwrap();
        for plane in x.vectors {
            for row in plane {
                for elem in row {
                    data.push(elem);
                }
            }
        }
        DataFieldProvider {
            width: x.width,
            height: x.height,
            depth: x.depth,
            data,
        }
    }

    fn delta(&self, (x, y, z): (f32, f32, f32)) -> (f32, f32, f32) {
        let x = x * (self.width as f32) + (self.width as f32) / 2.0;
        let y = y * (self.height as f32) + (self.height as f32) / 2.0;
        let z = z * (self.depth as f32) + (self.depth as f32) / 2.0;
        let lx = x.floor() as usize;
        let ly = y.floor() as usize;
        let lz = z.floor() as usize;
        let ux = x.ceil() as usize;
        let uy = y.ceil() as usize;
        let uz = z.ceil() as usize;
        let v1 = self.get_vec((lx, ly, lz)); // lower depth
        let v2 = self.get_vec((lx, uy, lz)); // lower depth
        let v3 = self.get_vec((ux, ly, lz)); // lower depth
        let v4 = self.get_vec((ux, uy, lz)); // lower depth
        let v5 = self.get_vec((lx, ly, uz)); // upper depth
        let v6 = self.get_vec((lx, uy, uz)); // upper depth
        let v7 = self.get_vec((ux, ly, uz)); // upper depth
        let v8 = self.get_vec((ux, uy, uz)); // upper depth

        use std::f32;
        // remove noise
        if v1 == (0.0, 0.0, 0.0)
            && v2 == (0.0, 0.0, 0.0)
            && v3 == (0.0, 0.0, 0.0)
            && v4 == (0.0, 0.0, 0.0)
            && v5 == (0.0, 0.0, 0.0)
            && v6 == (0.0, 0.0, 0.0)
            && v7 == (0.0, 0.0, 0.0)
            && v8 == (0.0, 0.0, 0.0)
        {
            return (f32::NAN, f32::NAN, f32::NAN);
        }

        let t1 = x - x.floor();
        let t2 = y - y.floor();
        let t3 = z - z.floor();

        lerp3d(v1, v2, v3, v4, v5, v6, v7, v8, t1, t2, t3)
    }

    fn data(&self) -> &[(f32, f32, f32)] {
        &self.data
    }
}