//! Grid drawing and interaction logic

use eframe::egui::{vec2, Align2, Color32, Context, FontId, Key, Rect, Stroke, Ui};
use enigo::Enigo;

use crate::input::{keyboard, mouse};
use crate::models::cell::{CellPlural, CellSingular, FocusedCell};
use crate::models::margin::Margin;

/// Generate all possible letter combinations for the grid
pub fn generate_letter_combinations() -> Vec<CellPlural> {
    let mut combos = Vec::with_capacity(26 * 26);
    let letters = ('A'..='Z').collect::<Vec<_>>();

    for &a in &letters {
        for &b in &letters {
            combos.push(CellPlural::with_chars(a, b));
        }
    }

    combos
}

/// Draw the main grid
pub fn draw_grid(
    ctx: &Context,
    ui: &mut Ui,
    cells: &[CellPlural],
    focused_cell: &mut FocusedCell,
    is_visible: &mut bool,
    coordinates_margin: &Margin,
    transparency: u8,
    exit_key: Key,
) {
    let available_size = ui.available_size_before_wrap();

    let desired_cell_size = 64.0;

    let cols = (available_size.x / desired_cell_size).floor().max(1.0) as usize;
    let rows = (available_size.y / desired_cell_size).floor().max(1.0) as usize;

    let cell_width = available_size.x / cols as f32;
    let cell_height = available_size.y / rows as f32;

    let origin = ui.min_rect().min;
    let transparent_color = Color32::from_rgba_unmultiplied(255, 235, 200, transparency);

    // Check for escape key to reset focused cell
    if ctx.input(|i| i.key_pressed(exit_key)) {
        focused_cell.reset();
    }

    // Check for letter keys to update focused cell
    let letters = ('A'..='Z').collect::<Vec<_>>();
    let mut enigo = mouse::create_enigo().unwrap_or_else(|e| {
        eprintln!("Failed to create Enigo: {}", e);
        panic!("Could not initialize mouse control");
    });

    // Handle key presses for the first level of selection
    if !focused_cell.has_conclusion() {
        for &a in &letters {
            if let Some(key) = keyboard::key_from_char(a) {
                if ctx.input(|i| i.key_released(key)) {
                    if !focused_cell.has_first() {
                        focused_cell.first = a;
                    } else if !focused_cell.has_last() {
                        focused_cell.last = a;
                    }
                }
            }
        }
    }

    // Draw the grid cells
    let mut index = 0;
    for row in 0..rows {
        for col in 0..cols {
            if index >= cells.len() {
                return;
            }

            let first = cells[index].first;
            let last = cells[index].last;
            let combo = &cells[index].combo;

            // Only draw cells that match the current selection
            if !focused_cell.has_first() || focused_cell.first == first {
                if !focused_cell.has_last() || focused_cell.last == last {
                    let rect = Rect::from_min_size(
                        origin + vec2(col as f32 * cell_width, row as f32 * cell_height),
                        vec2(cell_width, cell_height),
                    );

                    ui.painter().rect(
                        rect,
                        0.0,
                        transparent_color,
                        Stroke::new(1.0, transparent_color),
                        eframe::egui::StrokeKind::Outside,
                    );

                    if focused_cell.is_complete() {
                        // Draw micro grid for the second level of selection
                        draw_micro_grid(
                            ctx,
                            cell_width,
                            cell_height,
                            ui,
                            rect,
                            &mut enigo,
                            coordinates_margin,
                            !focused_cell.has_conclusion(),
                            transparency,
                            || {
                                *is_visible = false;
                                ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Visible(
                                    false,
                                ));
                                focused_cell.reset();
                            },
                        );
                    } else {
                        // Draw the cell label
                        ui.painter().text(
                            rect.center(),
                            Align2::CENTER_CENTER,
                            combo,
                            FontId::monospace(cell_height * 0.4),
                            transparent_color,
                        );
                    }
                }
            }

            index += 1;
        }
    }
}

/// Draw the micro grid for the second level of selection
pub fn draw_micro_grid<F>(
    ctx: &Context,
    parent_cell_width: f32,
    parent_cell_height: f32,
    ui: &mut Ui,
    parent_rect: Rect,
    enigo: &mut Enigo,
    margins: &Margin,
    has_focus: bool,
    transparency: u8,
    mut on_keypress: F,
) where
    F: FnMut(),
{
    // Define the micro grid layout
    let single_cell_values = "QWERASDFUOIPJKL;";
    let cells: Vec<CellSingular> = single_cell_values
        .chars()
        .map(|c| CellSingular { unit: c })
        .collect();

    let length = cells.len();
    let cols: usize = 4;
    let rows: usize = 4;

    let cell_width = parent_cell_width / cols as f32;
    let cell_height = parent_cell_height / rows as f32;

    let origin = parent_rect.min;
    let transparent_color = Color32::from_rgba_unmultiplied(255, 235, 200, transparency);

    let pixels_per_point = ui.ctx().pixels_per_point();

    // Draw each cell in the micro grid
    for (index, cell) in cells.iter().enumerate() {
        if index >= length {
            break;
        }

        let row = index / cols;
        let col = index % cols;

        let rect = Rect::from_min_size(
            origin + vec2(col as f32 * cell_width, row as f32 * cell_height),
            vec2(cell_width, cell_height),
        );

        let pos = rect.center();

        // Calculate screen coordinates
        let coordinates = (
            ((pos.x + margins.left as f32) * pixels_per_point) as i32,
            ((pos.y + margins.top as f32) * pixels_per_point) as i32,
        );

        // Handle key press for this cell
        if has_focus {
            if let Some(key) = keyboard::key_from_char(cell.unit) {
                if ctx.input(|i| i.key_pressed(key)) {
                    if let Ok(()) = mouse::move_cursor_to(coordinates.0, coordinates.1, enigo) {
                        on_keypress();
                    }
                }
            }
        }

        // Draw the cell
        ui.painter().rect(
            rect,
            0.0,
            transparent_color,
            Stroke::new(1.0, transparent_color),
            eframe::egui::StrokeKind::Middle,
        );

        // Draw the cell label
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            cell.unit,
            FontId::monospace(cell_height * 0.4),
            transparent_color,
        );
    }
}
