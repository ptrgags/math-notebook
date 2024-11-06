pub const EPSILON: f64 = 1e-15;

pub fn is_nearly(a: f64, b: f64) -> bool {
    // based on https://stackoverflow.com/a/28751714
    let diff = (a - b).abs();

    if diff <= EPSILON {
        true
    } else {
        diff <= EPSILON * a.abs().max(b.abs())
    }
}

#[cfg(test)]
pub fn assert_nearly(a: f64, b: f64) {
    if !is_nearly(a, b) {
        panic!("!is_nearly({}, {})", a, b);
    }
}
