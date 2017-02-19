use simd;

#[derive(Debug, Copy, Clone)]
pub enum SimdValue {
    F32x4(simd::f32x4),
    I32x4(simd::i32x4),
    U32x4(simd::u32x4),
    Bool32fx4(simd::bool32fx4),
    Bool32ix4(simd::bool32ix4),
}
