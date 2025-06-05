use avian2d::{position, prelude::*};
#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::{
    color::palettes::css::{AZURE, BISQUE, BROWN, GREY, HOT_PINK, ORANGE_RED, ROYAL_BLUE},
    input::common_conditions::input_pressed,
    picking::pointer::PointerLocation,
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
        PhysicsPickingPlugin::default(),
        #[cfg(not(target_arch = "wasm32"))]
        Wireframe2dPlugin::default(),
    ));
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, toggle_wireframe);

    app.add_systems(
        Update,
        (
            pointer_click_send_spawn_event.run_if(input_pressed(MouseButton::Left)),
            handle_spawn_particle_event,
        ),
    );

    app.init_resource::<SelectedElement>();
    app.add_event::<SpawnParticleEvent>();
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ElementTypes {
    Wall,
    #[default]
    Sand,
    Water,
    Ice,
    Fire,
    Oil,
    BouncyBall,
}

/// Indicates whether the particle is frozen in place or free to move around.
/// Things like walls and ice are frozen, while sand and water are not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionState {
    Frozen, // The particle is frozen in place and cannot move.
    Solid,  // The particle is solid and can move. (rectangle collider)
    Liquid, // The particle is liquid and can flow freely. (circle collider)
    Gas,    // The particle is gas and randomly wanders to fill its container.
}

/// The properties of an element
#[derive(Component, Debug, Clone, PartialEq)]
pub struct Element {
    pub color: Color,
    pub name: String,
    pub motion_state: MotionState,
    pub is_flammable: bool,
}

impl Element {
    pub fn new(color: Color, name: String, motion_state: MotionState, is_flammable: bool) -> Self {
        Self {
            color,
            name,
            motion_state,
            is_flammable,
        }
    }

    pub fn from_type(element_type: ElementTypes) -> Self {
        match element_type {
            ElementTypes::Wall => Element {
                color: GREY.into(),
                name: "Wall".to_string(),
                motion_state: MotionState::Frozen,
                is_flammable: false,
            },
            ElementTypes::Sand => Element {
                color: BISQUE.into(),
                name: "Sand".to_string(),
                motion_state: MotionState::Solid,
                is_flammable: false,
            },
            ElementTypes::Water => Element {
                color: ROYAL_BLUE.into(),
                name: "Water".to_string(),
                motion_state: MotionState::Liquid,
                is_flammable: false,
            },
            ElementTypes::Ice => Element {
                color: AZURE.into(),
                name: "Ice".to_string(),
                motion_state: MotionState::Frozen,
                is_flammable: false,
            },
            ElementTypes::Fire => Element {
                color: ORANGE_RED.into(),
                name: "Fire".to_string(),
                motion_state: MotionState::Gas,
                is_flammable: true,
            },
            ElementTypes::Oil => Element {
                color: BROWN.into(),
                name: "Oil".to_string(),
                motion_state: MotionState::Liquid,
                is_flammable: true,
            },
            ElementTypes::BouncyBall => Element {
                color: HOT_PINK.into(),
                name: "Bouncy Ball".to_string(),
                motion_state: MotionState::Solid,
                is_flammable: false,
            },
        }
    }
}

#[derive(Resource, Debug, Default, Clone, PartialEq)]
pub struct SelectedElement(pub ElementTypes);

/// A single physical unit of an element.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct Particle {
    pub element: Element,
}

/// Event for spawning a particle at a position with a specific element.
#[derive(Event, Debug, Clone)]
pub struct SpawnParticleEvent {
    pub position: Vec2,
    pub element: Element,
}

/// System to send spawn events on pointer click
fn pointer_click_send_spawn_event(
    mut event_writer: EventWriter<SpawnParticleEvent>,
    camera: Query<(&Camera, &GlobalTransform)>,
    pointers: Query<&PointerLocation>,
    selected_element: Res<SelectedElement>,
) {
    for pointer in pointers.iter() {
        if let Some(location) = pointer.location() {
            // Check if the pointer is pressed
            // Send the spawn event with the pointer location and selected element
            if let Ok((camera, camera_transform)) = camera.single() {
                match camera.viewport_to_world_2d(camera_transform, location.position) {
                    Ok(spawn_location) => {
                        // Create the spawn event with the position and selected element
                        event_writer.write(SpawnParticleEvent {
                            position: spawn_location,
                            element: Element::from_type(selected_element.0),
                        });
                    }
                    Err(_) => {
                        warn!("Failed to convert pointer location to world position");
                    }
                }
            }
        }
    }
}

/// System to handle spawn events
fn handle_spawn_particle_event(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnParticleEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in event_reader.read() {
        // Spawn the particle entity with the specified element and position
        let element = event.element.clone();
        let position = event.position;
        commands.spawn((
            Particle {
                element: element.clone(),
            },
            Transform::from_xyz(position.x, position.y, 0.0),
            match element.motion_state {
                MotionState::Frozen => (
                    RigidBody::Static,
                    Collider::rectangle(1.0, 1.0),
                    Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
                ),
                MotionState::Solid => (
                    RigidBody::Dynamic,
                    Collider::rectangle(1.0, 1.0),
                    Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
                ),
                MotionState::Liquid => (
                    RigidBody::Dynamic,
                    Collider::circle(0.5),
                    Mesh2d(meshes.add(Circle::new(0.5))),
                ),
                MotionState::Gas => (
                    RigidBody::Dynamic,
                    Collider::circle(0.1),
                    Mesh2d(meshes.add(Circle::new(0.1))),
                ),
            },
            MeshMaterial2d(materials.add(element.color)),
            Name::new(format!("{} particle", element.name)),
        ));
    }
}
