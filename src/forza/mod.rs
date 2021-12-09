use std::{
    collections::LinkedList,
    intrinsics::transmute,
    io::{Read, Write},
    mem::size_of,
    net::UdpSocket,
    sync::mpsc::{Iter, Receiver, TryIter},
    thread::JoinHandle,
};

#[repr(C)]
#[derive(Debug, Default)]
pub struct Packet {
    pub is_race_on: i32,   // = 1 when race is on. = 0 when in menus/race stopped
    pub timestamp_ms: u32, //Can overflow to 0 eventually
    pub engine_max_rpm: f32,
    pub engine_idle_rpm: f32,
    pub current_engine_rpm: f32,
    pub acceleration_x: f32, //In the car's local space, X = right, Y = up, Z = forward
    pub acceleration_y: f32,
    pub acceleration_z: f32,
    pub velocity_x: f32, //In the car's local space, X = right, Y = up, Z = forward
    pub velocity_y: f32,
    pub velocity_z: f32,
    pub angular_velocity_x: f32, //In the car's local space, X = pitch, Y = yaw, Z = roll
    pub angular_velocity_y: f32,
    pub angular_velocity_z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub normalized_suspension_travel_front_left: f32, // Suspension travel normalized: 0.0f = max stretch, 1.0 = max compression
    pub normalized_suspension_travel_front_right: f32,
    pub normalized_suspension_travel_rear_left: f32,
    pub normalized_suspension_travel_rear_right: f32,
    pub tire_slip_ratio_front_left: f32, // Tire normalized slip ratio, = 0 means 100% grip and |ratio| > 1.0 means loss of grip.
    pub tire_slip_ratio_front_right: f32,
    pub tire_slip_ratio_rear_left: f32,
    pub tire_slip_ratio_rear_right: f32,
    pub wheel_rotation_speed_front_left: f32, // Wheel rotation speed radians/sec.
    pub wheel_rotation_speed_front_right: f32,
    pub wheel_rotation_speed_rear_left: f32,
    pub wheel_rotation_speed_rear_right: f32,
    pub wheel_on_rumble_strip_front_left: i32, // = 1 when wheel is on rumble strip, = 0 when off.
    pub wheel_on_rumble_strip_front_right: i32,
    pub wheel_on_rumble_strip_rear_left: i32,
    pub wheel_on_rumble_strip_rear_right: i32,
    pub wheel_in_puddle_depth_front_left: f32, // = from 0 to 1, where 1 is the deepest puddle
    pub wheel_in_puddle_depth_front_right: f32,
    pub wheel_in_puddle_depth_rear_left: f32,
    pub wheel_in_puddle_depth_rear_right: f32,
    pub surface_rumble_front_left: f32, // Non-dimensional surface rumble values passed to controller force feedback
    pub surface_rumble_front_right: f32,
    pub surface_rumble_rear_left: f32,
    pub surface_rumble_rear_right: f32,
    pub tire_slip_angle_front_left: f32, // Tire normalized slip angle, = 0 means 100% grip and |angle| > 1.0 means loss of grip.
    pub tire_slip_angle_front_right: f32,
    pub tire_slip_angle_rear_left: f32,
    pub tire_slip_angle_rear_right: f32,
    pub tire_combined_slip_front_left: f32, // Tire normalized combined slip, = 0 means 100% grip and |slip| > 1.0 means loss of grip.
    pub tire_combined_slip_front_right: f32,
    pub tire_combined_slip_rear_left: f32,
    pub tire_combined_slip_rear_right: f32,
    pub suspension_travel_meters_front_left: f32, // Actual suspension travel in meters
    pub suspension_travel_meters_front_right: f32,
    pub suspension_travel_meters_rear_left: f32,
    pub suspension_travel_meters_rear_right: f32,
    pub car_ordinal: i32,              //Unique ID of the car make/model
    pub car_class: i32, //Between 0 (D -- worst cars) and 7 (X class -- best cars) inclusive
    pub car_performance_index: i32, //Between 100 (slowest car) and 999 (fastest car) inclusive
    pub drivetrain_type: i32, //Corresponds to EDrivetrainType, 0 = FWD, 1 = RWD, 2: = AWD
    pub num_cylinders: i32, //Number of cylinders in the engine
    pub horizon_placeholder: [u8; 12], // unknown FH4 values
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub speed: f32,  // meters per second
    pub power: f32,  // watts
    pub torque: f32, // newton meter
    pub tire_temp_front_left: f32,
    pub tire_temp_front_right: f32,
    pub tire_temp_rear_left: f32,
    pub tire_temp_rear_right: f32,
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

type PacketRaw = [u8; size_of::<Packet>()];
pub type PacketVec = std::vec::Vec<Packet>;

#[derive(Debug, PartialEq)]
pub enum GameMode {
    FreeRoam,
    Race,
}

impl Packet {
    pub fn game_mode(&self) -> GameMode {
        match self.race_position {
            0 => GameMode::FreeRoam,
            _ => GameMode::Race,
        }
    }

