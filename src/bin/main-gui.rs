// Alias the backend to something less mouthful
use egui_sdl2_gl as egui_backend;

use egui::Color32;
use egui_backend::sdl2::video::GLProfile;
use egui_backend::{egui, sdl2};
use egui_backend::{sdl2::event::Event, DpiScaling, ShaderVersion};
use forzanalyst::gui::control_panel::ControlPanel;
use sdl2::video::SwapInterval;
use std::time::Instant;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    let mut control_panel = ControlPanel::new();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    // On linux, OpenGL ES Mesa driver 22.0.0+ can be used like so:
    // gl_attr.set_context_profile(GLProfile::GLES);

    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(0);

    let window = video_subsystem
        .window(
            "Demo: Egui backend for SDL2 + GL",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // Create a window context
    let _ctx = window.gl_create_context().unwrap();
    window
        .subsystem()
        .gl_set_swap_interval(SwapInterval::VSync)
        .unwrap();

    // Init egui stuff
    // let shader_ver = ShaderVersion::Default;
    // On linux use GLES SL 100+, like so:
    let (mut painter, mut egui_state) =
        egui_backend::with_sdl2(&window, ShaderVersion::Default, DpiScaling::Custom(3.0));
    let mut egui_ctx = egui::CtxRef::default();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut test_str: String =
        "A text box to write in. Cut, copy, paste commands are available.".to_owned();

    let mut quit = false;
    let mut slider = 0.0;

    let start_time = Instant::now();

    'running: loop {
        egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_state.input.take());

        egui::Window::new("Forza").show(&egui_ctx, |ui| {
            ui.label(" ");
            ui.text_edit_multiline(&mut test_str);
            ui.label(" ");
            ui.add(egui::Slider::new(&mut slider, 0.0..=50.0).text("Slider"));
            ui.label(" ");
            ui.separator();
            if ui.button("Quit?").clicked() {
                quit = true;
            }
        });

        control_panel.render(&egui_ctx);

        let (egui_output, paint_cmds) = egui_ctx.end_frame();
        // Process ouput
        egui_state.process_output(&window, &egui_output);

        let paint_jobs = egui_ctx.tessellate(paint_cmds);

        if !egui_output.needs_repaint {
            match event_pump.wait_event() {
                Event::Quit { .. } => break 'running,
                event => {
                    // Process input event
                    egui_state.process_input(&window, event, &mut painter);
                }
            }
        } else {
            painter.paint_jobs(Some(Color32::LIGHT_GRAY), paint_jobs, &egui_ctx.texture());
            window.gl_swap_window();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {
                        // Process input event
                        egui_state.process_input(&window, event, &mut painter);
                    }
                }
            }
        }

        if quit {
            break;
        }
    }
}
