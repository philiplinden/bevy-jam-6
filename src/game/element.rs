use bevy::{
    color::palettes::css::{BISQUE, BROWN, GREY, ROYAL_BLUE},
    prelude::*,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ElementTypes {
    #[default]
    Sand,
    Water,
    Wall,
    Oil,
}

/// Indicates whether the particle is frozen in place or free to move around.
/// Things like walls and ice are frozen, while sand and water are not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionState {
    Frozen, // Static particles (walls, barriers)
    Solid,  // Dynamic solid particles (sand, rocks)
    Liquid, // Liquid particles that flow
    Gas,    // Gas particles that diffuse
}

/// The properties of an element
#[derive(Component, Debug, Clone, PartialEq)]
pub struct Element {
    pub color: Color,
    pub motion_state: MotionState,
}

impl Element {
    pub fn from_type(element_type: ElementTypes) -> Self {
        match element_type {
            ElementTypes::Sand => Self {
                color: BISQUE.into(),
                motion_state: MotionState::Solid,
            },
            ElementTypes::Water => Self {
                color: ROYAL_BLUE.into(),
                motion_state: MotionState::Liquid,
            },
            ElementTypes::Wall => Self {
                color: GREY.into(),
                motion_state: MotionState::Frozen,
            },
            ElementTypes::Oil => Self {
                color: BROWN.into(),
                motion_state: MotionState::Liquid,
            },
        }
    }
}

#[derive(Resource, Debug, Default, Clone, PartialEq)]
pub struct SelectedElement(pub ElementTypes);
