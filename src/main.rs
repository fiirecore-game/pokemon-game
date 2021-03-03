use io::args::Args;
use frc_data::configuration::Configuration;
use macroquad::camera::Camera2D;
use macroquad::prelude::BLACK;
use macroquad::prelude::Conf;
use macroquad::prelude::clear_background;
use macroquad::prelude::collections::storage;
use macroquad::prelude::get_frame_time;
use macroquad::prelude::info;
use macroquad::prelude::next_frame;
use macroquad::prelude::coroutines::start_coroutine;
use scene::loading::manager::load_coroutine;
use scene::manager::SceneManager;
use frc_data::data::PersistantDataLocation;
use scene::Scene;

pub mod util;
pub mod scene;
pub mod io;
pub mod world;
pub mod battle;
pub mod gui;

pub mod pokemon;

pub static TITLE: &str = "Pokemon FireRed";
pub static DEBUG_NAME: &str = env!("CARGO_PKG_NAME");
pub static AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static BASE_WIDTH: u32 = 240;
pub static BASE_HEIGHT: u32 = 160;

pub static SCALE: f32 = 3.0;

static mut QUIT: bool = false;


#[macroquad::main(settings)]
async fn main() {

    info!("Starting {} v{}", TITLE, VERSION);
    info!("By {}", AUTHORS);
    
    if cfg!(debug_assertions) {
        info!("Running in debug mode");
    }

    let config = Configuration::load_from_file().await;

    config.on_reload();

    storage::store(config);

    let args = crate::io::args::parse_args();

    macroquad::camera::set_camera(Camera2D::from_display_rect(macroquad::prelude::Rect::new(0.0, 0.0, BASE_WIDTH as _, BASE_HEIGHT as _)));

    if !args.contains(&Args::DisableAudio) {
        frc_audio::create();
    }

    let loading_coroutine = if cfg!(not(target_arch = "wasm32")) {
        start_coroutine(load_coroutine())
    } else {
        start_coroutine(async {
            let texture = crate::util::graphics::texture::byte_texture(include_bytes!("../build/assets/loading.png"));
            loop {
                clear_background(macroquad::prelude::BLUE);
                macroquad::prelude::draw_texture(texture, 0.0, 0.0, macroquad::prelude::WHITE);
                next_frame().await;
            }
        })
    };

    info!("Loading assets...");

    frc_audio::bind_world_music().await;
    
    // storage::store(io::data::player::PlayerData::load_from_file().await);
    
    let mut scene_manager = SceneManager::new();

    pokemon::load();

    info!("Finished loading assets!");


    if cfg!(not(target_arch = "wasm32")) {
        while !loading_coroutine.is_done() {
            macroquad::prelude::coroutines::wait_seconds(0.05).await;
        } 
    }

    macroquad::prelude::coroutines::stop_coroutine(loading_coroutine); 

    if cfg!(target_arch = "wasm32") {
        load_coroutine().await;
    }  

    info!("Starting game!");

    scene_manager.on_start();

    loop {

        #[cfg(target_arch = "wasm32")]
        frc_audio::quadsnd::context::music::MIXER.lock().frame();


        scene_manager.input(get_frame_time());
        
        scene_manager.poll(get_frame_time()).await;


        clear_background(BLACK);

        scene_manager.render();
        // io::input::touchscreen::TOUCH_CONTROLS.render();


        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F12) {
            if let Some(mut config) = storage::get_mut::<Configuration>() {
                frc_data::data::PersistantData::reload(std::ops::DerefMut::deref_mut(&mut config)).await; // maybe change into coroutine
            }
            if let Some(mut player_data) = storage::get_mut::<crate::io::data::player::PlayerData>() {
                frc_data::data::PersistantData::reload(std::ops::DerefMut::deref_mut(&mut player_data)).await;
            }
        }

        if unsafe{QUIT} {
            break;
        }

        next_frame().await;
    }

    info!("Quitting game...");
    scene_manager.quit();

}

pub fn queue_quit() {
    unsafe {
        QUIT = true;
    }
}

fn settings() -> Conf {
    Conf {
        window_title: TITLE.to_string(),
        window_width: (BASE_WIDTH * SCALE as u32) as _,
        window_height: (BASE_HEIGHT * SCALE as u32) as _,
        sample_count: 1,
        ..Default::default()
    }
}