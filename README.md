# geo-quadkey-rs
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/masaishi/geo-quadkey-rs/blob/main/LICENSE) [![crates.io](https://img.shields.io/crates/v/geo-quadkey-rs.svg?logo=rust)](https://crates.io/crates/geo-quadkey-rs)

`geo-quadkey-rs` is a Rust library for encoding and decoding geographical coordinates to and from QuadKeys, a tiling approach used by Microsoft's [Bing Maps Tile System](https://learn.microsoft.com/en-us/bingmaps/articles/bing-maps-tile-system) for interactive mapping solutions.

![Bing Maps Tile System](https://learn.microsoft.com/en-us/bingmaps/articles/media/5cff54de-5133-4369-8680-52d2723eb756.jpg)

*Source: [Microsoft Bing Maps Tile System](https://learn.microsoft.com/en-us/bingmaps/articles/bing-maps-tile-system)

I referenced this repository made in Ruby was very helpful in creating this library: [deg84/quadkey](https://github.com/deg84/quadkey/tree/master)

## Usage

Include `geo-quadkey-rs` in your `Cargo.toml`:

```toml
[dependencies]
geo-quadkey-rs = "0.1.0"
```

Then include it in your code:

```rust
extern crate geo_quadkey_rs;

use geo_quadkey_rs::Quadkey;

// Encode coordinates to a quadkey
let quadkey = Quadkey::encode(47.60357, -122.32945, 23);

// Decode a quadkey to coordinates
let (latitude, longitude, precision) = Quadkey::decode("12022001101101100101102");

// Find neighbors of a quadkey
let neighbors = Quadkey::neighbors("12022001101101100101102");
```

## Functions

The `Quadkey` struct provides the following methods:

- `encode(latitude: f64, longitude: f64, precision: usize) -> String`
- `decode(quadkey: &str) -> (f64, f64, usize)`
- `neighbors(quadkey: &str) -> Vec<String>`
- `clip(n: f64, min_value: f64, max_value: f64) -> f64`
- `map_size(precision: usize) -> f64`
- `ground_resolution(latitude: f64, precision: usize) -> f64`
- `coordinates_to_pixel(latitude: f64, longitude: f64, precision: usize) -> (i32, i32)`
- `pixel_to_coordinates(pixel_x: i32, pixel_y: i32, precision: usize) -> (f64, f64)`
- `pixel_to_tile(pixel_x: i32, pixel_y: i32) -> (i32, i32)`
- `tile_to_pixel(tile_x: i32, tile_y: i32) -> (i32, i32)`
- `tile_to_quadkey(tile_x: i32, tile_y: i32, precision: usize) -> String`
- `quadkey_to_tile(quadkey: &str) -> (i32, i32, usize)`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
