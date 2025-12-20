use pyo3::prelude::*;


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
        // Update variables if not escaped
        x = xn;
        y = yn;
    }
    iter + 1
}


fn _render_frame(
    width: usize,
    height: usize,
    center_x: f64,
    center_y: f64,
    scale: f64,
    max_iter: u32,
) -> Vec<u32> {
    /// Render a full frame of Mandelbrot escape iterations.
    /// 
    /// The rendered frame is `width` pixels by `height` pixels. In the complex
    /// plane, the frame is centered on `(center_x, center_y)`, and has a width
    /// of `scale` units.
    /// 
    /// Parameters
    /// ----------
    /// width, height : usize
    ///     Frame width and height, respectively, in pixels.
    /// center_x, center_y : f64
    ///     Frame center in the complex plane.
    /// scale : f64
    ///     The width of the frame in the complex plane.
    /// max_iter : u32
    ///     The maximum number of iterations to perform before deciding a point
    ///     is part of the Mandelbrot set.
    /// 
    /// Returns
    /// -------
    /// buffer : Vec<u32>
    ///     Frame buffer of escape iterations. Buffer indexing begins at the
    ///     top left of the frame and makes its way across, then down. That 
    ///     is, the first `width` entries correspond to the first row of the
    ///     frame, the entries starting at `2*width` correspond to the second
    ///     row of the frame, and so on.
    
    let mut buffer = vec![0u32; width*height]

    let pixel_step = scale / width as f64;
    let x_start = center_x - (width as f64 / 2.0);
    let y_start = center_y - (height as f64 / 2.0);

    for y_iter in 0..height {
        let cy = y_start + (pixel_step * y_iter);
        for x_iter in 0..width {
            let cx = x_start + (pixel_step * x_iter);
            buffer[(y * width) + x] = evaluate_single_point(cx, cy, max_iter);
        }
    }
    buffer
}


#[pyfunction]
fn render_frame(
    width: usize,
    height: usize,
    center_x: &str,
    center_y: &str,
    scale: &str,
    max_iter: u32,
) -> PyResult<Vec<u32>> {
    let center_x = center_x.parse().unwrap();
    let center_y = center_y.parse().unwrap();
    let scale = scale.parse().unwrap();

    Ok(
        _render_frame(
            width,
            height,
            &center_x,
            &center_y,
            &scale,
            max_iter,
        )
    )
}


#[pymodule]
fn mandelbrot(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(render_frame, module)?)?;
    Ok(())
}
