use egui::{
    color, Align2, Color32, CursorIcon, FontId, MultiTouchInfo, PointerButton, Pos2, RichText,
    Stroke, Ui,
};

// TODO 1. Add presets
// TODO 2. Add radial color mode
// TODO 3. Add opacity control for lines
// TODO 4. Add point labels
// TODO 5. Gray out certain options when certain color modes are selected
// TODO 6. Change rendering algo to draw lines in following order: 0, last, 1, last-1, 2, last-2...
// TODO 7. Re order options menu some more
// TODO 8. Add reset button
// TODO 9. Remove show points radio - check if point size is 0 to achieve same effect

enum ColorMode {
    Monochrome,
    Length,
    Radial,
}

impl ColorMode {
    fn label(&self) -> &str {
        match *self {
            ColorMode::Monochrome => "Monochrome",
            ColorMode::Length => "Length",
            ColorMode::Radial => "Radial",
        }
    }
}

#[derive(PartialEq)]
enum Preset {
    Rainbow,
    Pencil,
    Educational,
}

impl Preset {
    fn style(&self) -> TimesCircleStyle {
        match *self {
            Preset::Rainbow => TimesCircleStyle {
                stroke: 0.02,
                color: Color32::BLACK,
                background_color: Color32::BLACK,
                color_mode: ColorMode::Length,
                perimeter_points_radius: 0.0,
                perimeter_point_color: Color32::BLACK,
            },
            Preset::Pencil => TimesCircleStyle {
                stroke: 0.10,
                color: Color32::BLACK,
                background_color: Color32::WHITE,
                color_mode: ColorMode::Monochrome,
                perimeter_points_radius: 0.0,
                perimeter_point_color: Color32::BLACK,
            },
            Preset::Educational => TimesCircleStyle {
                stroke: 1.0,
                color: Color32::BLACK,
                background_color: Color32::WHITE,
                color_mode: ColorMode::Monochrome,
                perimeter_points_radius: 5.0,
                perimeter_point_color: Color32::RED,
            },
        }
    }
    fn label(&self) -> &str {
        match *self {
            Preset::Rainbow => "Rainbow",
            Preset::Pencil => "Pencil",
            Preset::Educational => "Educational",
        }
    }
}

pub struct TimesCircleApp {
    first_frame: bool,
    center: Pos2,
    offset: Pos2,
    zoom: f32,
    rotation: f32,
    paused: bool,
    num_points: usize,
    multiplier: f32,
    step_size: f32,
    style: TimesCircleStyle,
    preset: Preset,
}

struct TimesCircleStyle {
    stroke: f32,
    color: Color32,
    background_color: Color32,
    color_mode: ColorMode,
    perimeter_points_radius: f32,
    perimeter_point_color: Color32,
}

impl eframe::App for TimesCircleApp {
    // Called whenever frame needs to be redrawn, maybe several times a second
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Calculate center of window (may change as window is resized)
        self.center = Pos2 {
            x: (ctx.available_rect().max.x - ctx.available_rect().min.x) / 2.0,
            y: (ctx.available_rect().max.y - ctx.available_rect().min.y) / 2.0,
        };

        // Draw ui
        self.ui(ctx);

        // Request a repaint (update gets called again immediately after this)
        // When animating circle
        if !self.paused && self.multiplier < self.num_points as f32 && self.multiplier > 0.0 {
            self.multiplier += self.step_size;
            ctx.request_repaint();
        }
    }
}

