# fractals

## Building the Rust Module

Install `maturin` using `pip` or `mamba` or whatever.
From within the `rust` directory (within your Python virtual environment), run `maturin develop`.
This will build the module and install it into the current Python environment.


## Generating a Mandelbrot Zoom

Use the `generate_zoom.py` script by running something like:
```
python fractals/generate_zoom.py -0.743643887037151 0.13182590420533 "zoom_data/test_zoom/" --rate 0.9 --frames 75
```
