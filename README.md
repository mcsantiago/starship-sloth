# RS-Sloth-Rasterizer

This is a Rust port of the [sloth-rasterizer](https://github.com/mcsantiago/sloth-rasterizer) project. 

## Features
- TODO:

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

### License
This project is licensed under the MIT License.

### Acknowledgements
This project is based on the [tiny-renderer](https://github.com/ssloy/tinyrenderer) project. This project is merely an exercise to understand the basics of software rasterization and hopefully act as the foundation for future GPU-based renderings.
