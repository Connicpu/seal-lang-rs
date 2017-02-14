use simd;

#[derive(Debug, Copy, Clone)]
pub enum SimdValue {
    F32x4(simd::f32x4),
}
