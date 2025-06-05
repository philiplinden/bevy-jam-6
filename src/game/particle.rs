use avian2d::prelude::*;
use bevy::{
    color::palettes::css::{AZURE, BISQUE, BROWN, GREY, HOT_PINK, ORANGE_RED, ROYAL_BLUE},
    input::common_conditions::input_pressed,
    picking::pointer::PointerLocation,
    prelude::*,
};
use bevy_rand::prelude::*;

use super::sandbox::ScreenWrap;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        EntropyPlugin::<WyRand>::default(),
        PhysicsPlugins::default(),
        PhysicsPickingPlugin::default(),
    ));
    app.add_systems(
        Update,
        (
            pointer_click_send_spawn_event.run_if(input_pressed(MouseButton::Left)),
            setup_particle_visuals,
            handle_spawn_particle_event,
        ),
    );

    app.insert_resource(SelectedElement(ElementTypes::Sand));
    app.add_event::<SpawnParticleEvent>();
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ElementTypes {
    #[default]
    Sand,
    Water,
    Wall,
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
#[derive(Bundle, Debug, Clone)]
pub struct Particle {
    pub element: Element,
    collider: Collider,
    rigid_body: RigidBody,
    transform: Transform,
    entropy: Entropy<WyRand>,
}

impl Particle {
    pub fn new(element: Element, position: Vec2) -> Self {
        let (collider, rigid_body) = match element.motion_state {
            MotionState::Frozen => (Collider::rectangle(1.0, 1.0), RigidBody::Static),
            MotionState::Solid => (Collider::rectangle(1.0, 1.0), RigidBody::Dynamic),
            MotionState::Liquid => (Collider::circle(0.5), RigidBody::Dynamic),
            MotionState::Gas => (Collider::circle(0.1), RigidBody::Dynamic),
        };
        Self {
            element,
            collider,
            rigid_body,
            transform: Transform::from_translation(position.extend(0.0)),
            entropy: Entropy::<WyRand>::default(),
        }
    }
}

/// When an element is added to an entity, this system sets the visual shape and color based on the element's properties.
fn setup_particle_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &Element, &Transform, &Name), Added<Element>>,
) {
    for (entity, element, transform, name) in query.iter() {
        info!("Setting up visuals for particle: {:?}", name);
        let mesh_handle = match element.motion_state {
            MotionState::Frozen | MotionState::Solid => meshes.add(Rectangle::new(1.0, 1.0)),
            MotionState::Liquid => meshes.add(Circle::new(0.5)),
            MotionState::Gas => meshes.add(Circle::new(0.1)),
        };
        let material_handle = materials.add(ColorMaterial::from_color(element.color));
        commands.entity(entity).insert((
            Mesh2d(mesh_handle),
            MeshMaterial2d(material_handle),
            *transform,
        ));
        info!("{:?} should have color {:?}", name, element.color);
    }
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

/// This system listens for the [`SpawnParticleEvent`] and spawns a particle entity with the specified element and position.
fn handle_spawn_particle_event(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnParticleEvent>,
) {
    for event in event_reader.read() {
        let element = event.element.clone();
        let position = event.position;
        info!("Spawning {:?} particle at position: {:?}", element.name, position);
        commands.spawn((
            Name::new(format!("{} particle", element.name)),
            Particle::new(element, position),
            ScreenWrap,
        ));
    }
}
