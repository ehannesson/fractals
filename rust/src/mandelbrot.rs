fn evaluate_single_point(cx: f64, cy: f64, max_iter: u32) -> u32 {
    /// Compute Mandelbrot escape iteration count.
    /// 
    /// Returns the number of iterations it takes for the point to escape, or
    /// `max_iter` if the point does not escape.
    let mut x: f64 = 0;
    let mut y: f64 = 0;

    for iter in 0..max_iter {
        // Perform one iteration
        let xn = x*x - y*y + cx;
        let yn = 2.0*x*y + cy;
        // Check if we've escaped
        if xn*xn + yn*yn > 4.0 {
            iter + 1
        }
        // Update variables with new value
        x = xn;
        y = yn;
    }
    iter + 1
}