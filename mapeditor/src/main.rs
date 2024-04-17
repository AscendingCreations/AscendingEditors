#![allow(dead_code, clippy::collapsible_match, unused_imports)]
use backtrace::Backtrace;
use camera::{
    controls::{Controls, FlatControls, FlatSettings},
    Projection,
};
use cosmic_text::{Attrs, Metrics};
use graphics::*;
use input::{Bindings, FrameTime, InputHandler, Key};
use log::{error, info, warn, Level, LevelFilter, Metadata, Record};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{prelude::*, Read, Write},
    iter, panic,
    sync::Arc,
    time::{Duration, Instant},
};
use wgpu::{Backends, Dx12Compiler, InstanceDescriptor, InstanceFlags};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, WindowButtons},
};

mod collection;
mod config;
mod editor_input;
mod gfx_collection;
mod interface;
mod map;
mod map_data;
mod renderer;
mod resource;
mod tileset;

use collection::*;
use config::*;
use editor_input::{dialog_input::*, *};
use gfx_collection::*;
use interface::*;
use map::*;
use map_data::*;
use renderer::*;
use resource::*;
use tileset::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
enum Axis {
    Forward,
    Sideward,
    Yaw,
    Pitch,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
enum Action {
    None,
}

// creates a static global logger type for setting the logger
static MY_LOGGER: MyLogger = MyLogger(Level::Debug);

struct MyLogger(pub Level);

impl log::Log for MyLogger {
    // checks if it can log these types of events.
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.0
    }

    // This logs to a panic file. This is so we can see
    // Errors and such if a program crashes in full render mode.
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("{} - {}\n", record.level(), record.args());
            println!("{}", &msg);

            let mut file = match File::options()
                .append(true)
                .create(true)
                .open("map_editor_log.txt")
            {
                Ok(v) => v,
                Err(_) => return,
            };

            let _ = file.write(msg.as_bytes());
        }
    }
    fn flush(&self) {}
}

