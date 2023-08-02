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

use crate::config::*;
use crate::network::{DmxData, StreamReceiver};

#[derive(Component)]
pub struct DmxGradient { }

#[derive(Event)]
struct StreamEvent(DmxData);

fn generate_dmx_image(color: u8) -> Image {
    Image::new(
        Extent3d {
            width: IMAGE_WIDTH as u32,
            height: IMAGE_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        iter::repeat(0).take(DMX_SIZE)
            .collect::<Vec<u8>>(),
        TextureFormat::Rgba8Uint,
    )
}

pub fn setup_shader(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MultiColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn(Camera2dBundle::default());

    let image = generate_dmx_image(255);
    let handle = images.add(image);

    commands.spawn(
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(128.)),
            material: materials.add(MultiColorMaterial {
                dmx_data: handle,
            }),
            ..default()
        }
    )
    .insert(DmxGradient{});
}

pub fn update_shader(
    mut materials: ResMut<Assets<MultiColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<&Handle<MultiColorMaterial>, With<DmxGradient>>,
    receiver: Res<StreamReceiver>
) {
    let material = materials.get_mut(query.single()).unwrap();
    let image = images.get_mut(&material.dmx_data).unwrap();

    for from_stream in receiver.try_iter() {
        if from_stream.universe != 0 {
            continue;
        }
        let universe = from_stream.universe as usize;
        let start = (universe - 1) * IMAGE_WIDTH;
        let end = start + UNIVERSE_SIZE;
        let incoming_data = from_stream.data.iter().cloned().flat_map(|d| [d, d, d, d]);
        image.data.splice(start..end, incoming_data);
    }
}

fn test_system(
    time: Res<Time>,
    mut materials: ResMut<Assets<MultiColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    webview_query: Query<&Handle<MultiColorMaterial>, With<DmxGradient>>,
) {
    let material = materials.get_mut(webview_query.single()).unwrap();

    let image = images.get_mut(&material.dmx_data).unwrap();

    let color = (((time.elapsed_seconds() % 5.0) / 5.0) * 255.0) as u8;
    image
        .data
        .copy_from_slice(&[color, color, color, 255].repeat(48 * 1));
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material2d for MultiColorMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/multi_color_material.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/multi_color_material.wgsl".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct MultiColorMaterial {
    #[texture(0)]
    #[sampler(1)]
    dmx_data: Handle<Image>,
}