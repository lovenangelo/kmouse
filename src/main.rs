use eframe::{
    egui::{
        accesskit::Vec2, vec2, Align2, CentralPanel, Color32, Context, FontId, Frame, Grid, Id,
        Key, Layout, Margin, PointerButton, Rect, Rgba, RichText, Sense, Stroke, TextBuffer, Ui,
        UiBuilder, ViewportBuilder, ViewportClass, ViewportCommand, ViewportId, Visuals, Window,
    },
    epaint::text,
    App, NativeOptions,
};

fn main() -> eframe::Result {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_mouse_passthrough(true)
            .with_transparent(true)
            .with_titlebar_shown(false)
            .with_decorations(false)
            .with_fullscreen(true),
        ..Default::default()
    };
    eframe::run_native(
        "Kmouse",
        native_options,
        Box::new(|_cc| Ok(Box::<Kmouse>::default())),
    )
}

struct Kmouse {
    title: String,
    cells: Vec<CellPlural>,
}

struct CellPlural {
    combo: String,
    first: char,
    last: char,
}

impl CellPlural {
    fn new() -> Self {
        Self {
            combo: String::new(),
            first: char::default(),
            last: char::default(),
        }
    }
}

struct CellSingular {
    unit: char,
}

impl Kmouse {
    fn generate_letter_combinations() -> Vec<CellPlural> {
        let mut combos: Vec<CellPlural> = vec![];
        let letters = ('A'..='Z').collect::<Vec<_>>();
        let symbols = vec!['@', '#', '$', '%', '&'];

        for &a in &letters {
            for &b in &letters {
                let mut cell = CellPlural::new();
                let combo = format!("{}{}", a, b);
                cell.combo = combo;
                cell.first = a;
                cell.last = b;
                combos.push(cell);
            }
            for &b in &symbols {
                let mut cell = CellPlural::new();
                let combo = format!("{}{}", a, b);
                cell.combo = combo;
                cell.first = a;
                cell.last = b;
                combos.push(cell);
            }
        }

        combos
    }

    fn draw_grid(&mut self, ctx: &Context, ui: &mut Ui) {
        let available_size = ui.available_size();
        let cells = &self.cells;

        // You can control the base "feel" of cell size here:
        let desired_cell_size = 64.0;

        // Calculate number of cols and rows to fill screen
        let cols = (available_size.x / desired_cell_size).floor().max(1.0) as usize;
        let rows = (available_size.y / desired_cell_size).floor().max(1.0) as usize;

        // Recalculate cell size so grid fits perfectly
        let cell_width = available_size.x / cols as f32;
        let cell_height = available_size.y / rows as f32;

        let origin = ui.min_rect().min;
        let semi_transparent_white = Color32::from_rgba_unmultiplied(255, 255, 255, 64);

        let mut index = 0;

        for row in 0..rows {
            for col in 0..cols {
                if index >= cells.len() {
                    return;
                }

                let first = &cells[index].first;
                let combo = &cells[index].combo;

                if ctx.input(|i| {
                    let mut tmp = [0; 4];
                    i.key_pressed(
                        Key::from_name(first.encode_utf8(&mut tmp))
                            .expect(format!("invalid name {}", first).as_str()),
                    )
                }) {
                    self.title = String::from("key pressed");
                }
                let rect = Rect::from_min_size(
                    origin + vec2(col as f32 * cell_width, row as f32 * cell_height),
                    vec2(cell_width, cell_height),
                );

                ui.painter().rect(
                    rect,
                    0.0,
                    semi_transparent_white,
                    Stroke::new(1.0, Color32::WHITE),
                    eframe::egui::StrokeKind::Middle,
                );

                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    combo,
                    FontId::monospace(cell_height * 0.4),
                    Color32::WHITE,
                );

                index += 1;
            }
        }
    }
}

impl Default for Kmouse {
    fn default() -> Self {
        Self {
            cells: Self::generate_letter_combinations(),
            title: String::from("Kmouse"),
        }
    }
}

impl App for Kmouse {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            let app_rect = ui.max_rect();

            /*
                        let title_bar_height = 32.0;
                        let title_bar_rect = {
                            let mut rect = app_rect;
                            rect.max.y = rect.min.y + title_bar_height;
                            rect
                        };
            */
            self.draw_grid(ctx, ui);
        });
    }
}

fn title_bar_ui(ui: &mut Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
    let painter = ui.painter();

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );
}
