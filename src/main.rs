use eframe::{self, egui};

const DEFAULT_WINDOW_WIDTH: f32 = 800.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 600.0;
const BOARD_AREA_SIZE: f32 = 500.0;
const X_OFFSET_ADJUSTMENT: f32 = 150.0;
const STROKE_THICKNESS: f32 = 1.0;
const HEX_ANGLE_INCREMENT: f32 = 60.0;

mod board;
mod game;
use game::HEX_DRAW_SIZE;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT]),
        ..Default::default()
    };
    eframe::run_native(
        "Hex Game",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    game: game::Game,
    x_offset: f32,
    y_offset: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            game: game::Game::new(),
            x_offset: 0.0, // Will be calculated dynamically
            y_offset: 0.0, // Will be calculated dynamically
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Calculate dynamic offsets for centering
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        let size = HEX_DRAW_SIZE;

        for (hex, _state) in &self.game.board.cells {
            let (px, py) = self.hex_to_pixel(*hex, size);
            let (final_px, final_py) = self.transform_no_offset(px, py, size);
            min_x = min_x.min(final_px);
            max_x = max_x.max(final_px);
            min_y = min_y.min(final_py);
            max_y = max_y.max(final_py);
        }

        self.x_offset = (BOARD_AREA_SIZE - (max_x - min_x)) / 2.0 - min_x + X_OFFSET_ADJUSTMENT;
        self.y_offset = (BOARD_AREA_SIZE - (max_y - min_y)) / 2.0 - min_y;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hex Game");

            match self.game.state {
                game::GameState::Finished { winner } => {
                    let winner_text = match winner {
                        board::CellState::Red => "Red",
                        board::CellState::Blue => "Blue",
                        _ => "Unknown",
                    };
                    ui.label(format!("Winner is: {}", winner_text));
                }
                game::GameState::InProgress => {
                    self.render_board(ui);
                }
                game::GameState::WaitingForPieRuleChoice => {
                    ui.label("Would you like to apply the pie rule?");
                    ui.horizontal(|ui| {
                        if ui.button("Apply Pie Rule").clicked() {
                            self.game.handle_pie_rule_decision(true);
                        }
                        if ui.button("Continue Normal Play").clicked() {
                            self.game.handle_pie_rule_decision(false);
                        }
                    });
                }
            }
        });
    }
}

impl MyApp {
    fn render_board(&mut self, ui: &mut egui::Ui) {
        let (_rect, response) = ui.allocate_exact_size(egui::Vec2::new(BOARD_AREA_SIZE, BOARD_AREA_SIZE), egui::Sense::click());
        let painter = ui.painter();
        let size = HEX_DRAW_SIZE;

        for (hex, state) in &self.game.board.cells {
            let (px, py) = self.hex_to_pixel(*hex, size);
            let (final_px, final_py) = self.transform(px, py, size);
            let center = egui::pos2(final_px, final_py);

            let color = match state {
                board::CellState::Empty => egui::Color32::from_gray(128),
                board::CellState::Red => egui::Color32::RED,
                board::CellState::Blue => egui::Color32::BLUE,
            };

            let points: Vec<egui::Pos2> = (0..6)
                .map(|i| {
                    let angle = (HEX_ANGLE_INCREMENT * i as f32).to_radians();
                    let x = center.x + size * angle.cos();
                    let y = center.y + size * angle.sin();
                    egui::pos2(x, y)
                })
                .collect();
            
            painter.add(egui::Shape::convex_polygon(points, color, egui::Stroke::new(STROKE_THICKNESS, egui::Color32::BLACK)));
        }

        if response.clicked() {
            if let Some(pos) = response.hover_pos() {
                let (inv_px, inv_py) = self.inverse_transform(pos.x, pos.y, size);
                let clicked_hex = self.pixel_to_hex_no_offset(inv_px, inv_py, size);
                self.game.handle_click(clicked_hex);
            }
        }
    }

    fn transform_no_offset(&self, px: f32, py: f32, size: f32) -> (f32, f32) {
        let pivot_hex = board::Hex { q: 5, r: 5 };
        let (pivot_px, pivot_py) = self.hex_to_pixel(pivot_hex, size);
        let angle_rad = -60.0f32.to_radians();
        let cos_angle = angle_rad.cos();
        let sin_angle = angle_rad.sin();
        
        let rel_px = px - pivot_px;
        let rel_py = py - pivot_py;

        let rotated_px = rel_px * cos_angle - rel_py * sin_angle;
        let rotated_py = rel_px * sin_angle + rel_py * cos_angle;

        (rotated_px + pivot_px, rotated_py + pivot_py)
    }

    fn transform(&self, px: f32, py: f32, size: f32) -> (f32, f32) {
        let (transformed_px, transformed_py) = self.transform_no_offset(px, py, size);
        (transformed_px + self.x_offset, transformed_py + self.y_offset)
    }

    fn inverse_transform(&self, px: f32, py: f32, size: f32) -> (f32, f32) {
        let pivot_hex = board::Hex { q: 5, r: 5 };
        let (pivot_px, pivot_py) = self.hex_to_pixel(pivot_hex, size);
        let angle_rad = 60.0f32.to_radians();
        let cos_angle = angle_rad.cos();
        let sin_angle = angle_rad.sin();

        let rel_px = (px - self.x_offset) - pivot_px;
        let rel_py = (py - self.y_offset) - pivot_py;

        let rotated_px = rel_px * cos_angle - rel_py * sin_angle;
        let rotated_py = rel_px * sin_angle + rel_py * cos_angle;

        (rotated_px + pivot_px, rotated_py + pivot_py)
    }

    fn hex_to_pixel(&self, hex: board::Hex, size: f32) -> (f32, f32) {
        let x = size * (3.0 / 2.0 * hex.q as f32);
        let y = size * (f32::sqrt(3.0) / 2.0 * hex.q as f32 + f32::sqrt(3.0) * hex.r as f32);
        (x, y)
    }

    fn pixel_to_hex_no_offset(&self, px: f32, py: f32, size: f32) -> board::Hex {
        let q = (2.0 / 3.0 * px) / size;
        let r = (-1.0 / 3.0 * px + f32::sqrt(3.0) / 3.0 * py) / size;
        self.hex_round(q, r)
    }

    fn hex_round(&self, q: f32, r: f32) -> board::Hex {
        let s = -q - r;
        let mut rq = q.round();
        let mut rr = r.round();
        let rs = s.round();

        let q_diff = (rq - q).abs();
        let r_diff = (rr - r).abs();
        let s_diff = (rs - s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            rq = -rr - rs;
        } else if r_diff > s_diff {
            rr = -rq - rs;
        }

        board::Hex { q: rq as i32, r: rr as i32 }
    }
}