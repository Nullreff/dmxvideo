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

#[derive(Serialize, Deserialize)]
struct Network {
    address: String,
    port: u16,
}

#[derive(Serialize, Deserialize)]
struct Profile {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Mapping {

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
    Gradient,
}

#[derive(Serialize, Deserialize)]
struct Fixture {
    position: Position,
    size: Size,
    grid: Grid,
    mode: Mode,

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

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

pub fn save_config(path: &str) -> Result<()> {
    // Some data structure.
    let config = Config {
        network: Network {
            address: "0.0.0.0".to_owned(),
            port: 6454,
        },
        profiles: vec!(),
        mappings: vec!(),
        outputs: vec!(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&config)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    Ok(())
}