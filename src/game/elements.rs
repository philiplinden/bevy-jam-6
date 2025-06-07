use std::hash::Hash;

use bevy::{
    color::palettes::css::{BISQUE, GREY, MAROON, ORANGE_RED, ROYAL_BLUE, TAN, WHITE},
    prelude::*,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementType {
    #[default]
    Powder,
    Sand,
    Water,
    Oil,
    Fire,
    Steam,
    Wall,
}

/// Indicates whether the particle is frozen in place or free to move around.
/// Things like walls and ice are frozen, while sand and water are not.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffusionRule {
    Frozen,  // Static particles (walls, barriers)
    Fall,    // Slide along other particles with friction, like sand or powder
    Fill,    // Roll along other particles to fill the container, like water or oil
    Diffuse, // Randomly move in any direction, like fire or steam
}

/// The properties of an element
#[derive(Component, Debug, Clone, PartialEq)]
pub struct Element {
    pub color: Color,
    pub diffusion_rule: DiffusionRule,
    pub density: f32
}

impl Element {
    pub fn from_type(element_type: ElementType) -> Self {
        match element_type {
            ElementType::Powder => Self {
                color: BISQUE.into(),
                diffusion_rule: DiffusionRule::Fall,
                density: 1.0,
            },
            ElementType::Sand => Self {
                color: TAN.into(),
                diffusion_rule: DiffusionRule::Fall,
                density: 1.5,
            },
            ElementType::Water => Self {
                color: ROYAL_BLUE.into(),
                diffusion_rule: DiffusionRule::Fill,
                density: 1.0,
            },
            ElementType::Oil => Self {
                color: MAROON.into(),
                diffusion_rule: DiffusionRule::Fill,
                density: 0.8,
            },
            ElementType::Fire => Self {
                color: ORANGE_RED.into(),
                diffusion_rule: DiffusionRule::Diffuse,
                density: 0.5,
            },
            ElementType::Steam => Self {
                color: WHITE.into(),
                diffusion_rule: DiffusionRule::Diffuse,
                density: 0.1,
            },
            ElementType::Wall => Self {
                color: GREY.into(),
                diffusion_rule: DiffusionRule::Frozen,
                density: 2.0,
            },
        }
    }
}

/// Resource to keep track of the currently selected element type
/// This is used to determine what element to place when the user clicks on the grid.
#[derive(Resource, Debug, Default, Clone, PartialEq)]
pub struct SelectedElement(pub ElementType);
