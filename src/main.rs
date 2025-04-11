use eframe::{
    egui::{
        vec2, Align2, CentralPanel, Color32, Context, FontId, Frame, Key, Rect, Stroke, Ui,
        ViewportBuilder,
    },
    App, NativeOptions,
};

const SINGLE_CELL_VAlUES: &str = "qweruiopasdfjkl;";

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
    focused_cell: FocusedCell,
}

struct CellPlural {
    combo: String,
    first: char,
    last: char,
}

#[derive(Debug)]
struct FocusedCell {
    first: char,
    last: char,
}

impl FocusedCell {
    fn new() -> Self {
        Self {
            first: char::default(),
            last: char::default(),
        }
    }
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

#[derive(Debug)]
struct CellSingular {
    unit: char,
}

impl Kmouse {
    fn generate_letter_combinations() -> Vec<CellPlural> {
        let mut combos: Vec<CellPlural> = vec![];
        let letters = ('A'..='Z').collect::<Vec<_>>();

        for &a in &letters {
            for &b in &letters {
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

        let desired_cell_size = 64.0;

        let cols = (available_size.x / desired_cell_size).floor().max(1.0) as usize;
        let rows = (available_size.y / desired_cell_size).floor().max(1.0) as usize;

        let cell_width = available_size.x / cols as f32;
        let cell_height = available_size.y / rows as f32;

        let origin = ui.min_rect().min;
        let transparent_color = Color32::from_rgba_unmultiplied(255, 235, 200, 10);

        let mut index = 0;

        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            self.focused_cell = FocusedCell::new();
        }
        let letters = ('A'..='Z').collect::<Vec<_>>();

        for &a in &letters {
            if ctx.input(|i| {
                i.key_pressed(
                    Key::from_name(&a.to_string().as_str())
                        .expect(format!("invalid name {}", &a).as_str()),
                )
            }) {
                if self.focused_cell.first != '\0' {
                    self.focused_cell.last = a;
                }
                if self.focused_cell.first == '\0' {
                    self.focused_cell.first = a;
                }
                println!("{:?}", self.focused_cell)
            }
        }

        for row in 0..rows {
            for col in 0..cols {
                if index >= cells.len() {
                    return;
                }
                let first = &cells[index].first;
                let last = &cells[index].last;
                let combo = &cells[index].combo;

                if self.focused_cell.first == '\0'
                    || (self.focused_cell.first != '\0' && &self.focused_cell.first == first)
                {
                    if self.focused_cell.last == '\0'
                        || (self.focused_cell.last != '\0' && &self.focused_cell.last == last)
                    {
                        let rect = Rect::from_min_size(
                            origin + vec2(col as f32 * cell_width, row as f32 * cell_height),
                            vec2(cell_width, cell_height),
                        );

                        ui.painter().rect(
                            rect,
                            0.0,
                            transparent_color,
                            Stroke::new(1.0, transparent_color),
                            eframe::egui::StrokeKind::Middle,
                        );

                        ui.painter().text(
                            rect.center(),
                            Align2::CENTER_CENTER,
                            combo,
                            FontId::monospace(cell_height * 0.4),
                            transparent_color,
                        );
                    }
                }

                index += 1;
            }
        }
    }

    fn draw_micro_grids(
        &mut self,
        ctx: &Context,
        parent_cell_width: f32,
        parent_cell_height: f32,
        ui: &mut Ui,
    ) {
        let cells: Vec<CellSingular> = SINGLE_CELL_VAlUES
            .chars()
            .map(|c| CellSingular { unit: c })
            .collect();
        let length = SINGLE_CELL_VAlUES.chars().count();

        let cols: usize = 4;
        let rows: usize = 4;

        let cell_width = parent_cell_width / cols as f32;
        let cell_height = parent_cell_height / rows as f32;

        let origin = ui.min_rect().min;
        let transparent_color = Color32::from_rgba_unmultiplied(255, 235, 200, 10);

        let mut index = 0;

        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            self.focused_cell = FocusedCell::new();
        }

        for row in 0..rows {
            for col in 0..cols {
                if index >= length {
                    return;
                }
                let first = &cells[index];

                if self.focused_cell.first == '\0'
                    || (self.focused_cell.first != '\0' && &self.focused_cell.first == &first.unit)
                {
                    if ctx.input(|i| {
                        let mut tmp = [0; 4];
                        i.key_pressed(
                            Key::from_name(first.unit.encode_utf8(&mut tmp))
                                .expect(format!("invalid {}", first.unit).as_str()),
                        )
                    }) {
                        println!("{:?}", first);
                    }
                    let rect = Rect::from_min_size(
                        origin + vec2(col as f32 * cell_width, row as f32 * cell_height),
                        vec2(cell_width, cell_height),
                    );

                    ui.painter().rect(
                        rect,
                        0.0,
                        transparent_color,
                        Stroke::new(1.0, transparent_color),
                        eframe::egui::StrokeKind::Middle,
                    );

                    ui.painter().text(
                        rect.center(),
                        Align2::CENTER_CENTER,
                        first.unit,
                        FontId::monospace(cell_height * 0.4),
                        transparent_color,
                    );
                }

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
            focused_cell: FocusedCell::new(),
        }
    }
}

impl App for Kmouse {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            self.draw_grid(ctx, ui);
        });
    }
}
