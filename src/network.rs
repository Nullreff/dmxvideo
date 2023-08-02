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

pub struct DmxData {
    pub universe: u16,
    pub data: Vec<u8>,
}

use crate::config::*;


#[derive(Resource, Deref)]
pub struct StreamReceiver(Receiver<DmxData>);

fn setup_artnet(tx: Sender<DmxData>) {
    let socket = UdpSocket::bind(("0.0.0.0", 6454)).unwrap();

    info!("Artnet listener started");

    std::thread::spawn(move || loop {
        let mut buffer = [0u8; 1024];
        let (length, addr) = socket.recv_from(&mut buffer).unwrap();
        let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();

        if let ArtCommand::Output(o) = command {
            if o.port_address >= (MAX_UNIVERSES as u16).try_into().unwrap() {
                let addr : u16 = o.port_address.into();
                warn!("Received artnet on universe {}, outside the limit of {}",
                    addr, MAX_UNIVERSES);
                continue;
            }

            if *o.length as usize != UNIVERSE_SIZE {
                warn!("Received artnet data with length {}, expected {}", *o.length, UNIVERSE_SIZE);
                continue;
            }

            let data = DmxData {
                universe: o.port_address.into(),
                data: o.data.as_ref().to_vec(),

            };

            tx.send(data).unwrap();
        }
    });
}

pub fn setup_network(
    mut commands: Commands,
) {
    let (tx, rx) = bounded::<DmxData>(10);
    setup_artnet(tx);
    commands.insert_resource(StreamReceiver(rx));
}
