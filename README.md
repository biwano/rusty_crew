# My Bevy Game

A 3D space shooter game built with the [Bevy](https://bevyengine.org/) game engine. Control a spaceship, shoot projectiles at moving targets, and destroy them before they escape!

For more information about Bevy, check out the [Bevy documentation](https://docs.rs/bevy/).

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust**: This project uses Rust nightly toolchain (install from [rustup.rs](https://rustup.rs/))
- **Git**: For cloning the repository (if applicable)

The project includes a `rust-toolchain.toml` file that will automatically use the correct Rust version.

## Getting Started

### 1. Clone the Repository

```bash
git clone git@github.com:biwano/rusty_crew.git
cd rusty_crew
```

### 2. Install Dependencies

The project uses Cargo (Rust's package manager) which comes with Rust. Dependencies are automatically managed through `Cargo.toml`.

### 3. Build the Project

```bash
cargo build
```

For a release build with optimizations:

```bash
cargo build --release
```

### 4. Run the Game

```bash
cargo run
```

Or run the release version:

```bash
cargo run --release
```

## Troubleshooting

### Build Issues

If you encounter build errors:

1. Ensure you have the latest Rust toolchain: `rustup update`
2. Clean the build cache: `cargo clean`
3. Rebuild: `cargo build`

### Asset Loading Issues

- Ensure all `.glb` model files are in the `assets/models/` directory
- Check that asset paths in code match the file structure

## Contributing

Contributions are welcome! You can contribute to this project in two ways:

1. **Pull Requests**: Fork the repository, make your changes, and submit a pull request for review.
2. **Direct Access**: If you'd like write access to the repository, please reach out to request it.

All contributions should follow the existing code style and include appropriate tests/documentation where applicable.

## License

This project is licensed under the MIT License.

Copyright (c) 2024

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.



