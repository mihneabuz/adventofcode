use ndarray::{Array, Array2};

pub fn from_str(s: &str) -> Array2<u8> {
    let mut m = 0;
    let mut acc = Vec::new();

    for b in s.bytes() {
        if b == b'\n' {
            m += 1;
            continue;
        }
        acc.push(b);
    }

    if !s.ends_with('\n') {
        m += 1;
    }

    let n = (s.len() - m + 1) / m;
    Array::from_vec(acc).into_shape((n, m)).unwrap()
}
