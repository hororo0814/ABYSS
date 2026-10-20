pub struct ScreenScale {
    scale: f32,
    offset_x: f32,
    offset_y: f32,
}

impl ScreenScale {
    pub fn new(width: f32, height: f32) -> Self {
        let min_size = f32::min(width / 1400.0, height / 850.0);
        Self {
            scale: min_size,
            offset_x: (width - 1400.0 * min_size) / 2.0,
            offset_y: (height - 850.0 * min_size) / 2.0,
        }
    }

    pub fn transform_x(&self, x: f32) -> f32 {
        x * self.scale + self.offset_x
    }

    pub fn transform_y(&self, y: f32) -> f32 {
        y * self.scale + self.offset_y
    }

    pub fn transform_size(&self, size: f32) -> f32 {
        size * self.scale
    }

    pub fn inverse_x(&self, x: f32) -> f32 {
        (x - self.offset_x) / self.scale
    }

    pub fn inverse_y(&self, y: f32) -> f32 {
        (y - self.offset_y) / self.scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_is_one_for_reference_size() {
        let screen = ScreenScale::new(1400.0, 850.0);

        assert_eq!(screen.scale, 1.0);
        assert_eq!(screen.offset_x, 0.0);
        assert_eq!(screen.offset_y, 0.0);
    }

    #[test]
    fn scale_fits_inside_smaller_window() {
        let screen = ScreenScale::new(1200.0, 700.0);

        let expected_scale = 700.0 / 850.0;
        let expected_offset_x = (1200.0 - 1400.0 * expected_scale) / 2.0;

        assert!((screen.scale - expected_scale).abs() < 0.0001);
        assert!((screen.offset_x - expected_offset_x).abs() < 0.0001);
        assert!(screen.offset_y.abs() < 0.0001);
    }

    #[test]
    fn scale_preserves_aspect_ratio_when_window_is_wider() {
        let screen = ScreenScale::new(1600.0, 850.0);

        assert_eq!(screen.scale, 1.0);
        assert_eq!(screen.offset_x, 100.0);
        assert_eq!(screen.offset_y, 0.0);
    }

    #[test]
    fn transform_x_keeps_coordinate_at_reference_size() {
        let screen = ScreenScale::new(1400.0, 850.0);

        assert_eq!(screen.transform_x(100.0), 100.0);
    }

    #[test]
    fn transform_x_applies_scale_and_offset() {
        let screen = ScreenScale::new(1200.0, 850.0);

        // scale = 1200 / 1400、offset_x = 0
        let expected = 100.0 * (1200.0 / 1400.0);

        assert!((screen.transform_x(100.0) - expected).abs() < 0.0001);
    }

    #[test]
    fn transform_y_applies_scale_and_offset() {
        let screen = ScreenScale::new(1400.0, 700.0);

        // scale = 700 / 850、offset_y = 0
        let expected = 200.0 * (700.0 / 850.0);

        assert!((screen.transform_y(200.0) - expected).abs() < 0.0001);
    }

    #[test]
    fn transform_size_keeps_size_at_reference_scale() {
        let screen = ScreenScale::new(1400.0, 850.0);

        assert_eq!(screen.transform_size(100.0), 100.0);
    }

    #[test]
    fn transform_size_scales_down_with_window() {
        let screen = ScreenScale::new(1200.0, 850.0);

        let expected = 100.0 * (1200.0 / 1400.0);

        assert!((screen.transform_size(100.0) - expected).abs() < 0.0001);
    }

    #[test]
    fn inverse_x_restores_original_coordinate() {
        let screen = ScreenScale::new(1200.0, 850.0);
        let original_x = 100.0;

        let screen_x = screen.transform_x(original_x);
        let restored_x = screen.inverse_x(screen_x);

        assert!((restored_x - original_x).abs() < 0.0001);
    }

    #[test]
    fn inverse_y_restores_original_coordinate() {
        let screen = ScreenScale::new(1400.0, 700.0);
        let original_y = 200.0;

        let screen_y = screen.transform_y(original_y);
        let restored_y = screen.inverse_y(screen_y);

        assert!((restored_y - original_y).abs() < 0.0001);
    }
}
