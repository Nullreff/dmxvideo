use serde::{Deserialize, Serialize};
use serde_json::Result;
use bevy::ecs::system::Commands;
use std::{
    env,
    fs,
    process,
};
use bevy::ecs::system::Resource;
use bevy::prelude::Deref;


pub const VALUE_SIZE : usize = 256;
pub const UNIVERSE_SIZE : usize = 512;
pub const MAX_UNIVERSES : usize = 512;
pub const CHANNELS : usize = 4;

pub const DMX_WIDTH : usize = UNIVERSE_SIZE * CHANNELS;
pub const DMX_HEIGHT : usize = MAX_UNIVERSES;
pub const DMX_SIZE : usize = DMX_WIDTH * DMX_HEIGHT;

pub const IMAGE_WIDTH : usize = UNIVERSE_SIZE;
pub const IMAGE_HEIGHT : usize = MAX_UNIVERSES;
pub const IMAGE_SIZE : usize = IMAGE_WIDTH * IMAGE_HEIGHT;

pub const NETWORK_ADDRESS: &str = "0.0.0.0";
pub const NETWORK_PORT: u16 = 6454;

fn sample_config() -> Config {
    Config {
        network: Network {
            address: "0.0.0.0".to_owned(),
            port: 6454,
        },
        profiles: vec!(
            Profile {
                name: "Brightstripe".to_owned(),
                channels: (0..48).into_iter().flat_map(|i| [Channel::Red(i), Channel::Green(i), Channel::Blue(i)]).collect(),
            },
        ),
        mappings: vec!(
            Mapping {
                from: MappingGroup {
                    profile: "Brigthstripe".to_owned(),
                    patch: Patch {
                        universe: 0,
                        address: 1,
                    }
                },
                to: MappingGroup {
                    profile: "Brigthstripe".to_owned(),
                    patch: Patch {
                        universe: 0,
                        address: 1,
                    }
                },
                count: 28,
            },
        ),
        outputs: vec!(
            Output {
                name: "Main".to_owned(),
                width: 960,
                height: 540,
                fixtures: vec!(
                    Fixture {
                        position: Position { x: 0, y: 0 },
                        size: Size { width: 100, height: 10 },
                        grid: Grid { rows: 14, columns: 2 },
                        mode: Mode::Rgb,
                        layout: Layout { direction: Direction::Horizontal, count: 48 },
                    }
                ),
            }
        ),
    }
}


#[derive(Serialize, Deserialize)]
pub struct Network {
    address: String,
    port: u16,
}

#[derive(Serialize, Deserialize)]
pub enum Channel {
    Dimmer(usize),
    Red(usize),
    Green(usize),
    Blue(usize),
    Shutter(usize),
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    name: String,
    channels: Vec<Channel>,
}

#[derive(Serialize, Deserialize)]
pub struct MappingGroup {
    profile: String,
    patch: Patch,
}

#[derive(Serialize, Deserialize)]
pub struct Mapping {
    from: MappingGroup,
    to: MappingGroup,
    count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Size {
    width: usize,
    height: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Grid {
    rows: usize,
    columns: usize,
}

#[derive(Serialize, Deserialize)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Serialize, Deserialize)]
pub struct Layout {
    direction: Direction,
    count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Patch {
    universe: usize,
    address: usize,
}

#[derive(Serialize, Deserialize)]
pub enum Mode {
    Dimmer,
    Rgb,
}

#[derive(Serialize, Deserialize)]
pub struct Fixture {
    position: Position,
    size: Size,
    grid: Grid,
    mode: Mode,
    layout: Layout,
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    name: String,
    width: usize,
    height: usize,
    fixtures: Vec<Fixture>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    network: Network,
    profiles: Vec<Profile>,
    mappings: Vec<Mapping>,
    outputs: Vec<Output>,
}

#[derive(Resource, Deref)]
pub struct ConfigResource(Config);

pub fn load_config(config: &str) -> Result<Config> {
    serde_json::from_str(config)
}

pub fn save_config(config: &Config) -> Result<String> {
    serde_json::to_string(&config)
}

pub fn setup_config(
    mut commands: Commands,
) {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <config file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let input = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            process::exit(1);
        }
    };

    let config = load_config(&input).unwrap();
    commands.insert_resource(ConfigResource(config));
}
