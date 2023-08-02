use std::{iter, thread};
use crossbeam_channel::{bounded, Sender, Receiver};
use std::net::{UdpSocket, ToSocketAddrs};
use artnet_protocol::{ArtCommand, Poll};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Material2d, Material2dKey, Material2dPlugin},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    window::{CursorGrabMode, PresentMode, WindowLevel, WindowTheme},
    app::PluginGroupBuilder,
    reflect::{TypePath, TypeUuid},
    render::{
        render_resource::{AsBindGroup, ShaderRef, RenderPipelineDescriptor, SpecializedMeshPipelineError, Extent3d, TextureDimension, TextureFormat},
        mesh::{MeshVertexBufferLayout, MeshVertexAttribute},
    },
    utils::Duration,
    asset::ChangeWatcher,
};



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