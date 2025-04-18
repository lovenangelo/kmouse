use std::{
    sync::{Arc, Mutex},
    thread,
};

use eframe::{
    egui::{
        vec2, Align2, CentralPanel, Color32, Context, FontId, Frame, Key, Margin, Rect, Stroke, Ui,
        ViewportBuilder,
    },
    App, NativeOptions,
};

use enigo::{Enigo, Keyboard, Mouse, Settings};
use once_cell::sync::OnceCell;
use rdev::{listen, EventType};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt};
use x11rb::rust_connection::RustConnection;

fn get_work_area() -> Result<(u32, u32, u32, u32), Box<dyn std::error::Error>> {
    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    // Intern the _NET_WORKAREA atom
    let atom_reply = conn.intern_atom(false, b"_NET_WORKAREA")?.reply()?;
    let atom = atom_reply.atom;

    // Get the property from the root window
    let prop = conn
        .get_property(false, screen.root, atom, AtomEnum::CARDINAL, 0, 4)?
        .reply()?;

    if let Some(data) = prop.value32() {
        let values: Vec<u32> = data.collect();
        if values.len() >= 4 {
            let x = values[0];
            let y = values[1];
            let width = values[2];
            let height = values[3];
            return Ok((x, y, width, height));
        }
    }

    Err("Failed to retrieve _NET_WORKAREA".into())
}

fn main() -> eframe::Result {
    let (work_x, work_y, work_width, work_height) = get_work_area().unwrap();
    let screen_width = 1920; // or detect this dynamically
    let screen_height = 1080;

    let margin_top = work_y;
    let margin_bottom = screen_height - (work_y + work_height);
    let margin_left = work_x;
    let margin_right = screen_width - (work_x + work_width);

    let kmargin_frame = KmouseMargin {
        top: margin_top as i8,
        left: margin_left as i8,
        right: margin_right as i8,
        bottom: margin_bottom as i8,
    };
    let kmargin_coordinate = KmouseMargin {
        top: margin_top as i8,
        left: margin_left as i8,
        right: margin_right as i8,
        bottom: margin_bottom as i8,
    };
    let base_margine = KmouseMargin {
        top: margin_top as i8,
        left: margin_left as i8,
        right: margin_right as i8,
        bottom: margin_bottom as i8,
    };

    let mut kmouse = Kmouse::default();
    kmouse.frame_margin = kmargin_frame;
    kmouse.coordinates_margin = kmargin_coordinate;
    kmouse.base_margin = base_margine;
    let visible_clone = Arc::clone(&kmouse.is_visible);
    let has_started_clone = Arc::clone(&kmouse.has_completed);
    let focused_cell_clone = Arc::clone(&kmouse.focused_cell);
    thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            let mut vis = visible_clone.lock().unwrap();
            let mut has_started = has_started_clone.lock().unwrap();
            let mut focused_cell = focused_cell_clone.lock().unwrap();
            if let EventType::KeyPress(key) = event.event_type {
                if let Some(ctx) = CTX_CELL.get() {
                    if key == rdev::Key::ControlRight {
                        *vis = !*vis;
                        if *vis {
                            if !*has_started {
                                *has_started = true;
                            }
                            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Visible(true));
                            *focused_cell = FocusedCell::new();
                        }
                        if !*vis {
                            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Visible(false));
                        }
                    }
                }
            }
        }) {
            eprintln!("Error: {:?}", error);
        }
    });

    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_mouse_passthrough(true)
            .with_fullscreen(true)
            .with_transparent(true)
            .with_titlebar_shown(false)
            .with_always_on_top()
            .with_decorations(false),
        ..Default::default()
    };

    eframe::run_native(
        "Kmouse",
        native_options,
        Box::new(|_cc| Ok(Box::new(kmouse))),
    )
}

#[derive(Debug)]
struct Kmouse {
    cells: Vec<CellPlural>,
    focused_cell: Arc<Mutex<FocusedCell>>,
    is_visible: Arc<Mutex<bool>>,
    coordinates_margin: KmouseMargin,
    base_margin: KmouseMargin,
    frame_margin: KmouseMargin,
    has_completed: Arc<Mutex<bool>>,
}

#[derive(Debug)]
struct KmouseMargin {
    top: i8,
    left: i8,
    right: i8,
    bottom: i8,
}

#[derive(Debug)]
struct CellPlural {
    combo: String,
    first: char,
    last: char,
}

#[derive(Debug)]
struct FocusedCell {
    first: char,
    last: char,
    conclusion: char,
}

