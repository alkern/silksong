use bevy::prelude::*;
use std::f32::consts::PI;

//TODO model scale
pub struct Scale(u8);

impl Scale {
    pub fn size(&self) -> u8 {
        self.0
    }
}

pub fn calculate_scale_position_by_angle(center: &Vec2, point: &Vec2, scale: &Scale) -> u8 {
    let direction = point - center;
    let (angle_x, angle_y) = match direction.try_normalize() {
        None => (0.0, 0.0),
        Some(direction) => (direction.x.acos(), direction.y.asin()),
    };

    let part = (PI * 2.0) / scale.size() as f32;

    //TODO refactor
    let index = (angle_x / part).ceil() as u8;
    if angle_y >= 0.0 {
        index
    } else {
        scale.size() - ((scale.size() / 2) - index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_points_on_normal_sized_scale() {
        let s = Scale(8);
        let c = Vec2::ZERO;
        // 0°
        assert_eq!(
            0,
            calculate_scale_position_by_angle(&c, &Vec2::new(0.0, 0.0), &s)
        );
        // 12°
        assert_eq!(
            1,
            calculate_scale_position_by_angle(&c, &Vec2::new(0.9781476, 0.2079117), &s)
        );
        // 45°
        assert_eq!(
            1,
            calculate_scale_position_by_angle(&c, &Vec2::new(1.0, 1.0), &s)
        );
        // > 45°
        assert_eq!(
            2,
            calculate_scale_position_by_angle(&c, &Vec2::new(1.0, 1.1), &s)
        );
        // 90°
        assert_eq!(
            2,
            calculate_scale_position_by_angle(&c, &Vec2::new(0.0, 1.0), &s)
        );
        // 120°
        assert_eq!(
            3,
            calculate_scale_position_by_angle(&c, &Vec2::new(-0.5, 0.8660254), &s)
        );
        // 130°
        assert_eq!(
            3,
            calculate_scale_position_by_angle(&c, &Vec2::new(-0.6427876, 0.7660444), &s)
        );
        // 140°
        assert_eq!(
            4,
            calculate_scale_position_by_angle(&c, &Vec2::new(-0.7660444, 0.6427876), &s)
        );
        // 270°
        assert_eq!(
            6,
            calculate_scale_position_by_angle(&c, &Vec2::new(0.0, -1.0), &s)
        );
    }

    #[test]
    fn test_calculate_90deg() {
        // 90°
        let center = Vec2::new(1.0, 1.0);
        let point = Vec2::new(1.0, 2.0);

        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(1))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(2))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(3))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(4))
        );
        assert_eq!(
            2u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(5))
        );
        assert_eq!(
            2u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(6))
        );
        assert_eq!(
            2u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(7))
        );
        assert_eq!(
            2u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(8))
        );
        assert_eq!(
            3u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(9))
        );
        assert_eq!(
            3u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(10))
        );
    }
    #[test]
    fn test_calculate_45deg() {
        // 90°
        let center = Vec2::new(1.0, 1.0);
        let point = Vec2::new(2.0, 2.0);

        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(1))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(2))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(3))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(4))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(5))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(6))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(7))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(8))
        );
        assert_eq!(
            2u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(9))
        );
        assert_eq!(
            2u8,
            calculate_scale_position_by_angle(&center, &point, &Scale(10))
        );
    }
}
