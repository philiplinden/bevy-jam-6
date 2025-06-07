use crate::game::elements::{ElementType, SelectedElement};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

struct ElementButton {
    name: &'static str,
    color: egui::Color32,
    is_strong: bool,
    is_italics: bool,
    // action: &'static str,
}

impl ElementButton {
    fn new(name: &'static str, color: egui::Color32, _action: &'static str) -> Self {
        Self {
            name,
            color,
            is_strong: false,
            is_italics: false,
            // action,
        }
    }

    fn strong(mut self) -> Self {
        self.is_strong = true;
        self
    }

    fn italics(mut self) -> Self {
        self.is_italics = true;
        self
    }
}

pub fn element_dashboard(
    mut contexts: EguiContexts,
    mut selected_element: ResMut<SelectedElement>,
) {
    let ctx = contexts.ctx_mut();

    // Define all elements with their properties
    let elements = vec![
        // Elements
        ElementButton::new(
            "POWDER",
            egui::Color32::from_rgb(210, 180, 140),
            "Spawn Powder",
        ),
        ElementButton::new(
            "WATER",
            egui::Color32::from_rgb(64, 164, 223),
            "Spawn Water",
        )
        .strong(),
        ElementButton::new("FIRE", egui::Color32::from_rgb(255, 69, 0), "Spawn Fire").strong(),
        ElementButton::new("SEED", egui::Color32::from_rgb(139, 69, 19), "Spawn Seed"),
        ElementButton::new(
            "G-POWER",
            egui::Color32::from_rgb(148, 0, 211),
            "Spawn G-Power",
        )
        .strong(),
        ElementButton::new("FAN", egui::Color32::from_rgb(169, 169, 169), "Spawn Fan"),
        ElementButton::new("ICE", egui::Color32::from_rgb(176, 224, 230), "Spawn Ice"),
        ElementButton::new(
            "S-BALL",
            egui::Color32::from_rgb(255, 215, 0),
            "Spawn S-Ball",
        ),
        ElementButton::new("CLONE", egui::Color32::from_rgb(50, 205, 50), "Spawn Clone").italics(),
        ElementButton::new(
            "F-WORKS",
            egui::Color32::from_rgb(255, 20, 147),
            "Spawn F-Works",
        )
        .strong(),
        ElementButton::new("OIL", egui::Color32::BLACK, "Spawn Oil").strong(),
        ElementButton::new("C-4", egui::Color32::from_rgb(220, 20, 60), "Spawn C-4").strong(),
        ElementButton::new(
            "STONE",
            egui::Color32::from_rgb(128, 128, 128),
            "Spawn Stone",
        ),
        ElementButton::new("MAGMA", egui::Color32::from_rgb(255, 140, 0), "Spawn Magma").strong(),
        ElementButton::new("VIRUS", egui::Color32::from_rgb(0, 128, 0), "Spawn Virus").strong(),
        ElementButton::new("NITRO", egui::Color32::from_rgb(255, 165, 0), "Spawn Nitro").strong(),
        ElementButton::new("ANT", egui::Color32::from_rgb(139, 69, 19), "Spawn Ant").strong(),
        ElementButton::new("TORCH", egui::Color32::from_rgb(255, 140, 0), "Spawn Torch").strong(),
        ElementButton::new("GAS", egui::Color32::from_rgb(192, 192, 192), "Spawn Gas").italics(),
        ElementButton::new(
            "SOAPY",
            egui::Color32::from_rgb(255, 192, 203),
            "Spawn Soapy",
        ),
        ElementButton::new(
            "THUNDER",
            egui::Color32::from_rgb(255, 255, 0),
            "Spawn Thunder",
        )
        .strong(),
        ElementButton::new(
            "METAL",
            egui::Color32::from_rgb(192, 192, 192),
            "Spawn Metal",
        )
        .strong(),
        ElementButton::new("BOMB", egui::Color32::from_rgb(139, 0, 0), "Spawn Bomb").strong(),
        ElementButton::new("LASER", egui::Color32::from_rgb(255, 0, 0), "Spawn Laser"),
        ElementButton::new("ACID", egui::Color32::from_rgb(173, 255, 47), "Spawn Acid").strong(),
        ElementButton::new("VINE", egui::Color32::from_rgb(34, 139, 34), "Spawn Vine"),
        ElementButton::new("SALT", egui::Color32::from_rgb(248, 248, 255), "Spawn Salt"),
        ElementButton::new(
            "GLASS",
            egui::Color32::from_rgb(173, 216, 230),
            "Spawn Glass",
        ),
        ElementButton::new("BIRD", egui::Color32::from_rgb(255, 165, 0), "Spawn Bird"),
        ElementButton::new(
            "MERCURY",
            egui::Color32::from_rgb(192, 192, 192),
            "Spawn Mercury",
        )
        .strong(),
        ElementButton::new("SPARK", egui::Color32::from_rgb(255, 255, 0), "Spawn Spark").strong(),
        ElementButton::new("FUSE", egui::Color32::from_rgb(139, 69, 19), "Spawn Fuse"),
        ElementButton::new(
            "CLOUD",
            egui::Color32::from_rgb(220, 220, 220),
            "Spawn Cloud",
        ),
        ElementButton::new("PUMP", egui::Color32::from_rgb(105, 105, 105), "Spawn Pump"),
        ElementButton::new("WIND", egui::Color32::from_rgb(135, 206, 235), "Spawn Wind").italics(),
        ElementButton::new("AIR", egui::Color32::from_rgb(230, 230, 250), "Spawn Air"),
        ElementButton::new("DRAG", egui::Color32::from_rgb(75, 0, 130), "Spawn Drag").strong(),
        ElementButton::new(
            "BUBBLE",
            egui::Color32::from_rgb(173, 216, 230),
            "Spawn Bubble",
        ),
        ElementButton::new("WHEEL", egui::Color32::from_rgb(139, 69, 19), "Spawn Wheel").strong(),
        ElementButton::new(
            "PLAYER",
            egui::Color32::from_rgb(0, 191, 255),
            "Spawn Player",
        )
        .strong(),
        ElementButton::new(
            "FIGHTER",
            egui::Color32::from_rgb(255, 0, 0),
            "Spawn Fighter",
        )
        .strong(),
        ElementButton::new("BOX", egui::Color32::from_rgb(160, 82, 45), "Spawn Box").strong(),
        ElementButton::new("BALL", egui::Color32::from_rgb(255, 165, 0), "Spawn Ball").strong(),
        ElementButton::new(
            "CREATE",
            egui::Color32::from_rgb(138, 43, 226),
            "Spawn Create",
        )
        .strong(),
    ];

    let tools = vec![
        ElementButton::new("ERASE", egui::Color32::WHITE, "Erase").strong(),
        ElementButton::new("COPY/PASTE", egui::Color32::WHITE, "Copy/Paste"),
        ElementButton::new("TEXT", egui::Color32::WHITE, "Text"),
        ElementButton::new("PEN type", egui::Color32::WHITE, "Pen type"),
        ElementButton::new("PEN THICKNESS", egui::Color32::WHITE, "Pen Thickness"),
        ElementButton::new("SCALE", egui::Color32::WHITE, "Scale"),
        ElementButton::new("SPEED", egui::Color32::WHITE, "Speed"),
        ElementButton::new("StartStop", egui::Color32::WHITE, "StartStop"),
        ElementButton::new("SAVE", egui::Color32::WHITE, "Save").strong(),
        ElementButton::new("LOAD", egui::Color32::WHITE, "Load"),
        ElementButton::new("GRID", egui::Color32::WHITE, "Grid"),
        ElementButton::new("EFFECT", egui::Color32::WHITE, "Effect"),
        ElementButton::new("RESET", egui::Color32::WHITE, "Reset"),
    ];

    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let available_width = ui.available_width();
            let button_width = 80.0; // Approximate button width
            let spacing = 8.0; // Spacing between columns
            let max_possible_columns =
                ((available_width + spacing) / (button_width + spacing)).floor() as usize;

            // Combine all buttons
            let all_buttons: Vec<&ElementButton> = elements.iter().chain(tools.iter()).collect();
            let total_items = all_buttons.len();

            // Don't create more columns than we have items, and ensure at least 1 column
            let num_columns = max_possible_columns.min(total_items).max(1);
            let items_per_column = total_items.div_ceil(num_columns); // Ceiling division

            // Create columns
            for col in 0..num_columns {
                let start_idx = col * items_per_column;

                // Skip empty columns
                if start_idx >= total_items {
                    break;
                }

                ui.vertical(|ui| {
                    let end_idx = ((col + 1) * items_per_column).min(total_items);

                    for button in &all_buttons[start_idx..end_idx] {
                        let is_selected = if let Some(element_type) =
                            element_type_from_button_name(button.name)
                        {
                            selected_element.0 == element_type
                        } else {
                            false
                        };
                        let mut rich_text = egui::RichText::new(button.name)
                            .color(button.color)
                            .size(14.0);
                        if button.is_strong {
                            rich_text = rich_text.strong();
                        }
                        if button.is_italics {
                            rich_text = rich_text.italics();
                        }
                        let button_response = if is_selected {
                            ui.add(
                                egui::Button::new(rich_text)
                                    .stroke(egui::Stroke::new(2.0, egui::Color32::YELLOW)),
                            )
                        } else {
                            ui.add(egui::Button::new(rich_text))
                        };
                        if button_response.clicked() {
                            if let Some(element_type) = element_type_from_button_name(button.name) {
                                selected_element.0 = element_type;
                                info!("Selected element: {:?}", selected_element.0);
                            }
                        }
                    }
                });

                // Add spacing between columns except for the last one
                if col < num_columns - 1 {
                    ui.add_space(spacing);
                }
            }
        });
    });
}

fn element_type_from_button_name(name: &str) -> Option<ElementType> {
    match name {
        "POWDER" => Some(ElementType::Powder),
        "SAND" => Some(ElementType::Sand),
        "WATER" => Some(ElementType::Water),
        "OIL" => Some(ElementType::Oil),
        "FIRE" => Some(ElementType::Fire),
        "STEAM" => Some(ElementType::Steam),
        "WALL" => Some(ElementType::Wall),
        _ => None,
    }
}
