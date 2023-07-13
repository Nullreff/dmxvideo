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

const VALUE_SIZE : usize = 256;
const UNIVERSE_SIZE : usize = 512;
const MAX_UNIVERSES : usize = 512;

struct DmxData {
    universe: u16,
    data: Vec<u8>,
}

#[derive(Resource, Deref)]
struct StreamReceiver(Receiver<DmxData>);

#[derive(Event)]
struct StreamEvent(DmxData);

fn main() {
    App::new()
        .add_plugins((
            window_plugin(),
            Material2dPlugin::<MultiColorMaterial>::default(),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, read_stream)
        .run();
    
}

fn window_plugin() -> PluginGroupBuilder {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "DMX Video".into(),
            resolution: (500., 300.).into(),
            present_mode: PresentMode::AutoVsync,
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Dark),
            ..default()
        }),
        ..default()
    }).set(AssetPlugin {
        watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
        ..Default::default()
    })
}

fn generate_dmx_image(color: u8) -> Image {
    Image::new(
        Extent3d {
            width: UNIVERSE_SIZE as u32 / 4,
            height: 1, //MAX_UNIVERSES as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        iter::repeat(0).take(UNIVERSE_SIZE / 4)
            .flat_map(|a| vec![0, 0, 0, 255])
            .collect::<Vec<u8>>(),
        TextureFormat::Rgba8UnormSrgb,
    )
}

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

fn setup(
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
                texture: handle,
            }),
            ..default()
        }
    )
    .insert(DmxGradient{});


    let (tx, rx) = bounded::<DmxData>(10);
    setup_artnet(tx);
    commands.insert_resource(StreamReceiver(rx));
}

// This system reads from the receiver and sends events to Bevy
fn read_stream(
    mut materials: ResMut<Assets<MultiColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<&Handle<MultiColorMaterial>, With<DmxGradient>>,
    receiver: Res<StreamReceiver>
) {
    let material = materials.get_mut(query.single()).unwrap();
    let image = images.get_mut(&material.texture).unwrap();

    for from_stream in receiver.try_iter() {
        if from_stream.universe != 0 {
            continue;
        }
        image
            .data
            .copy_from_slice(from_stream.data.as_slice());
    }
}

fn test_system(
    time: Res<Time>,
    mut materials: ResMut<Assets<MultiColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    webview_query: Query<&Handle<MultiColorMaterial>, With<DmxGradient>>,
) {
    let material = materials.get_mut(webview_query.single()).unwrap();

    let image = images.get_mut(&material.texture).unwrap();

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
    texture: Handle<Image>,
}

#[derive(Component)]
struct DmxGradient { }
