use std::io::{Read, Write};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TireStat<T> {
    pub front_left: T,
    pub front_right: T,
    pub rear_left: T,
    pub rear_right: T,
}

pub type PacketRaw = [u8; std::mem::size_of::<Packet>()];
pub type PacketVec = std::vec::Vec<Packet>;

#[repr(C)]
#[derive(Debug, Default)]
pub struct Packet {
    pub is_race_on: i32,   // = 1 when race is on. = 0 when in menus/race stopped
    pub timestamp_ms: u32, //Can overflow to 0 eventually
    pub engine_max_rpm: f32,
    pub engine_idle_rpm: f32,
    pub current_engine_rpm: f32,
    pub acceleration: Vec3<f32>, //In the car's local space, X = right, Y = up, Z = forward
    pub velocity: Vec3<f32>,     //In the car's local space, X = right, Y = up, Z = forward
    pub angular_velocity: Vec3<f32>, //In the car's local space, X = pitch, Y = yaw, Z = roll
    pub rotation: Vec3<f32>,     // X = yaw, Y = pitch, Z = roll
    pub normalized_suspension_travel: TireStat<f32>, // Suspension travel normalized: 0.0f = max stretch, 1.0 = max compression
    pub tire_slip_ratio: TireStat<f32>, // Tire normalized slip ratio, = 0 means 100% grip and |ratio| > 1.0 means loss of grip.
    pub wheel_rotation_speed: TireStat<f32>, // Wheel rotation speed radians/sec.
    pub wheel_on_rumble_strip: TireStat<i32>, // = 1 when wheel is on rumble strip, = 0 when off.
    pub wheel_in_puddle_depth: TireStat<f32>, // = from 0 to 1, where 1 is the deepest puddle
    pub surface_rumble: TireStat<f32>, // Non-dimensional surface rumble values passed to controller force feedback
    pub tire_slip_angle: TireStat<f32>, // Tire normalized slip angle, = 0 means 100% grip and |angle| > 1.0 means loss of grip.
    pub tire_combined_slip: TireStat<f32>, // Tire normalized combined slip, = 0 means 100% grip and |slip| > 1.0 means loss of grip.
    pub suspension_travel: TireStat<f32>,  // Actual suspension travel in meters
    pub car_ordinal: i32,                  //Unique ID of the car make/model
    pub car_class: i32, //Between 0 (D -- worst cars) and 7 (X class -- best cars) inclusive
    pub car_performance_index: i32, //Between 100 (slowest car) and 999 (fastest car) inclusive
    pub drivetrain_type: i32, //Corresponds to EDrivetrainType, 0 = FWD, 1 = RWD, 2: = AWD
    pub num_cylinders: i32, //Number of cylinders in the engine
    pub horizon_placeholder: [u8; 12], // unknown FH4 values
    pub position: Vec3<f32>,
    pub speed: f32,  // meters per second
    pub power: f32,  // watts
    pub torque: f32, // newton meter
    pub tire_temp: TireStat<f32>,
    pub boost: f32,
    pub fuel: f32,
    pub distance_traveled: f32,
    pub best_lap: f32,
    pub last_lap: f32,
    pub current_lap: f32,
    pub current_race_time: f32,
    pub lap_number: u16,
    pub race_position: u8,
    pub accel: u8,
    pub brake: u8,
    pub clutch: u8,
    pub hand_brake: u8,
    pub gear: u8,
    pub steer: i8,
    pub normalized_driving_line: i8,
    pub normalized_aibrake_difference: i8,
}

#[derive(Debug, PartialEq)]
pub enum GameMode {
    FreeRoam,
    Race,
    None,
}

impl Packet {
    pub fn game_mode(&self) -> GameMode {
        match self.race_position {
            0 => GameMode::FreeRoam,
            _ => GameMode::Race,
        }
    }

    pub fn position(&self) -> (f32, f32) {
        (self.position.x, self.position.z)
    }

    pub fn as_buf(&'_ self) -> &'_ PacketRaw {
        unsafe { std::mem::transmute::<&Packet, &PacketRaw>(self) }
    }

    pub fn as_buf_mut(&'_ mut self) -> &'_ mut PacketRaw {
        unsafe { std::mem::transmute::<&mut Packet, &mut PacketRaw>(self) }
    }
}

pub fn write_packets<'a>(
    packets: impl Iterator<Item = &'a Packet>,
    output: &mut std::fs::File,
) -> std::io::Result<()> {
    let output = std::io::BufWriter::new(output);
    let mut output = zstd::Encoder::new(output, 0)?;

    let mut packet_count = 0;
    for packet in packets {
        packet_count += 1;
        output.write_all(packet.as_buf())?;
    }
    output.finish().and_then(|mut w| w.flush())?;

    println!("Packets written: {}", packet_count);
    Ok(())
}

pub fn read_packets(input: &mut std::fs::File) -> std::io::Result<PacketVec> {
    let input = std::io::BufReader::new(input);
    let mut input = zstd::Decoder::new(input)?;

    let mut packets = PacketVec::with_capacity(1024);
    loop {
        let mut packet = Packet::default();
        if let Err(error) = input.read_exact(packet.as_buf_mut()) {
            match error.kind() {
                std::io::ErrorKind::UnexpectedEof => break,
                _ => return Err(error),
            }
        };
        packets.push(packet);
    }

    println!("Packets read: {}", packets.len());
    Ok(packets)
}
