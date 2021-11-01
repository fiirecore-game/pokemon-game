pub extern crate firecore_engine as engine;
pub extern crate firecore_pokedex_engine as pokedex;
extern crate firecore_saves as saves;
extern crate firecore_storage as storage;

pub mod args;
pub mod battle;
pub mod game;
pub mod state;
pub mod world;

use std::ops::{Deref, DerefMut};

use game::{config::Configuration, init};
use saves::PlayerSaves;

use engine::{
    tetra::{Context, ContextBuilder, Result},
    util::{HEIGHT, WIDTH},
    EngineContext,
};

use log::info;
use pokedex::context::PokedexClientContext;
use state::StateManager;

extern crate firecore_battle as battlelib;
extern crate firecore_world as worldlib;

pub const TITLE: &str = "Pokemon FireRed";
pub const DEBUG_NAME: &str = env!("CARGO_PKG_NAME");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const DEFAULT_SCALE: f32 = 3.0;

fn main() -> Result {
    init::logger();

    info!("Starting {} v{}", TITLE, VERSION);
    info!("By {}", AUTHORS);

    let args = args();

    #[cfg(debug_assertions)]
    if !args.contains(&Args::NoSeed) {
        // init::seed_random(engine::util::date() % 1000000)
    }

    #[cfg(feature = "discord")]
    use discord_rich_presence::{activity::Activity, new_client, DiscordIpc};

    #[cfg(feature = "discord")]
    let mut client = {
        let mut client = new_client("862413316420665386")
            .unwrap_or_else(|err| panic!("Could not create discord IPC client with error {}", err));
        client
            .connect()
            .unwrap_or_else(|err| panic!("Could not connect to discord with error {}", err));
        client
            .set_activity(Activity::new().state("test state").details("test details"))
            .unwrap_or_else(|err| panic!("Could not set client activity with error {}", err));
        client
    };

    let debug = cfg!(debug_assertions);
    let save_locally = cfg!(debug_assertions);

    let fonts = bincode::deserialize(include_bytes!("../build/data/fonts.bin"))
        .unwrap_or_else(|err| panic!("Could not load font sheets with error {}", err));

    let mut engine = engine::build(
        ContextBuilder::new(
            TITLE,
            (WIDTH * DEFAULT_SCALE) as _,
            (HEIGHT * DEFAULT_SCALE) as _,
        )
        .resizable(true)
        .show_mouse(true),
        fonts,
    )?;

    // Loads configuration, sets up controls

    let configuration = Configuration::load(&mut engine, save_locally)?;

    // Load dexes;

    let (pokedex, movedex, itemdex) = bincode::deserialize(include_bytes!("../build/data/dex.bin"))
        .unwrap_or_else(|err| panic!("Could not deserialize pokedex with error {}", err));

    let dex_engine = bincode::deserialize(include_bytes!("../build/data/dex_engine.bin"))
        .unwrap_or_else(|err| {
            panic!(
                "Could not deserialize pokedex engine data with error {}",
                err
            )
        });

    let dex = PokedexClientContext::new(&mut engine, &pokedex, &movedex, &itemdex, dex_engine)?;

    let mut saves = PlayerSaves::load(save_locally)
        .unwrap_or_else(|err| panic!("Could not load player saves with error {}", err));

    saves.select_first_or_default(
        save_locally,
        &mut rand::thread_rng(),
        dex.pokedex,
        dex.movedex,
        dex.itemdex,
    );

    let mut ctx = GameContext {
        engine,
        dex,
        configuration,
        saves,
        save_locally,
        debug,
    };

    engine::tetra::run(&mut ctx, |ctx| StateManager::new(ctx, args))?;

    #[cfg(feature = "discord")]
    client.close().unwrap();

    Ok(())
}

pub struct GameContext<'d> {
    pub engine: EngineContext,
    pub dex: PokedexClientContext<'d>,
    pub configuration: Configuration,
    pub saves: PlayerSaves<'d>,
    pub save_locally: bool,
    pub debug: bool,
}

impl<'d> Deref for GameContext<'d> {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        self.engine.deref()
    }
}

impl<'d> DerefMut for GameContext<'d> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.engine.deref_mut()
    }
}

#[derive(PartialEq)]
pub enum Args {
    DisableAudio,
    Debug,
    #[cfg(debug_assertions)]
    NoSeed,
}

pub fn args() -> Vec<Args> {
    let mut list = Vec::new();
    let mut args = pico_args::Arguments::from_env();

    if args.contains("-a") {
        list.push(Args::DisableAudio);
    }

    if args.contains("-d") {
        list.push(Args::Debug);
    }

    #[cfg(debug_assertions)]
    if args.contains("-s") {
        list.push(Args::NoSeed);
    }

    list
}