impl TimesCircleApp {
    // Initialize app state
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        TimesCircleApp {
            first_frame: true,
            center: Pos2 { x: 0.0, y: 0.0 },
            offset: Pos2 { x: 0.0, y: 0.0 },
            zoom: 0.85,
            rotation: std::f32::consts::PI,
            paused: false,
            num_points: 5000,
            multiplier: 2.0,
            step_size: 0.01,
            style: TimesCircleStyle {
                stroke: 0.02,
                color: Color32::BLACK,
                background_color: Color32::BLACK,
                color_mode: ColorMode::Length,
                perimeter_points_radius: 0.0,
                perimeter_point_color: Color32::BLACK,
            },
            preset: Preset::Rainbow,
        }
    }

    fn ui(&mut self, ctx: &egui::Context) {
        let frame = egui::Frame::default().fill(self.style.background_color);
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            // Handle mouse controls
            if ui.ui_contains_pointer() || self.first_frame {
                self.handle_mouse_inputs(ctx);

                // Use first frame to fix bug
                self.first_frame = false;
                ctx.request_repaint();
            }

            // Handle multitouch gestures for mobile
            if let Some(multi_touch) = ui.ctx().multi_touch() {
                self.handle_multitouch_inputs(multi_touch);
            }

            // Display options Ui
            egui::Window::new("Options")
                .collapsible(true)
                .auto_sized()
                .anchor(Align2::LEFT_TOP, [10.0, 10.0])
                .show(ctx, |ui| {
                    self.options_ui(ui);
                });

            // Paint times circle
            self.paint_times_circle(ui);
        });
    }

    fn options_ui(&mut self, ui: &mut Ui) {
        // p Mod n text
        let p_mod_n =
            RichText::new(format!("{:.2} Mod {}", self.multiplier, self.num_points).as_str())
                .font(FontId::proportional(20.0));
        ui.label(p_mod_n);

        self.control_options(ui);

        // TODO add spac here

        ui.horizontal(|ui| {
            ui.heading("Style");
        });

        self.style_options_ui(ui);
    }

    fn control_options(&mut self, ui: &mut Ui) {
        // Num points slider
        ui.add(egui::Slider::new(&mut self.num_points, 0..=10000).text("Points"));

        // Multiplier slider
        ui.add(
            egui::Slider::new(&mut self.multiplier, 0.0..=self.num_points as f32)
                .text("Multiplier")
                .min_decimals(2)
                .max_decimals(2),
        );

        // Step size slider
        ui.add(
            egui::Slider::new(&mut self.step_size, 0.0..=1.0)
                .text("Step Size")
                .min_decimals(1)
                .max_decimals(3),
        );

        // Playback buttons
        ui.horizontal(|ui| {
            if ui.button("▶").clicked() {
                self.paused = false;
            }
            if ui.button("■").clicked() {
                self.paused = true;
            }
        });
    }

    fn style_options_ui(&mut self, ui: &mut Ui) {
        // Presets
        ui.horizontal(|ui| {
            ui.label("Presets");
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", self.preset.label()))
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_value(&mut self.preset, Preset::Rainbow, "Rainbow")
                        .clicked()
                    {
                        self.preset = Preset::Rainbow;
                        self.style = self.preset.style();
                    };
                    if ui
                        .selectable_value(&mut self.preset, Preset::Pencil, "Pencil")
                        .clicked()
                    {
                        self.preset = Preset::Pencil;
                        self.style = self.preset.style();
                    };
                    if ui
                        .selectable_value(&mut self.preset, Preset::Educational, "Educational")
                        .clicked()
                    {
                        self.preset = Preset::Educational;
                        self.style = self.preset.style();
                        self.num_points = 10;
                        self.paused = true;
                        self.multiplier = 1.0;
                    };
                });
        });

        // Background color picker
        ui.horizontal(|ui| {
            ui.label("Background Color");
            ui.color_edit_button_srgba(&mut self.style.background_color);
        });

        // Color mode
        ui.horizontal(|ui| {
            ui.label("Color Mode");
            let (color_mode_text, next_mode) = match &self.style.color_mode {
                ColorMode::Monochrome => (self.style.color_mode.label(), ColorMode::Length),
                ColorMode::Length => (self.style.color_mode.label(), ColorMode::Radial),
                ColorMode::Radial => (self.style.color_mode.label(), ColorMode::Monochrome),
            };
            if ui
                .selectable_label(false, color_mode_text.to_string())
                .clicked()
            {
                self.style.color_mode = next_mode;
            };
        });

        // Line color picker
        ui.horizontal(|ui| {
            if matches!(self.style.color_mode, ColorMode::Monochrome) {
                ui.set_enabled(true);
            } else {
                ui.set_enabled(false);
            }
            ui.label("Line Color");
            ui.color_edit_button_srgba(&mut self.style.color);
        });

        // Line width slider
        ui.add(
            egui::Slider::new(&mut self.style.stroke, 0.0..=1.0)
                .text("Line Width")
                .max_decimals(2),
        );

        // Points radius slider
        ui.add(
            egui::Slider::new(&mut self.style.perimeter_points_radius, 0.0..=10.0)
                .text("Point Size")
                .min_decimals(2)
                .max_decimals(2),
        );

        // Point color picker
        ui.horizontal(|ui| {
            if self.style.perimeter_points_radius == 0.0 {
                ui.set_enabled(false);
            } else {
                ui.set_enabled(true);
            }
            ui.label("Point Color");
            ui.color_edit_button_srgba(&mut self.style.perimeter_point_color);
        });
    }

    fn paint_times_circle(&mut self, ui: &mut Ui) {
        // Calculate radius of circle from screen size
        let radius: f32 = if self.center.y < self.center.x {
            self.center.y * self.zoom
        } else {
            self.center.x * self.zoom
        };

        // Generate evenly spaced points around the circumference of a circle
        let points: Vec<Pos2> = TimesCircleApp::generate_points(self.num_points, self.rotation);

        // FIXME Fix artifacts
        for i in 0..(self.num_points) {
            // Find the point to connect to
            let j = ((i as f32) * self.multiplier) as usize % self.num_points;

            // Transform to world coords
            let p1 = Pos2 {
                x: points[i].x * radius + self.center.x + self.offset.x,
                y: points[i].y * radius + self.center.y + self.offset.y,
            };
            let p2 = Pos2 {
                x: points[j].x * radius + self.center.x + self.offset.x,
                y: points[j].y * radius + self.center.y + self.offset.y,
            };

            // TODO implement other color modes
            match self.style.color_mode {
                ColorMode::Monochrome => {
                    ui.painter()
                        .line_segment([p1, p2], Stroke::new(self.style.stroke, self.style.color));
                }
                ColorMode::Length => {
                    let line_length = TimesCircleApp::distance_between(points[i], points[j]);
                    let color = color::Hsva {
                        h: line_length / 2.0,
                        s: 1.0,
                        v: 1.0,
                        a: 1.0,
                    };
                    ui.painter()
                        .line_segment([p1, p2], Stroke::new(self.style.stroke, color));
                }
                ColorMode::Radial => ui
                    .painter()
                    .line_segment([p1, p2], Stroke::new(self.style.stroke, Color32::DARK_BLUE)),
            }
        }

        // Draw circle
        ui.painter().circle(
            Pos2 {
                x: self.center.x + self.offset.x,
                y: self.center.y + self.offset.y,
            },
            radius,
            Color32::TRANSPARENT,
            Stroke::new(self.style.stroke, self.style.color),
        );

        if self.style.perimeter_points_radius > 0.0 {
            self.draw_perimeter_points(radius, &points, ui);
        }
    }

    fn draw_perimeter_points(&mut self, radius: f32, points: &[Pos2], ui: &mut Ui) {
        // Draw points
        for point in points {
            let p = Pos2 {
                x: point.x * radius + self.center.x + self.offset.x,
                y: point.y * radius + self.center.y + self.offset.y,
            };
            ui.painter().circle(
                p,
                self.style.perimeter_points_radius,
                self.style.perimeter_point_color,
                Stroke::new(self.style.stroke, self.style.perimeter_point_color),
            );
        }
    }

    fn handle_mouse_inputs(&mut self, ctx: &egui::Context) {
        // Allow to drag circle around with mouse
        if ctx.input().pointer.button_down(PointerButton::Primary) {
            ctx.output().cursor_icon = CursorIcon::Grab;
            self.offset.x += ctx.input().pointer.delta().x;
            self.offset.y += ctx.input().pointer.delta().y;
        }

        // Zoom
        if let Some(pos) = ctx.pointer_hover_pos() {
            let factor = ctx.input().zoom_delta();
            self.zoom *= factor;

            // Change offset to zoom into mouse location
            self.offset.x += (self.center.x + self.offset.x - pos.x) * (factor - 1.0);
            self.offset.y += (self.center.y + self.offset.y - pos.y) * (factor - 1.0);
        }
    }

    fn handle_multitouch_inputs(&mut self, multi_touch: MultiTouchInfo) {
        self.zoom *= multi_touch.zoom_delta;
        self.rotation += multi_touch.rotation_delta;
        self.offset.x += multi_touch.translation_delta.x;
        self.offset.y += multi_touch.translation_delta.y;
    }

    // Generate evenly spaced points around a circle of radius 1, starting at given start_angle
    fn generate_points(num_points: usize, start_angle: f32) -> Vec<Pos2> {
        let n: f32 = num_points as f32;
        let mut points: Vec<Pos2> = Vec::with_capacity(num_points);
        let mut angle: f32 = start_angle;
        for _ in 0..num_points {
            let point = Pos2 {
                x: f32::cos(angle),
                y: f32::sin(angle),
            };
            angle += std::f32::consts::TAU / n;
            points.push(point);
        }
        points
    }

    fn distance_between(p1: Pos2, p2: Pos2) -> f32 {
        f32::sqrt((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2))
    }
}
