use num::Complex;

/// Try to determine if a given complex number `c` is in the Mandelbrot set or not, using at most `max_tries` iterations to decide
pub fn escape_time(c: Complex<f64>, max_tries: usize) -> Option<usize> {
    let mut z = Complex::<f64>::default();

    for i in 0..max_tries {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }

        z = (z * z) + c;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_time_test() {
        let complex = Complex { re: -1.0, im: 1.0 };
        assert_eq!(escape_time(complex, 100), Some(3));

        let complex = Complex { re: -1.25, im: 0.2 };
        assert_eq!(escape_time(complex, 100), Some(11));

        let complex = Complex {
            re: -0.02,
            im: 0.01,
        };
        assert_eq!(escape_time(complex, 100), None);
    }
}
