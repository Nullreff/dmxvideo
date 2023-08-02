use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Material2d, Material2dKey},
    reflect::{TypePath, TypeUuid},
    render::{
        render_resource::{AsBindGroup, ShaderRef, RenderPipelineDescriptor, SpecializedMeshPipelineError, Extent3d, TextureDimension, TextureFormat},
        mesh::{MeshVertexBufferLayout},
    },
};
use crate::config::*;
use crate::network::{DmxData, StreamReceiver};
use std::{iter};

const SHADER_NAME : &str = "shaders/dmx_material.wgsl";

#[derive(Component)]
pub struct DmxGradient { }

#[derive(Event)]
struct StreamEvent(DmxData);


#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct DmxMaterial {
    #[texture(0)]
    #[sampler(1)]
    dmx_data: Handle<Image>,
}

impl Material2d for DmxMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_NAME.into()
    }
    fn vertex_shader() -> ShaderRef {
        SHADER_NAME.into()
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

fn generate_dmx_image(color: u8) -> Image {
    Image::new(
        Extent3d {
            width: IMAGE_WIDTH as u32,
            height: IMAGE_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        iter::repeat(color).take(DMX_SIZE)
            .collect::<Vec<u8>>(),
        TextureFormat::Rgba8Uint,
    )
}

pub fn setup_shader(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<DmxMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn(Camera2dBundle::default());

    let image = generate_dmx_image(0);
    let handle = images.add(image);

    commands.spawn(
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(128.)),
            material: materials.add(DmxMaterial {
                dmx_data: handle,
            }),
            ..default()
        }
    )
    .insert(DmxGradient{});
}

pub fn update_shader(
    mut materials: ResMut<Assets<DmxMaterial>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<&Handle<DmxMaterial>, With<DmxGradient>>,
    receiver: Res<StreamReceiver>
) {
    let material = materials.get_mut(query.single()).unwrap();
    let image = images.get_mut(&material.dmx_data).unwrap();

    for from_stream in receiver.try_iter() {
        let universe = from_stream.universe as usize;
        let start = universe * IMAGE_WIDTH;
        let end = start + UNIVERSE_SIZE;
        let incoming_data = from_stream.data.iter().cloned().flat_map(|d| [d, d, d, d]);
        image.data.splice(start..end, incoming_data);
    }
}
