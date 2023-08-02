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