    pub fn as_buf<'a>(&'a self) -> &'a PacketRaw {
        unsafe { transmute::<&Packet, &PacketRaw>(self) }
    }

    pub fn as_buf_mut<'a>(&'a mut self) -> &'a mut PacketRaw {
        unsafe { transmute::<&mut Packet, &mut PacketRaw>(self) }
    }
}

pub fn write_packets<'a>(
    packets: impl Iterator<Item = &'a Packet>,
    output: &mut std::fs::File,
) -> std::io::Result<()> {
    let mut packet_count = 0;
    for packet in packets {
        packet_count += 1;
        output.write_all(packet.as_buf())?;
    }
    println!("Packets written: {}", packet_count);
    Ok(())
}

pub fn read_packets(input: &mut std::fs::File) -> std::io::Result<PacketVec> {
    let input_len = input.metadata()?.len() as usize;
    if input_len % size_of::<Packet>() != 0 {
        panic!("Invalid size file.");
    }

    let packets_count = input_len / size_of::<Packet>();
    let mut packets = PacketVec::with_capacity(packets_count);
    for _ in 0..packets_count {
        let mut packet = Packet::default();
        input.read_exact(packet.as_buf_mut())?;
        packets.push(packet);
    }

    println!("Packets read: {}", packets.len());
    Ok(packets)
}

pub fn chunkify(packets: PacketVec) -> LinkedList<Chunk> {
    let mut chunks = LinkedList::from([Chunk::new()]);
    for p in packets {
        if p.current_race_time == 0.0 {
            match (p.game_mode(), chunks.back().unwrap().game_mode()) {
                (GameMode::FreeRoam, GameMode::FreeRoam) => {}
                _ => chunks.push_back(Chunk::new()),
            }
        }

        chunks.back_mut().unwrap().push(p);
    }
    return chunks;
}

pub type Chunks = LinkedList<Chunk>;

pub struct Socket {
    thread: JoinHandle<()>,
    receiver: Receiver<Packet>,
}

impl Socket {
    pub fn new(addr: &str) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        let socket = UdpSocket::bind(addr).expect("couldn't bind to address");
        println!("Listening on {:?}...", socket.local_addr().unwrap());

        let thread = std::thread::spawn(move || {
            let mut last_packet_timestamp = 0u32;
            loop {
                let mut packet = Packet::default();
                socket.recv_from(packet.as_buf_mut()).unwrap();

                if packet.is_race_on == 0 {
                    continue;
                }
                if packet.timestamp_ms == last_packet_timestamp {
                    continue;
                }

                last_packet_timestamp = packet.timestamp_ms;
                sender.send(packet).unwrap();
            }
        });

        Socket { thread, receiver }
    }

    pub fn iter(&self) -> Iter<'_, Packet> {
        self.receiver.iter()
    }

    pub fn try_iter(&self) -> TryIter<'_, Packet> {
        self.receiver.try_iter()
    }
}

pub struct Chunk {
    pub packets: PacketVec,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            packets: PacketVec::with_capacity(5 * 60 * 60),
        }
    }

    pub fn with_packets(packets: PacketVec) -> Self {
        Chunk { packets }
    }

    pub fn finalize(&mut self) {
        if !self.is_empty() {
            self.packets.shrink_to_fit();
        }
    }

    pub fn is_empty(&self) -> bool {
        self.packets.is_empty()
    }

    pub fn game_mode(&self) -> GameMode {
        self.packets
            .first()
            .map(|p| p.game_mode())
            .unwrap_or(GameMode::FreeRoam)
    }

    pub fn lap_count(&self) -> u16 {
        self.packets.iter().map(|p| p.lap_number).max().unwrap_or(0)
    }

    pub fn lap_packets(&self, lap_id: u16) -> impl Iterator<Item = &Packet> + '_ {
        self.packets.iter().filter(move |p| p.lap_number == lap_id)
    }

    pub fn push(&mut self, packet: Packet) {
        self.packets.push(packet)
    }
}
