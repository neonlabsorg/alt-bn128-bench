//! alt-bn128-bench significant module

/// Formats a float number with precision in the sense of number of significant digits
pub fn precision(float: f64, prec: usize) -> String {
    // Compute absolute value
    let a = float.abs();

    // If abs value is greater than 1, then precision becomes less than "standard"
    let prec = if a >= 1. {
        // Reduce by number of digits, minimum 0
        let n = (1. + a.log10().floor()) as usize;
        if n <= prec {
            prec - n
        } else {
            0
        }
        // If precision is less than 1 (but non-zero), then precision becomes greater than "standard"
    } else if a > 0. {
        // Increase number of digits
        let n = -(1. + a.log10().floor()) as usize;
        prec + n
        // Special case for 0
    } else {
        0
    };

    // Format with the given computed precision
    format!("{0:.1$}", float, prec)
}
