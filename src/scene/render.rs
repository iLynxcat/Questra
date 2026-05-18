use std::ops::{Add, Mul, Sub};

pub fn lerp_smooth<T>(current: T, target: T, half_life: f32, dt: f32) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T>,
{
    current + (target - current) * (1.0 - (-dt / half_life).exp())
}
