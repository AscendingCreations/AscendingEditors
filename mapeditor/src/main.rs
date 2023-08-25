#![windows_subsystem = "windows"]

#[macro_use]
extern crate educe;
extern crate serde_big_array;

mod layout;
mod mapstate;
mod widgets;

use araiseal_logger::*;
use camera::{
    controls::{FlatControls, FlatSettings},
    Projection,
};
use cosmic_text::{Attrs, Metrics};
use graphics::iced_renderer::core::text::Renderer as _;
use graphics::iced_wgpu::{Backend, Renderer, Settings};
use graphics::iced_winit::{
    conversion,
    core::{mouse, renderer, Color as iced_color},
    runtime::{program, Debug},
    style::Theme,
    winit, Clipboard,
};
use graphics::*;
use input::{Bindings, FrameTime, InputHandler};
use layout::*;
use mapstate::State;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fs};
use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[tokio::main]
async fn main() -> Result<(), AscendingError> {
    let logger = Box::new(MyLogger::new("map_editor_log.txt"));
    logger.set_boxed_logger().unwrap();

    info!("starting up");
    info!("Setting Panic Hook");

    std::panic::set_hook(Box::new(|panic_info| {
        let bt = backtrace::Backtrace::new();

        error!("PANIC: {}, BACKTRACE: {:?}", panic_info, bt);
    }));

    fs::create_dir_all("./data/maps/")?;

    info!("Checked or Created Directorys");
    //ItemData::create_files()?;

    info!("Checked or Created Files");

    // Starts an event gathering type for the window.
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("MapEditor")
        .with_inner_size(PhysicalSize::new(800, 600))
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    // Generates an Instance for WGPU. Sets WGPU to be allowed on all possible supported backends
    // These are DX12, DX11, Vulkan, Metal and Gles. if none of these work on a system they cant
    // play the game basically.
    let instance = wgpu::Instance::default();

    // This is used to ensure the GPU can load the correct.
    let compatible_surface = unsafe { instance.create_surface(&window).unwrap() };
    // This creates the Window Struct and Device struct that holds all the rendering information
    // we need to render to the screen. Window holds most of the window information including
    // the surface type. device includes the queue and GPU device for rendering.
    // This then adds gpu_window and gpu_device and creates our renderer type. for easy passing of window, device and font system.
    let mut renderer = instance
        .create_device(
            window,
            &wgpu::RequestAdapterOptions {
                // High performance mode says to use Dedicated Graphics devices first.
                // Low power is APU graphic devices First.
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&compatible_surface),
                // we will never use this as this forces us to use an alternative renderer.
                force_fallback_adapter: false,
            },
            // used to deturmine if we need special limits or features for our backends.
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
            // How we are presenting the screen which causes it to either clip to a FPS limit or be unlimited.
            wgpu::PresentMode::AutoVsync,
        )
        .await
        .unwrap();

    // get the screen size.
    let mut size = renderer.size();

    // setup our system which includes Camera and projection as well as our controls.
    // for the camera.
    let system = System::new(
        &mut renderer,
        Projection::Orthographic {
            left: 0.0,
            right: size.width,
            bottom: 0.0,
            top: size.height,
            near: 1.0,
            far: -100.0,
        },
        FlatControls::new(FlatSettings::default()),
        [size.width, size.height],
    );
    // get the Scale factor the pc currently is using for upscaling or downscaling the rendering.
    let scale = renderer.window().current_monitor().unwrap().scale_factor();

    // We establish the different renderers here to load their data up to use them.
    let text_renderer = TextRenderer::new(&renderer).unwrap();
    let sprite_renderer = ImageRenderer::new(&renderer).unwrap();
    let mut map_renderer = MapRenderer::new(&mut renderer, 9).unwrap();

    // prepare the image atlases to store textures in
    let text_atlas = TextAtlas::new(&mut renderer).unwrap();
    let mut map_atlas: AtlasGroup =
        AtlasGroup::new(&mut renderer, wgpu::TextureFormat::Rgba8UnormSrgb);

    // prepare the tilesheets Vec.
    let mut tilesheets: Vec<TileSheet> = Vec::with_capacity(100);

    // lets preload all the tilesheets. if any fail we stop loading them and continue.
    for i in 1..=100 {
        if let Ok(texture) = Texture::from_file(format!("images/tiles/{}.png", i)) {
            match texture.new_tilesheet(&mut map_atlas, &renderer, 20) {
                Some(sheet) => tilesheets.push(sheet),
                None => break,
            }
        } else {
            break;
        }
    }

    // We make a new Map to render here.
    let mut map = Map::new(&mut renderer, 20);
    map.init_texture_layer(&mut map_renderer);

    // prepare the sprites
    let mut sprites: Vec<Image> = Vec::with_capacity(20);

    //prepare the rendered on map text
    let mut texts: Vec<Text> = Vec::with_capacity(1024);

    // lay out each text over the tiles.
    for i in 0..1024 {
        let posx = (i % 32) * 20 + 15;
        let posy = (i / 32) * 20 + 15;
        let mut text = Text::new(
            &mut renderer,
            Some(Metrics::new(16.0, 16.0).scale(scale as f32)),
            Vec3::new(posx as f32, posy as f32, 1.0),
            Vec2::new(32.0, 32.0),
        );

        text.set_buffer_size(&mut renderer, size.width as i32, size.height as i32);
        text.set_text(&mut renderer, "A", Attrs::new());
        texts.push(text);
    }

    // iceds debugger start up
    let mut debug = Debug::new();

    // setup the renderer for iced for UI rendering.
    let mut iced_renderer = Renderer::new(Backend::new(
        renderer.device(),
        renderer.queue(),
        Settings::default(),
        renderer.surface_format(),
    ));

    // start up iceds controls for keyboard etc entry.
    let iced_controls = Pages::new();

    // Start your program up with the UI you want to render with.
    let mut iced_state = program::State::new(
        iced_controls,
        system.iced_view().logical_size(),
        &mut iced_renderer,
        &mut debug,
    );

    iced_renderer.load_font(Cow::from(
        graphics::iced_aw::graphics::icons::ICON_FONT_BYTES,
    ));
    // Allow the window to be seen. hiding it then making visible speeds up
    // load times.
    renderer.window().set_visible(true);

    let mut state = State {
        system,
        sprites,
        texts,
        map,
        map_renderer,
        map_atlas,
        sprite_renderer,
        text_atlas,
        text_renderer,
    };

    // Create the mouse/keyboard bindings for our stuff.
    let mut bindings = Bindings::<Action, Axis>::new();
    bindings.insert_action(Action::Quit, vec![winit::event::VirtualKeyCode::Q.into()]);

    // set bindings and create our own input handler.
    let mut input_handler = InputHandler::new(bindings);

    let mut frame_time = FrameTime::new();

    // this is for Copy paste stuff within Iced.
    let mut clipboard = Clipboard::connect(renderer.window());

    #[allow(deprecated)]
    event_loop.run(move |event, _, control_flow| {
        // we check for the first batch of events to ensure we dont need to stop rendering here first.
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
                ..
            } if window_id == renderer.window().id() => {
                if let WindowEvent::CloseRequested = *event {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => {
                if !iced_state.is_queue_empty() {
                    // We update iced
                    let _ = iced_state.update(
                        state.system.iced_view().logical_size(),
                        input_handler
                            .physical_mouse_position()
                            .map(|p| {
                                conversion::cursor_position(
                                    p,
                                    state.system.iced_view().scale_factor(),
                                )
                            })
                            .map(mouse::Cursor::Available)
                            .unwrap_or(mouse::Cursor::Unavailable),
                        &mut iced_renderer,
                        &Theme::Dark,
                        &renderer::Style {
                            text_color: iced_color::WHITE,
                        },
                        &mut clipboard,
                        &mut debug,
                    );

                    // and request a redraw
                    renderer.window().request_redraw();
                    return;
                }

                renderer.window().request_redraw();
            }
            _ => {}
        }

        // get the current window size so we can see if we need to resize the renderer.
        let new_size = renderer.size();
        let inner_size = renderer.window().inner_size();

        // if our rendering size is zero stop rendering to avoid errors.
        if new_size.width == 0.0
            || new_size.height == 0.0
            || inner_size.width == 0
            || inner_size.height == 0
        {
            return;
        }

        // update our inputs.
        input_handler.update(renderer.window(), &event, 1.0);

        // handle the GUI events here.
        if let Event::WindowEvent { ref event, .. } = &event {
            if let Some(event) = graphics::iced_winit::conversion::window_event(
                event,
                renderer.window().scale_factor(),
                input_handler.modifiers(),
            ) {
                iced_state.queue_event(event);
            }
        }

        // update our renderer based on events here
        if !renderer.update(&event).unwrap() {
            return;
        }

        if size != new_size {
            size = new_size;

            // Reset screen size for the Surface here.
            state.system.set_projection(Projection::Orthographic {
                left: 0.0,
                right: new_size.width,
                bottom: 0.0,
                top: new_size.height,
                near: 1.0,
                far: -100.0,
            });

            renderer.update_depth_texture();
        }

        // check if out close action was hit for esc
        if input_handler.is_action_down(&Action::Quit) {
            *control_flow = ControlFlow::Exit;
        }

        let seconds = frame_time.seconds();
        // update our systems data to the gpu. this is the Camera in the shaders.
        state.system.update(&renderer, &frame_time);

        // update our systems data to the gpu. this is the Screen in the shaders.
        state
            .system
            .update_screen(&renderer, [new_size.width, new_size.height]);

        // This adds the Image data to the Buffer for rendering.
        state.sprites.iter_mut().for_each(|sprite| {
            state.sprite_renderer.image_update(sprite, &mut renderer);
        });

        // this cycles all the Image's in the Image buffer by first putting them in rendering order
        // and then uploading them to the GPU if they have moved or changed in any way. clears the
        // Image buffer for the next render pass. Image buffer only holds the ID's and Sortign info
        // of the finalized Indicies of each Image.
        state.sprite_renderer.finalize(&mut renderer);

        state.texts.iter_mut().for_each(|text| {
            state
                .text_renderer
                .text_update(text, &mut state.text_atlas, &mut renderer)
                .unwrap();
        });

        state.text_renderer.finalize(&mut renderer);
        state.map_renderer.map_update(&mut state.map, &mut renderer);
        state.map_renderer.finalize(&mut renderer);

        // Start encoding commands. this stores all the rendering calls for execution when
        // finish is called.
        let mut encoder =
            renderer
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("command encoder"),
                });

        // Run the render pass. for the games renderer
        state.render(&renderer, &mut encoder);

        // Run the render pass for iced GUI renderer.
        iced_renderer.with_primitives(|backend, primitive| {
            backend.present(
                renderer.device(),
                renderer.queue(),
                &mut encoder,
                None,
                renderer.frame_buffer().as_ref().expect("no frame view?"),
                primitive,
                state.system.iced_view(),
                &debug.overlay(),
            );
        });

        // Submit our command queue. for it to upload all the changes that were made.
        // Also tells the system to begin running the commands on the GPU.
        renderer.queue().submit(std::iter::once(encoder.finish()));

        input_handler.end_frame();
        frame_time.update();
        renderer.present().unwrap();

        renderer
            .window_mut()
            .set_cursor_icon(iced_winit::conversion::mouse_interaction(
                iced_state.mouse_interaction(),
            ));

        // These clear the Last used image tags.
        //Can be used later to auto unload things not used anymore if ram/gpu ram becomes a issue.
        state.map_atlas.trim();
        state.text_atlas.trim();
    });
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
enum Action {
    Quit,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
enum Axis {
    Forward,
    Sideward,
    Yaw,
    Pitch,
}
