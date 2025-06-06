use bevy::prelude::*;

use super::elements::ElementType;
use std::collections::HashMap as Map;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ReactionRegistry>();
}

#[derive(Resource)]
pub struct ReactionRegistry {
    reactions: Map<(ElementType, ElementType), Reaction>,
}

impl Default for ReactionRegistry {
    fn default() -> Self {
        let mut registry = Self {
            reactions: Map::new(),
        };
        registry.register_reactions();
        registry
    }
}

impl ReactionRegistry {
    pub fn register_reaction(&mut self, reaction: Reaction) {
        let key = if reaction.reactants.len() == 2 {
            (reaction.reactants[0], reaction.reactants[1])
        } else {
            return; // Only support binary reactions for now
        };

        // Register both permutations
        self.reactions.insert(key, reaction);
        self.reactions.insert(
            (key.1, key.0),
            Reaction {
                reactants: vec![key.1, key.0],
                product: self.reactions[&key].product.clone(),
                energy_scalar: self.reactions[&key].energy_scalar,
            },
        );
    }

    pub fn find_reaction(&self, element1: ElementType, element2: ElementType) -> Option<&Reaction> {
        self.reactions.get(&(element1, element2))
    }

    fn register_reactions(&mut self) {
        // Water + Fire = Steam
        self.register_reaction(Reaction {
            reactants: vec![ElementType::Water, ElementType::Fire],
            product: ElementType::Steam,
            energy_scalar: 1.2,
        });
        self.register_reaction(Reaction {
            reactants: vec![ElementType::Oil, ElementType::Fire],
            product: ElementType::Fire,
            energy_scalar: 5.0,
        });
        self.register_reaction(Reaction {
            reactants: vec![ElementType::Powder, ElementType::Fire],
            product: ElementType::Sand,
            energy_scalar: 0.8,
        });
        self.register_reaction(Reaction {
            reactants: vec![ElementType::Water, ElementType::Powder],
            product: ElementType::Sand,
            energy_scalar: 1.0,
        });
        self.register_reaction(Reaction {
            reactants: vec![ElementType::Water, ElementType::Steam],
            product: ElementType::Water,
            energy_scalar: 0.8,
        });
    }
}

/// Represents the type of effect a reaction can have. For all reactions, the reactant particles are consumed and replaced by one or more new product particles. The effect determines how the reaction behaves in terms of energy and momentum conservation.
pub struct Reaction {
    pub reactants: Vec<ElementType>,
    pub product: ElementType,
    pub energy_scalar: f32, // The total energy of the system (momentum, heat, etc.) is scaled and applied to the product particles evenly distributed per unit mass. If intensity is between 0 and 1, the reaction is endothermic (absorbs energy). If intensity is greater than 1, the reaction is exothermic (releases energy). Must be greater than 0.
}

impl Default for Reaction {
    fn default() -> Self {
        Self {
            reactants: vec![],
            product: ElementType::Powder,
            energy_scalar: 1.0,
        }
    }
}
