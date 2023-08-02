use serde::{Deserialize, Serialize};
use serde_json::Result;

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
struct Network {
    address: String,
    port: u16,
}

#[derive(Serialize, Deserialize)]
enum Channel {
    Dimmer(usize),
    Red(usize),
    Green(usize),
    Blue(usize),
    Shutter(usize),
}

#[derive(Serialize, Deserialize)]
struct Profile {
    name: String,
    channels: Vec<Channel>,
}

#[derive(Serialize, Deserialize)]
struct MappingGroup {
    profile: String,
    patch: Patch,
}

#[derive(Serialize, Deserialize)]
struct Mapping {
    from: MappingGroup,
    to: MappingGroup,
    count: usize,
}

#[derive(Serialize, Deserialize)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Serialize, Deserialize)]
struct Size {
    width: usize,
    height: usize,
}

#[derive(Serialize, Deserialize)]
struct Grid {
    rows: usize,
    columns: usize,
}

#[derive(Serialize, Deserialize)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Serialize, Deserialize)]
struct Layout {
    direction: Direction,
    count: usize,
}

#[derive(Serialize, Deserialize)]
struct Patch {
    universe: usize,
    address: usize,
}

#[derive(Serialize, Deserialize)]
enum Mode {
    Dimmer,
    Rgb,
}

#[derive(Serialize, Deserialize)]
struct Fixture {
    position: Position,
    size: Size,
    grid: Grid,
    mode: Mode,
    layout: Layout,
}

#[derive(Serialize, Deserialize)]
struct Output {
    name: String,
    width: usize,
    height: usize,
    fixtures: Vec<Fixture>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    network: Network,
    profiles: Vec<Profile>,
    mappings: Vec<Mapping>,
    outputs: Vec<Output>,
}

fn load_config(path: &str) -> Result<()> {
    let input = std::fs::read_to_string(&path).unwrap();

    Ok(())
}

pub fn save_config(path: &str) -> Result<()> {
    let config = sample_config();
    // Serialize it to a JSON string.
    let j = serde_json::to_string(&config)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    Ok(())
}