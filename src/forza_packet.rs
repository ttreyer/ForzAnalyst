use std::{intrinsics::transmute, mem::size_of};

#[repr(C)]
#[derive(Debug, Default)]
pub struct ForzaPacket {
    is_race_on: i32,   // = 1 when race is on. = 0 when in menus/race stopped
    timestamp_ms: u32, //Can overflow to 0 eventually
    engine_max_rpm: f32,
    engine_idle_rpm: f32,
    current_engine_rpm: f32,
    acceleration_x: f32, //In the car's local space, X = right, Y = up, Z = forward
    acceleration_y: f32,
    acceleration_z: f32,
    velocity_x: f32, //In the car's local space, X = right, Y = up, Z = forward
    velocity_y: f32,
    velocity_z: f32,
    angular_velocity_x: f32, //In the car's local space, X = pitch, Y = yaw, Z = roll
    angular_velocity_y: f32,
    angular_velocity_z: f32,
    yaw: f32,
    pitch: f32,
    roll: f32,
    normalized_suspension_travel_front_left: f32, // Suspension travel normalized: 0.0f = max stretch, 1.0 = max compression
    normalized_suspension_travel_front_right: f32,
    normalized_suspension_travel_rear_left: f32,
    normalized_suspension_travel_rear_right: f32,
    tire_slip_ratio_front_left: f32, // Tire normalized slip ratio, = 0 means 100% grip and |ratio| > 1.0 means loss of grip.
    tire_slip_ratio_front_right: f32,
    tire_slip_ratio_rear_left: f32,
    tire_slip_ratio_rear_right: f32,
    wheel_rotation_speed_front_left: f32, // Wheel rotation speed radians/sec.
    wheel_rotation_speed_front_right: f32,
    wheel_rotation_speed_rear_left: f32,
    wheel_rotation_speed_rear_right: f32,
    wheel_on_rumble_strip_front_left: i32, // = 1 when wheel is on rumble strip, = 0 when off.
    wheel_on_rumble_strip_front_right: i32,
    wheel_on_rumble_strip_rear_left: i32,
    wheel_on_rumble_strip_rear_right: i32,
    wheel_in_puddle_depth_front_left: f32, // = from 0 to 1, where 1 is the deepest puddle
    wheel_in_puddle_depth_front_right: f32,
    wheel_in_puddle_depth_rear_left: f32,
    wheel_in_puddle_depth_rear_right: f32,
    surface_rumble_front_left: f32, // Non-dimensional surface rumble values passed to controller force feedback
    surface_rumble_front_right: f32,
    surface_rumble_rear_left: f32,
    surface_rumble_rear_right: f32,
    tire_slip_angle_front_left: f32, // Tire normalized slip angle, = 0 means 100% grip and |angle| > 1.0 means loss of grip.
    tire_slip_angle_front_right: f32,
    tire_slip_angle_rear_left: f32,
    tire_slip_angle_rear_right: f32,
    tire_combined_slip_front_left: f32, // Tire normalized combined slip, = 0 means 100% grip and |slip| > 1.0 means loss of grip.
    tire_combined_slip_front_right: f32,
    tire_combined_slip_rear_left: f32,
    tire_combined_slip_rear_right: f32,
    suspension_travel_meters_front_left: f32, // Actual suspension travel in meters
    suspension_travel_meters_front_right: f32,
    suspension_travel_meters_rear_left: f32,
    suspension_travel_meters_rear_right: f32,
    car_ordinal: i32,              //Unique ID of the car make/model
    car_class: i32, //Between 0 (D -- worst cars) and 7 (X class -- best cars) inclusive
    car_performance_index: i32, //Between 100 (slowest car) and 999 (fastest car) inclusive
    drivetrain_type: i32, //Corresponds to EDrivetrainType, 0 = FWD, 1 = RWD, 2: = AWD
    num_cylinders: i32, //Number of cylinders in the engine
    horizon_placeholder: [u8; 12], // unknown FH4 values
    position_x: f32,
    position_y: f32,
    position_z: f32,
    speed: f32,  // meters per second
    power: f32,  // watts
    torque: f32, // newton meter
    tire_temp_front_left: f32,
    tire_temp_front_right: f32,
    tire_temp_rear_left: f32,
    tire_temp_rear_right: f32,
    boost: f32,
    fuel: f32,
    distance_traveled: f32,
    best_lap: f32,
    last_lap: f32,
    current_lap: f32,
    current_race_time: f32,
    lap_number: u16,
    race_position: u8,
    accel: u8,
    brake: u8,
    clutch: u8,
    hand_brake: u8,
    gear: u8,
    steer: i8,
    normalized_driving_line: i8,
    normalized_aibrake_difference: i8,
}

type ForzaPacketRaw = [u8; size_of::<ForzaPacket>()];

impl ForzaPacket {
    pub fn as_buf<'a>(&'a mut self) -> &'a mut ForzaPacketRaw {
        unsafe { transmute::<&mut ForzaPacket, &mut ForzaPacketRaw>(self) }
    }
}