#[tokio::main]
async fn main() -> Result<(), GraphicsError> {
    // Create logger to output to a File
    log::set_logger(&MY_LOGGER).unwrap();
    // Set the Max level we accept logging to the file for.
    log::set_max_level(LevelFilter::Info);

    // This allows us to take control of panic!() so we can send it to a file via the logger.
    panic::set_hook(Box::new(|panic_info| {
        let bt = Backtrace::new();

        error!("PANIC: {}, BACKTRACE: {:?}", panic_info, bt);
    }));

    // Create the directory for our map data
    fs::create_dir_all("./data/maps/")?;

    // Starts an event gathering type for the window.
    let event_loop = EventLoop::new()?;

    // Builds the Windows that will be rendered too.
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Map Editor")
            .with_inner_size(PhysicalSize::new(
                (949.0 * ZOOM_LEVEL) as u32,
                (802.0 * ZOOM_LEVEL) as u32,
            ))
            .with_visible(false)
            .with_enabled_buttons({
                let mut buttons = WindowButtons::all();
                buttons.remove(WindowButtons::MAXIMIZE);
                buttons
            })
            .with_resizable(false)
            .build(&event_loop)
            .unwrap(),
    );

    // Generates an Instance for WGPU. Sets WGPU to be allowed on all possible supported backends
    // These are DX12, DX11, Vulkan, Metal and Gles. if none of these work on a system they cant
    // play the game basically.
    let instance = wgpu::Instance::new(InstanceDescriptor {
        backends: Backends::all(),
        flags: InstanceFlags::default(),
        dx12_shader_compiler: Dx12Compiler::default(),
        gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
    });

    // This is used to ensure the GPU can load the correct.
    let compatible_surface = instance.create_surface(window.clone()).unwrap();

    // This creates the Window Struct and Device struct that holds all the rendering information
    // we need to render to the screen. Window holds most of the window information including
    // the surface type. device includes the queue and GPU device for rendering.
    // This then adds gpu_window and gpu_device and creates our renderer type. for easy passing of window, device and font system.
    let mut renderer = instance
        .create_device(
            window,
            //used to find adapters
            AdapterOptions {
                allowed_backends: Backends::all(),
                power: AdapterPowerSettings::HighPower,
                compatible_surface: Some(compatible_surface),
            },
            // used to deturmine which adapters support our special limits or features for our backends.
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::default(),
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None,
            // How we are presenting the screen which causes it to either clip to a FPS limit or be unlimited.
            wgpu::PresentMode::AutoVsync,
        )
        .await
        .unwrap();

    // get the screen size.
    let size = renderer.size();

    // get the Scale factor the pc currently is using for upscaling or downscaling the rendering.
    let scale = renderer.window().current_monitor().unwrap().scale_factor();

    // We generate Texture atlases to use with out types.
    let mut atlases: Vec<AtlasSet> = iter::from_fn(|| {
        Some(AtlasSet::new(
            &mut renderer,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            true,
        ))
    })
    .take(4)
    .collect();

    // we generate the Text atlas seperatly since it contains a special texture that only has the red color to it.
    // and another for emojicons.
    let text_atlas = TextAtlas::new(&mut renderer).unwrap();

    // Load textures image
    let resource = TextureAllocation::new(&mut atlases, &renderer)?;

    // Compile all rendering data in one type for quick access and passing
    let mut systems = DrawSetting {
        gfx: GfxCollection::new(),
        renderer,
        size,
        scale,
        resource,
        audio_list: AudioCollection::new(),
    };

    // Initiate map editor data
    let mut config_data = load_config();
    let mut gui = Interface::new(&mut systems, &mut config_data);
    let mut tileset = Tileset::new(&mut systems, &mut config_data);
    let mut gameinput = GameInput::new();
    let mut mapview = MapView::new(&mut systems, &mut config_data);
    let mut database = EditorData::new()?;

    // Load the initial map
    database.load_map_data(&mut systems, &mut mapview);
    database.load_link_maps(&mut mapview);

    // setup our system which includes Camera and projection as well as our controls.
    // for the camera.
    let system = System::new(
        &mut systems.renderer,
        Projection::Orthographic {
            left: 0.0,
            right: size.width,
            bottom: 0.0,
            top: size.height,
            near: 1.0,
            far: -100.0,
        },
        FlatControls::new(FlatSettings { zoom: ZOOM_LEVEL }),
        [size.width, size.height],
    );

    // We establish the different renderers here to load their data up to use them.
    let text_renderer = TextRenderer::new(&systems.renderer).unwrap();
    let image_renderer = ImageRenderer::new(&systems.renderer).unwrap();
    let map_renderer = MapRenderer::new(&mut systems.renderer, 81).unwrap();
    let ui_renderer = RectRenderer::new(&systems.renderer).unwrap();

    // Allow the window to be seen. hiding it then making visible speeds up
    // load times.
    systems.renderer.window().set_visible(true);

    // add everything into our convience type for quicker access and passing.
    let mut graphics = Graphics {
        system,
        image_atlas: atlases.remove(0),
        map_renderer,
        map_atlas: atlases.remove(0),
        image_renderer,
        text_atlas,
        text_renderer,
        ui_renderer,
        ui_atlas: atlases.remove(0),
    };

    // Create the mouse/keyboard bindings for our stuff.
    let bindings = Bindings::<Action, Axis>::new();

    // set bindings and create our own input handler.
    let mut input_handler = InputHandler::new(bindings, Duration::from_millis(180));

    let mut frame_time = FrameTime::new();
    let mut time = 0.0f32;
    let mut fps = 0u32;

    let mut mouse_pos: PhysicalPosition<f64> = PhysicalPosition::new(0.0, 0.0);
    let mut mouse_press: bool = false;

    #[allow(deprecated)]
    event_loop.run(move |event, elwt| {
        // we check for the first batch of events to ensure we dont need to stop rendering here first.
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
                ..
            } if window_id == systems.renderer.window().id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        // Close preference window
                        if gui.preference.is_open {
                            config_data.set_data(load_config());
                            gui.preference.close(&mut systems);
                        }
                        if database.got_changes() {
                            // We found changes on our map, we need to confirm if we would like to proceed to exit the editor
                            gui.open_dialog(
                                &mut systems,
                                DialogType::MapSave,
                                Some(database.did_map_change.clone()),
                            );
                        } else {
                            gui.open_dialog(&mut systems, DialogType::ExitConfirm, None);
                        }
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        if !handle_key_input(
                            event,
                            &mut gui,
                            &mut mapview,
                            &mut database,
                            &mut systems,
                        ) {
                            // Make sure that we only trigger the shortcut keys when we are not on a textbox
                            access_shortcut(
                                event,
                                &mut systems,
                                &mut gameinput,
                                &mut database,
                                &mut tileset,
                                &mut mapview,
                                &mut gui,
                                &mut config_data,
                            );
                        };
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        mouse_pos = *position;

                        if mouse_press {
                            handle_input(
                                &mut systems,
                                MouseInputType::LeftDownMove,
                                &Vec2::new(mouse_pos.x as f32, mouse_pos.y as f32),
                                &mut gameinput,
                                &mut gui,
                                &mut tileset,
                                &mut mapview,
                                &mut database,
                                &mut config_data,
                                elwt,
                            );
                        } else {
                            handle_input(
                                &mut systems,
                                MouseInputType::Move,
                                &Vec2::new(mouse_pos.x as f32, mouse_pos.y as f32),
                                &mut gameinput,
                                &mut gui,
                                &mut tileset,
                                &mut mapview,
                                &mut database,
                                &mut config_data,
                                elwt,
                            );
                        }
                    }
                    WindowEvent::MouseInput { state, .. } => match state {
                        ElementState::Pressed => {
                            handle_input(
                                &mut systems,
                                MouseInputType::LeftDown,
                                &Vec2::new(mouse_pos.x as f32, mouse_pos.y as f32),
                                &mut gameinput,
                                &mut gui,
                                &mut tileset,
                                &mut mapview,
                                &mut database,
                                &mut config_data,
                                elwt,
                            );
                            mouse_press = true;
                        }
                        ElementState::Released => {
                            handle_input(
                                &mut systems,
                                MouseInputType::Release,
                                &Vec2::new(mouse_pos.x as f32, mouse_pos.y as f32),
                                &mut gameinput,
                                &mut gui,
                                &mut tileset,
                                &mut mapview,
                                &mut database,
                                &mut config_data,
                                elwt,
                            );
                            mouse_press = false;
                        }
                    },
                    _ => {}
                }
            }
            Event::AboutToWait => {
                systems.renderer.window().request_redraw();
            }
            _ => {}
        }

        // update our renderer based on events here
        if !systems.renderer.update(&event).unwrap() {
            return;
        }

        // get the current window size so we can see if we need to resize the renderer.
        let new_size = systems.renderer.size();

        // update our inputs.
        input_handler.update(systems.renderer.window(), &event, 1.0);

        if systems.size != new_size {
            systems.size = new_size;

            // Reset screen size for the Surface here.
            graphics.system.set_projection(Projection::Orthographic {
                left: 0.0,
                right: new_size.width,
                bottom: 0.0,
                top: new_size.height,
                near: 1.0,
                far: -100.0,
            });

            systems.renderer.update_depth_texture();
        }

        frame_time.update();
        let seconds = frame_time.seconds();
        // update our systems data to the gpu. this is the Camera in the shaders.
        graphics.system.update(&systems.renderer, &frame_time);

        // update our systems data to the gpu. this is the Screen in the shaders.
        graphics
            .system
            .update_screen(&systems.renderer, [new_size.width, new_size.height]);

        // This adds the Image data to the Buffer for rendering.
        add_image_to_buffer(
            &mut systems,
            &mut graphics,
            &mut mapview,
            &mut gui,
            &mut tileset,
        );

        // this cycles all the Image's in the Image buffer by first putting them in rendering order
        // and then uploading them to the GPU if they have moved or changed in any way. clears the
        // Image buffer for the next render pass. Image buffer only holds the ID's and Sortign info
        // of the finalized Indicies of each Image.
        graphics.image_renderer.finalize(&mut systems.renderer);
        graphics.map_renderer.finalize(&mut systems.renderer);
        graphics.text_renderer.finalize(&mut systems.renderer);
        graphics.ui_renderer.finalize(&mut systems.renderer);

        // Start encoding commands. this stores all the rendering calls for execution when
        // finish is called.
        let mut encoder =
            systems
                .renderer
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("command encoder"),
                });

        // Run the render pass. for the games renderer
        graphics.render(&systems.renderer, &mut encoder);

        // Submit our command queue. for it to upload all the changes that were made.
        // Also tells the system to begin running the commands on the GPU.
        systems
            .renderer
            .queue()
            .submit(std::iter::once(encoder.finish()));

        if time < seconds {
            systems.gfx.set_text(
                &mut systems.renderer,
                gui.labels[LABEL_FPS],
                &format!("FPS: {fps}"),
            );
            fps = 0u32;
            time = seconds + 1.0;
        }
        fps += 1;

        systems.renderer.window().pre_present_notify();
        systems.renderer.present().unwrap();

        // These clear the Last used image tags.
        //Can be used later to auto unload things not used anymore if ram/gpu ram becomes a issue.
        if fps == 1 {
            graphics.image_atlas.trim();
            graphics.map_atlas.trim();
            graphics.text_atlas.trim();
            systems.renderer.font_sys.shape_run_cache.trim(1024);
        }
    })?;

    Ok(())
}
