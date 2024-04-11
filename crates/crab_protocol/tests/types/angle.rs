use crab_protocol::types::angle::Angle;

#[test]
fn convert_angle() {
    assert_eq!(Angle::from_degrees(0.0).to_degrees(), 0.0);
}