impl FocusedCell {
    fn new() -> Self {
        Self {
            first: char::default(),
            last: char::default(),
            conclusion: char::default(),
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

static CTX_CELL: OnceCell<Arc<Context>> = OnceCell::new();

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
        let available_size = ui.available_size_before_wrap();
        let cells = &self.cells;

        let desired_cell_size = 64.0;

        let cols = (available_size.x / desired_cell_size).floor().max(1.0) as usize;
        let rows = (available_size.y / desired_cell_size).floor().max(1.0) as usize;

        let cell_width = available_size.x / cols as f32;
        let cell_height = available_size.y / rows as f32;

        let origin = ui.min_rect().min;
        let transparent_color = Color32::from_rgba_unmultiplied(255, 235, 200, 10);
        let mut index = 0;

        let mut vis = self.is_visible.lock().unwrap();

        let mut focused_cell = self.focused_cell.lock().unwrap();

        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            *focused_cell = FocusedCell::new();
        }
        let letters = ('A'..='Z').collect::<Vec<_>>();
        let mut enigo = Enigo::new(&Settings::default()).unwrap();

        if focused_cell.conclusion == '\0' {
            for &a in &letters {
                if ctx.input(|i| {
                    i.key_released(
                        Key::from_name(&a.to_string().as_str())
                            .expect(format!("invalid name {}", &a).as_str()),
                    )
                }) {
                    if focused_cell.first == '\0' || focused_cell.last == '\0' {
                        if focused_cell.first != '\0' {
                            focused_cell.last = a;
                        }
                        if focused_cell.first == '\0' {
                            focused_cell.first = a;
                        }
                    }
                }
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

                if focused_cell.first == '\0'
                    || (focused_cell.first != '\0' && &focused_cell.first == first)
                {
                    if focused_cell.last == '\0'
                        || (focused_cell.last != '\0' && &focused_cell.last == last)
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
                            eframe::egui::StrokeKind::Outside,
                        );
                        if focused_cell.first != '\0' && focused_cell.last != '\0' {
                            draw_micro_grids(
                                ctx,
                                cell_width,
                                cell_height,
                                ui,
                                rect,
                                &mut enigo,
                                &self.coordinates_margin,
                                focused_cell.first != '\0'
                                    && focused_cell.last != '\0'
                                    && focused_cell.conclusion == '\0',
                                || {
                                    *vis = false;
                                    ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Visible(
                                        false,
                                    ));
                                    *focused_cell = FocusedCell::new();
                                },
                            );
                        } else {
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
}

fn move_cursor_to(x: i32, y: i32, enigo: &mut Enigo) -> bool {
    enigo
        .move_mouse(x, y, enigo::Coordinate::Abs)
        .expect("invalid coordinates");
    enigo
        .button(enigo::Button::Left, enigo::Direction::Click)
        .is_ok()
}

fn draw_micro_grids<F>(
    ctx: &Context,
    parent_cell_width: f32,
    parent_cell_height: f32,
    ui: &mut Ui,
    parent_rect: Rect,
    enigo: &mut Enigo,
    kmargins: &KmouseMargin,
    has_focus: bool,
    mut on_keypress: F,
) where
    F: FnMut(),
{
    let single_cell_values: &str = "QWERASDFUOIPJKL;";
    let cells: Vec<CellSingular> = single_cell_values
        .chars()
        .map(|c| CellSingular { unit: c })
        .collect();
    let length = single_cell_values.chars().count();

    let cols: usize = 4;
    let rows: usize = 4;

    let cell_width = parent_cell_width / cols as f32;
    let cell_height = parent_cell_height / rows as f32;

    let origin = parent_rect.min;
    let transparent_color = Color32::from_rgba_unmultiplied(255, 235, 200, 10);

    let mut index = 0;

    let pixels_per_point = ui.ctx().pixels_per_point();

    for row in 0..rows {
        for col in 0..cols {
            if index >= length {
                return;
            }
            let first = &cells[index];

            let rect = Rect::from_min_size(
                origin + vec2(col as f32 * cell_width, row as f32 * cell_height),
                vec2(cell_width, cell_height),
            );

            let pos = rect.center();

            let coordinates = (
                ((pos.x + kmargins.left as f32) * pixels_per_point) as i32,
                ((pos.y + kmargins.top as f32) * pixels_per_point) as i32,
            );
            if has_focus {
                if ctx.input(|i| {
                    let mut tmp = [0; 4];
                    i.key_pressed(
                        Key::from_name(first.unit.encode_utf8(&mut tmp))
                            .expect(format!("invalid {}", first.unit).as_str()),
                    )
                }) {
                    if move_cursor_to(coordinates.0, coordinates.1, enigo) {
                        on_keypress();
                    }
                }
            }

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

            index += 1;
        }
    }
}

impl Default for Kmouse {
    fn default() -> Self {
        Self {
            cells: Self::generate_letter_combinations(),
            focused_cell: Arc::new(Mutex::new(FocusedCell::new())),
            has_completed: Arc::new(Mutex::new(false)),
            is_visible: Arc::new(Mutex::new(true)),
            base_margin: KmouseMargin {
                top: 0,
                left: 0,
                right: 0,
                bottom: 0,
            },
            coordinates_margin: KmouseMargin {
                top: 0,
                left: 0,
                right: 0,
                bottom: 0,
            },
            frame_margin: KmouseMargin {
                top: 0,
                left: 0,
                right: 0,
                bottom: 0,
            },
        }
    }
}

impl App for Kmouse {
    fn clear_color(&self, _visuals: &eframe::egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let _ = CTX_CELL.set(Arc::new(ctx.clone()));
        let transparent_color = Color32::from_rgba_unmultiplied(30, 60, 90, 89);
        let has_started_mutex = self.has_completed.lock().unwrap();
        let has_started = *has_started_mutex;
        drop(has_started_mutex);
        let margin = if has_started {
            let margin = Margin::ZERO;
            self.coordinates_margin = KmouseMargin {
                top: self.base_margin.top,
                left: self.base_margin.left,
                right: self.base_margin.right,
                bottom: self.base_margin.bottom,
            };
            margin
        } else {
            let margin = Margin {
                top: self.frame_margin.top,
                left: self.frame_margin.left,
                right: self.frame_margin.right,
                bottom: self.frame_margin.bottom,
            };
            self.coordinates_margin = KmouseMargin {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            margin
        };

        let transparent_frame = Frame {
            fill: transparent_color,
            ..Frame::default()
        }
        .outer_margin(margin);

        ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Fullscreen(true));
        CentralPanel::default()
            .frame(transparent_frame)
            .show(ctx, |ui| {
                self.draw_grid(ctx, ui);
            });
    }
}
