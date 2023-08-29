#[cfg(test)]
mod tests {
    use assert_float_eq::*;
    use geo_quadkey_rs::Quadkey;

    #[test]
    fn test_encode() {
        let latitude = 35.680725113703886;
        let longitude = -139.76669311523438;
        let precision = 16;

        let encoded = Quadkey::encode(latitude, longitude, precision);
        assert_eq!(encoded, "0221130032013320");
    }

    #[test]
    fn test_decode() {
        let quadkey = "0221130032013320";

        let (latitude, longitude, precision) = Quadkey::decode(quadkey);
        assert_float_relative_eq!(latitude, 35.680725113703886, 1e-4);
        assert_float_relative_eq!(longitude, -139.76669311523438, 1e-4);
        assert_eq!(precision, 16);
    }

    #[test]
    fn test_neighbors() {
        let quadkey = "0221130032013320";

        let neighbors = Quadkey::neighbors(quadkey);
        assert_eq!(neighbors, vec![
            "0221130032013213",
            "0221130032013302",
            "0221130032013303",
            "0221130032013231",
            "0221130032013320",
            "0221130032013321",
            "0221130032013233",
            "0221130032013322",
            "0221130032013323",
        ]);
    }

    #[test]
    fn test_clip() {
        let n = 10.5;
        let min_value = 5.0;
        let max_value = 15.0;

        let clipped = Quadkey::clip(n, min_value, max_value);
        assert_eq!(clipped, 10.5);
    }

    #[test]
    fn test_map_size() {
        let precision = 10;

        let size = Quadkey::map_size(precision);
        assert_eq!(size, 262144.0);
    }

    #[test]
    fn test_ground_resolution() {
        let latitude = 35.680725113703886;
        let precision = 10;

        let resolution = Quadkey::ground_resolution(latitude, precision);
        assert_float_relative_eq!(resolution, 124.17571152149304, 1e-4);
    }

    #[test]
    fn test_coordinates_to_pixel() {
        let latitude = 35.680725113703886;
        let longitude = -139.76669311523438;
        let precision = 10;

        let (pixel_x, pixel_y) = Quadkey::coordinates_to_pixel(latitude, longitude, precision);
        assert_eq!(pixel_x, 29297);
        assert_eq!(pixel_y, 103227);
    }

    #[test]
    fn test_pixel_to_coordinates() {
        let pixel_x = 29297;
        let pixel_y = 103227;
        let precision = 10;

        let (latitude, longitude) = Quadkey::pixel_to_coordinates(pixel_x, pixel_y, precision);
        assert_float_relative_eq!(latitude, 35.680725113703886, 1e-4);
        assert_float_relative_eq!(longitude, -139.76669311523438, 1e-4);
    }

    #[test]
    fn test_pixel_to_tile() {
        let pixel_x = 29297;
        let pixel_y = 103227;

        let (tile_x, tile_y) = Quadkey::pixel_to_tile(pixel_x, pixel_y);
        assert_eq!(tile_x, 114);
        assert_eq!(tile_y, 403);
    }

    #[test]
    fn test_tile_to_pixel() {
        let tile_x = 114;
        let tile_y = 403;

        let (pixel_x, pixel_y) = Quadkey::tile_to_pixel(tile_x, tile_y);
        assert_eq!(pixel_x, 29184);
        assert_eq!(pixel_y, 103168);
    }

    #[test]
    fn test_tile_to_quadkey() {
        let tile_x = 114;
        let tile_y = 403;
        let precision = 10;

        let quadkey = Quadkey::tile_to_quadkey(tile_x, tile_y, precision);
        assert_eq!(quadkey, "0221130032");
    }

    #[test]
    fn test_quadkey_to_tile() {
        let quadkey = "0221130032";

        let (tile_x, tile_y, precision) = Quadkey::quadkey_to_tile(quadkey);
        assert_eq!(tile_x, 114);
        assert_eq!(tile_y, 403);
        assert_eq!(precision, 10);
    }
}
