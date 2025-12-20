import argparse
import os

import numpy as np
from tqdm import tqdm

import mandelbrot


def generate_zoom_data(
    x_center: str,
    y_center: str,
    start_scale: str,
    scale_rate: float,
    max_iter: int,
    width: int,
    height: int,
    n_frames: int,
    data_dir: str,
):
    for i in tqdm(range(n_frames)):
        frame = np.array(
            mandelbrot.render_frame(
                width, height, x_center, y_center, start_scale, max_iter,
            ),
            dtype=np.float64,
        ).reshape(height, width)
        # Mask points in the set.
        frame[frame == max_iter] = np.nan
        
        # Save the file.
        filename = str(i).zfill(len(str(n_frames)))
        np.save(os.path.join(data_dir, f"{filename}.npy"))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("x_center", type=str)
    parser.add_argument("y_center", type=str)
    parser.add_argument("data_dir", type=str)
    parser.add_argument("--width", type=int, default=1920)
    parser.add_argument("--height", type=int, default=1080)
    parser.add_argument("--scale", type=str, default="4")
    parser.add_argument("--rate", type=float, default=0.99)
    parser.add_argument("--iters", type=int, default=2000)
    parser.add_argument("--frames", type=int, default=1)

    args = parser.parse_args()

    generate_zoom_data(
        args.x_center,
        args.y_center,
        args.scale,
        args.rate,
        args.iters,
        args.width,
        args.height,
        args.frames,
        args.data_dir,
    )
