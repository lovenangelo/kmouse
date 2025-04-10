use eframe::{
    egui::{
        accesskit::Vec2, vec2, Align2, CentralPanel, Color32, Context, FontId, Frame, Grid, Id,
        Layout, Margin, PointerButton, Rect, Rgba, RichText, Sense, Stroke, Ui, UiBuilder,
        ViewportBuilder, ViewportClass, ViewportCommand, ViewportId, Visuals, Window,
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
    show_overlay: bool,
    letters: Vec<String>,
}

impl Kmouse {
    fn generate_letter_combinations() -> Vec<String> {
        let mut combos = vec![];
        let letters = ('A'..='Z').collect::<Vec<_>>();
        let symbols = vec!['@', '#', '$', '%', '&'];

        for &a in &letters {
            for &b in &letters {
                combos.push(format!("{}{}", a, b));
            }
            for &b in &symbols {
                combos.push(format!("{}{}", a, b));
            }
        }

        combos
    }
}

impl Default for Kmouse {
    fn default() -> Self {
        Self {
            show_overlay: false,
            letters: Self::generate_letter_combinations(),
        }
    }
}

impl App for Kmouse {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if !self.show_overlay {
            CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
                let app_rect = ui.max_rect();

                let title_bar_height = 32.0;
                let title_bar_rect = {
                    let mut rect = app_rect;
                    rect.max.y = rect.min.y + title_bar_height;
                    rect
                };

                let title = "KMOUSE";

                title_bar_ui(ui, title_bar_rect, title);

                let available_size = ui.available_size();
                let total_cells = self.letters.len();

                // Estimate grid dimensions
                let cols = (total_cells as f32).sqrt().ceil() as usize;
                let cell_size = (available_size.x / cols as f32).min(
                    (available_size.y - title_bar_height)
                        / (total_cells as f32 / cols as f32).ceil(),
                );
                draw_grid(ui, &self.letters);
            });
        }
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

fn draw_grid(ui: &mut Ui, combos: &[String]) {
    let available_size = ui.available_size();

    // You can control the base "feel" of cell size here:
    let desired_cell_size = 64.0;

    // Calculate number of cols and rows to fill screen
    let cols = (available_size.x / desired_cell_size).floor().max(1.0) as usize;
    let rows = (available_size.y / desired_cell_size).floor().max(1.0) as usize;
    let total_cells = cols * rows;

    // Recalculate cell size so grid fits perfectly
    let cell_width = available_size.x / cols as f32;
    let cell_height = available_size.y / rows as f32;

    let origin = ui.min_rect().min;
    let semi_transparent_white = Color32::from_rgba_unmultiplied(255, 255, 255, 128);

    let mut index = 0;

    for row in 0..rows {
        for col in 0..cols {
            if index >= combos.len() {
                return;
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
                &combos[index],
                FontId::monospace(cell_height * 0.4),
                Color32::WHITE,
            );

            index += 1;
        }
    }
}
