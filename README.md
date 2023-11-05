# RS-Sloth-Rasterizer

This is a Rust port of the [sloth-rasterizer](https://github.com/mcsantiago/sloth-rasterizer) project. 
The `rs-sloth-rasterizer` is a project that emphasizes the basics of 3D rasterization from the ground up. 
It implements fundamental techniques used in the graphics pipeline to convert 3D models into 2D representations on your screen, all the while managing depth and texture application accurately.

![Rendered 3D model]("./screenshots/screenshot.png")

## Features
- 3D Model Rendering: Transforms 3D models into 2D viewport projections using a software-based rendering pipeline.
- Texture Mapping: Incorporates UV mapping for texturing 3D models, allowing for intricate visual detail on surfaces.
- Z-Buffering: Utilizes a z-buffer algorithm to manage depth, ensuring that closer objects occlude those further away.
- Rasterization: Employs a rasterization process to fill in pixels, based on vector representations of triangles in 3D space.
- Triangle Interpolation: Interpolates vertex attributes such as UV coordinates across triangles for seamless texturing.
- Pixel Drawing: Manages pixel color assignment on the output image, resulting in the final rendered scene.
- Resolution Handling: Takes into account different screen resolutions to ensure that the rendering adapts to various display sizes.
- UV Coordinate Management: Includes a robust system to correct and manage UV coordinates to prevent texture distortion.

## Prerequisites
To build and run the project, you need to have the following installed:
- Rust 1.70.0 or higher

## Getting Started
1. Clone this repository to your local machien or download the source code.
2. Ensure that the required prerequisites are installed.
3. Open a terminal or command prompt and navigate to the project directory.
4. Run the following command to build the project:
```
cargo run
```

### Running with Logs
I use env_logger which is set to print stderr only by default. To change this, set the variable `RUST_LOG` to info, debug, or error.

```
RUST_LOG=info cargo run
```

### Contributing
Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement". Don't forget to give the project a star! Thanks again!

### License
This project is licensed under the MIT License.

### Acknowledgements
This project is based on the [tiny-renderer](https://github.com/ssloy/tinyrenderer) project. This project is merely an exercise to understand the basics of software rasterization and hopefully act as the foundation for future GPU-based renderings.
