use eframe::egui;
use egui::CtxRef;

use crate::forza;

#[derive(Default)]
pub struct PacketPanel;

impl PacketPanel {
    fn show_num<T: eframe::egui::emath::Numeric>(ui: &mut egui::Ui, title: &str, data: T) {
        ui.horizontal(|ui| {
            ui.label(title);
            let mut data = data;
            ui.add_enabled(false, egui::DragValue::new(&mut data));
        });
    }

    fn show_vec3<T: eframe::egui::emath::Numeric>(
        ui: &mut egui::Ui,
        title: &str,
        data: &forza::Vec3<T>,
    ) {
        ui.horizontal(|ui| {
            ui.label(title);
            let mut data = data.clone();
            ui.add_enabled(false, egui::DragValue::new(&mut data.x));
            ui.add_enabled(false, egui::DragValue::new(&mut data.y));
            ui.add_enabled(false, egui::DragValue::new(&mut data.z));
        });
    }

    fn show_tire_stat<T: eframe::egui::emath::Numeric>(
        ui: &mut egui::Ui,
        title: &str,
        data: &forza::TireStat<T>,
    ) {
        let mut data = data.clone();
        ui.separator();
        egui::Grid::new(title).show(ui, |ui| {
            ui.label(title);
            ui.label("Left");
            ui.label("Right");
            ui.end_row();

            ui.label("Front");
            ui.add_enabled(false, egui::DragValue::new(&mut data.front_left));
            ui.add_enabled(false, egui::DragValue::new(&mut data.front_right));
            ui.end_row();

            ui.label("Rear");
            ui.add_enabled(false, egui::DragValue::new(&mut data.rear_left));
            ui.add_enabled(false, egui::DragValue::new(&mut data.rear_right));
            ui.end_row();
        });
    }

    pub fn show(&mut self, ctx: &CtxRef, packet: Option<&forza::Packet>) {
        let default_pkt = forza::Packet::default();
        let pkt = packet.unwrap_or(&default_pkt);
        egui::SidePanel::right("Packet")
            .min_width(250.0)
            .show(ctx, |ui| {
                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        Self::show_num(ui, "Is race on: ", pkt.is_race_on);
                        Self::show_num(ui, "Timestamp: ", pkt.timestamp_ms);
                        Self::show_vec3(
                            ui,
                            "Engine: ",
                            &forza::Vec3 {
                                x: pkt.engine_idle_rpm,
                                y: pkt.engine_max_rpm,
                                z: pkt.current_engine_rpm,
                            },
                        );
                        Self::show_vec3(ui, "Acceleration: ", &pkt.acceleration);
                        Self::show_vec3(ui, "Velocity: ", &pkt.velocity);
                        Self::show_vec3(ui, "Angular velocity: ", &pkt.angular_velocity);
                        Self::show_vec3(ui, "Rotation: ", &pkt.rotation);

                        egui::CollapsingHeader::new("Car")
                            .default_open(true)
                            .show(ui, |ui| {
                                // Self::show_vec3(ui, "Position: ", &pkt.position);
                                Self::show_num(ui, "Speed: ", pkt.speed);
                                Self::show_num(ui, "Power: ", pkt.power);
                                Self::show_num(ui, "Torque: ", pkt.torque);

                                Self::show_num(ui, "Steer: ", pkt.steer);
                                Self::show_num(ui, "Accel.: ", pkt.accel);
                                Self::show_num(ui, "Gear: ", pkt.gear);
                                Self::show_num(ui, "Clutch: ", pkt.clutch);
                                Self::show_num(ui, "Brake: ", pkt.brake);
                                Self::show_num(ui, "Hand brake: ", pkt.hand_brake);

                                Self::show_num(ui, "Boost: ", pkt.boost);
                                Self::show_num(ui, "Fuel: ", pkt.fuel);
                                Self::show_num(ui, "Driving line: ", pkt.normalized_driving_line);
                                Self::show_num(ui, "Airbrake: ", pkt.normalized_aibrake_difference);

                                egui::CollapsingHeader::new("Vehicule")
                                    .default_open(true)
                                    .show(ui, |ui| {
                                        Self::show_num(ui, "Ordinal: ", pkt.car_ordinal);
                                        Self::show_num(ui, "Class: ", pkt.car_class);
                                        Self::show_num(ui, "PI: ", pkt.car_performance_index);
                                        Self::show_num(ui, "XWD: ", pkt.drivetrain_type);
                                        Self::show_num(ui, "Cylinders: ", pkt.num_cylinders);
                                    });
                            });

                        egui::CollapsingHeader::new("Race")
                            .default_open(true)
                            .show(ui, |ui| {
                                Self::show_num(ui, "Best lap: ", pkt.best_lap);
                                Self::show_num(ui, "Last lap: ", pkt.last_lap);
                                Self::show_num(ui, "Current lap: ", pkt.current_lap);
                                Self::show_num(ui, "Last number: ", pkt.lap_number);
                                Self::show_num(ui, "Race time: ", pkt.current_race_time);
                                Self::show_num(ui, "Position: ", pkt.race_position);
                                Self::show_num(ui, "Distance: ", pkt.distance_traveled);
                            });

                        egui::CollapsingHeader::new("Wheel/Tire")
                            .default_open(true)
                            .show(ui, |ui| {
                                Self::show_tire_stat(ui, "Rot.", &pkt.wheel_rotation_speed);
                                Self::show_tire_stat(ui, "Temp.", &pkt.tire_temp);
                                Self::show_tire_stat(ui, "Slip ratio", &pkt.tire_slip_ratio);
                                Self::show_tire_stat(ui, "Slip angle", &pkt.tire_slip_angle);
                                Self::show_tire_stat(ui, "Slip combined", &pkt.tire_combined_slip);
                            });

                        egui::CollapsingHeader::new("Suspensions")
                            .default_open(true)
                            .show(ui, |ui| {
                                Self::show_tire_stat(
                                    ui,
                                    "Normalized",
                                    &pkt.normalized_suspension_travel,
                                );
                                Self::show_tire_stat(ui, "Travel", &pkt.suspension_travel);
                            });

                        egui::CollapsingHeader::new("Surface")
                            .default_open(true)
                            .show(ui, |ui| {
                                Self::show_tire_stat(ui, "Rumble", &pkt.surface_rumble);
                                Self::show_tire_stat(ui, "Strip", &pkt.wheel_on_rumble_strip);
                                Self::show_tire_stat(ui, "Puddle", &pkt.wheel_in_puddle_depth);
                            });
                    });
            });
    }
}
