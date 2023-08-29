use std::f64::consts::PI;

pub struct Quadkey;

impl Quadkey {
    const MIN_LATITUDE: f64 = -85.05112878;
    const MAX_LATITUDE: f64 = 85.05112878;
    const MIN_LONGITUDE: f64 = -180.0;
    const MAX_LONGITUDE: f64 = 180.0;
    const EARTH_RADIUS: f64 = 6378137.0;

    /// Encode coordinates to a quadkey
    pub fn encode(latitude: f64, longitude: f64, precision: usize) -> String {
        let (pixel_x, pixel_y) = Self::coordinates_to_pixel(latitude, longitude, precision);
        let (tile_x, tile_y) = Self::pixel_to_tile(pixel_x, pixel_y);
        Self::tile_to_quadkey(tile_x, tile_y, precision)
    }

    /// Decode a quadkey to coordinates
    pub fn decode(quadkey: &str) -> (f64, f64, usize) {
        let (tile_x, tile_y, precision) = Self::quadkey_to_tile(quadkey);
        let (pixel_x, pixel_y) = Self::tile_to_pixel(tile_x, tile_y);
        let (latitude, longitude) = Self::pixel_to_coordinates(pixel_x, pixel_y, precision);

        (latitude, longitude, precision)
    }

    /// Find neighbors of a quadkey
    pub fn neighbors(quadkey: &str) -> Vec<String> {
        let (tile_x, tile_y, precision) = Self::quadkey_to_tile(quadkey);
        let mut neighbors = Vec::new();
        let permutation = [
            [-1, -1],
            [0, -1],
            [1, -1],
            [-1, 0],
            [0, 0],
            [1, 0],
            [-1, 1],
            [0, 1],
            [1, 1],
        ];
        for value in permutation.iter() {
            let [perm_x, perm_y] = *value;
            neighbors.push(Self::tile_to_quadkey(
                tile_x + perm_x,
                tile_y + perm_y,
                precision,
            ));
        }

        neighbors
    }

    pub fn clip(n: f64, min_value: f64, max_value: f64) -> f64 {
        n.max(min_value).min(max_value)
    }

    pub fn map_size(precision: usize) -> f64 {
        256.0 * (1 << precision) as f64
    }

    pub fn ground_resolution(latitude: f64, precision: usize) -> f64 {
        let latitude = Self::clip(latitude, Self::MIN_LATITUDE, Self::MAX_LATITUDE);
        (latitude.to_radians().cos() * 2.0 * PI * Self::EARTH_RADIUS) / Self::map_size(precision)
    }

    pub fn coordinates_to_pixel(latitude: f64, longitude: f64, precision: usize) -> (i32, i32) {
        let latitude = Self::clip(latitude, Self::MIN_LATITUDE, Self::MAX_LATITUDE);
        let longitude = Self::clip(longitude, Self::MIN_LONGITUDE, Self::MAX_LONGITUDE);

        let x = (longitude + 180.0) / 360.0;
        let sin_latitude = (latitude * PI / 180.0).sin();
        let y = 0.5 - ((1.0 + sin_latitude) / (1.0 - sin_latitude)).ln() / (4.0 * PI);

        let map_size = Self::map_size(precision);
        let pixel_x = Self::clip(x * map_size + 0.5, 0.0, map_size - 1.0) as i32;
        let pixel_y = Self::clip(y * map_size + 0.5, 0.0, map_size - 1.0) as i32;

        (pixel_x, pixel_y)
    }

    pub fn pixel_to_coordinates(pixel_x: i32, pixel_y: i32, precision: usize) -> (f64, f64) {
        let map_size = Self::map_size(precision);
        let x = (Self::clip(pixel_x as f64, 0.0, map_size - 1.0) / map_size) - 0.5;
        let y = 0.5 - (Self::clip(pixel_y as f64, 0.0, map_size - 1.0) / map_size);

        let latitude = 90.0 - 360.0 * ((-y * 2.0 * PI).exp().atan() / PI);
        let longitude = 360.0 * x;

        (latitude, longitude)
    }

    pub fn pixel_to_tile(pixel_x: i32, pixel_y: i32) -> (i32, i32) {
        let tile_x = pixel_x / 256;
        let tile_y = pixel_y / 256;

        (tile_x, tile_y)
    }

    pub fn tile_to_pixel(tile_x: i32, tile_y: i32) -> (i32, i32) {
        let pixel_x = tile_x * 256;
        let pixel_y = tile_y * 256;

        (pixel_x, pixel_y)
    }

    pub fn tile_to_quadkey(tile_x: i32, tile_y: i32, precision: usize) -> String {
        let mut quadkey = String::new();

        for i in (1..=precision).rev() {
            let mut digit = b'0';
            let mask = 1 << (i - 1);
            if (tile_x & mask) != 0 {
                digit += 1;
            }
            if (tile_y & mask) != 0 {
                digit += 2;
            }
            quadkey.push(digit as char);
        }

        quadkey
    }

    pub fn quadkey_to_tile(quadkey: &str) -> (i32, i32, usize) {
        let mut tile_x = 0;
        let mut tile_y = 0;
        let precision = quadkey.len();

        for (i, digit) in quadkey.chars().enumerate() {
            let mask = 1 << (precision - i - 1);
            match digit {
                '0' => {}
                '1' => tile_x |= mask,
                '2' => tile_y |= mask,
                '3' => {
                    tile_x |= mask;
                    tile_y |= mask;
                }
                _ => panic!("Invalid Quadkey digit sequence."),
            }
        }

        (tile_x, tile_y, precision)
    }
}
