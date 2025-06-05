
use bevy::{
    color::palettes::css::{AZURE, BISQUE, BROWN, GREY, HOT_PINK, ORANGE_RED, ROYAL_BLUE},
    prelude::*,
};

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
