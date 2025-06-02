use crate::music::model::Scale;
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn calculate_scale_position_by_angle(center: &Vec2, point: &Vec2, scale: &impl Scale) -> u8 {
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

    struct MockScale(u8);
    impl Scale for MockScale {
        fn steps(&self) -> Vec<crate::music::model::Step> {
            todo!()
        }

        fn size(&self) -> u8 {
            self.0
        }

        fn root(&self) -> &crate::music::model::Note {
            todo!()
        }
    }

    #[test]
    fn test_multiple_points_on_normal_sized_scale() {
        let s = MockScale(8);
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
            calculate_scale_position_by_angle(&center, &point, &MockScale(1))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(2))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(3))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(4))
        );
        assert_eq!(
            2u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(5))
        );
        assert_eq!(
            2u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(6))
        );
        assert_eq!(
            2u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(7))
        );
        assert_eq!(
            2u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(8))
        );
        assert_eq!(
            3u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(9))
        );
        assert_eq!(
            3u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(10))
        );
    }
    #[test]
    fn test_calculate_45deg() {
        // 90°
        let center = Vec2::new(1.0, 1.0);
        let point = Vec2::new(2.0, 2.0);

        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(1))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(2))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(3))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(4))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(5))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(6))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(7))
        );
        assert_eq!(
            1u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(8))
        );
        assert_eq!(
            2u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(9))
        );
        assert_eq!(
            2u8,
            calculate_scale_position_by_angle(&center, &point, &MockScale(10))
        );
    }
}
