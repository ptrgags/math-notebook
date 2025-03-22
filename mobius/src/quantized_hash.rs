use std::hash::Hash;

/// Trait for a type that can be quantized to form a
/// hash key. The hash key may depend on the quantization
/// bits.
pub trait QuantizedHash {
    type QuantizedType: Eq + Hash;
    fn quantize(&self, quantize_bits: i32) -> Self::QuantizedType;
}
