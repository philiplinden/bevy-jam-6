use avian2d::prelude::*;
use bevy::{input::common_conditions::input_pressed, prelude::*};

use super::elements::{DiffusionRule, Element, ElementType, SelectedElement};
use super::sandbox::ScreenWrap;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default());
    app.add_systems(
        Update,
        (
            spawn_particle_on_click.run_if(input_pressed(MouseButton::Left)),
            setup_particle_visuals,
        ),
    );

    app.insert_resource(SelectedElement(ElementType::Sand));
}

#[derive(Bundle, Debug, Clone)]
pub struct Particle {
    pub element: Element,
    collider: Collider,
    rigid_body: RigidBody,
    transform: Transform,
}

impl Particle {
    pub fn new(element: Element, position: Vec2) -> Self {
        let (collider, rigid_body) = match element.diffusion_rule {
            DiffusionRule::Frozen => (Collider::rectangle(1.0, 1.0), RigidBody::Static),
            DiffusionRule::Fall => (Collider::rectangle(1.0, 1.0), RigidBody::Dynamic),
            DiffusionRule::Fill => (Collider::circle(0.5), RigidBody::Dynamic),
            DiffusionRule::Diffuse => (Collider::circle(0.1), RigidBody::Dynamic),
        };
        Self {
            element,
            collider,
            rigid_body,
            transform: Transform::from_translation(position.extend(0.0)),
        }
    }
}

fn setup_particle_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &Element), Added<Element>>,
) {
    for (entity, element) in query.iter() {
        let mesh_handle = match element.diffusion_rule {
            DiffusionRule::Frozen | DiffusionRule::Fall => meshes.add(Rectangle::new(1.0, 1.0)),
            DiffusionRule::Fill => meshes.add(Circle::new(0.5)),
            DiffusionRule::Diffuse => meshes.add(Circle::new(0.1)),
        };
        let material_handle = materials.add(ColorMaterial::from_color(element.color));
        commands
            .entity(entity)
            .insert((Mesh2d(mesh_handle), MeshMaterial2d(material_handle)));
    }
}

fn spawn_particle_on_click(
    mut commands: Commands,
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    selected_element: Res<SelectedElement>,
) {
    if let Ok(window) = windows.single() {
        if let Some(cursor_position) = window.cursor_position() {
            if let Ok((camera, camera_transform)) = camera.single() {
                if let Ok(world_position) =
                    camera.viewport_to_world_2d(camera_transform, cursor_position)
                {
                    let element = Element::from_type(selected_element.0);
                    commands.spawn((Particle::new(element, world_position), ScreenWrap));
                }
            }
        }
    }
}
