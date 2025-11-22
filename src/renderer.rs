use eframe::egui::{self, Context, Ui};
use crate::board::{Board, CellState, Hex};
use crate::game::{Game, HEX_DRAW_SIZE};
use std::f32::consts::PI;

const SQRT_3: f32 = 1.7320508; // Approximately sqrt(3)

pub struct BoardRenderer {
    hex_size: f32, // Corresponds to HEX_DRAW_SIZE
    x_offset: f32,
    y_offset: f32,
    hex_spacing: f32,
    pixels_per_hex_row: f32,
}

impl BoardRenderer {
    pub fn new(_cc: &Context) -> Self {
        Self {
            hex_size: HEX_DRAW_SIZE,
            x_offset: 0.0,
            y_offset: 0.0,
            hex_spacing: HEX_DRAW_SIZE * 1.5,
            pixels_per_hex_row: HEX_DRAW_SIZE * SQRT_3,
        }
    }

    pub fn calculate_offsets(&mut self, board: &Board) {
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;

        // Iterate through all possible hexes for the given board size
        for q in -board.size..=board.size {
            for r in (-board.size).max(-q - board.size)..=board.size.min(-q + board.size) {
                let hex = Hex { q, r };
                let pixel_pos = self.transform_no_offset(hex);
                min_x = min_x.min(pixel_pos.x);
                max_x = max_x.max(pixel_pos.x);
                min_y = min_y.min(pixel_pos.y);
                max_y = max_y.max(pixel_pos.y);
            }
        }

        let board_width = max_x - min_x + self.hex_size * SQRT_3; // Add hex width to boundaries
        let board_height = max_y - min_y + self.hex_size * 2.0; // Add hex height to boundaries

        // Assuming we want to center it within a default window size for now
        // These values should eventually come from the UI's available_size().
        // For now, hardcode to match main.rs default window size.
        let window_width = 800.0;
        let window_height = 600.0;

        self.x_offset = (window_width - board_width) / 2.0 - min_x;
        self.y_offset = (window_height - board_height) / 2.0 - min_y;
    }

    pub fn render_board(&mut self, ui: &mut Ui, game: &mut Game) -> Option<Hex> {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click());
        let mut clicked_hex: Option<Hex> = None;

        if response.clicked() {
            if let Some(mouse_pos) = ui.input(|i| i.pointer.latest_pos()) { // Correct way to get mouse position
                let hex = self.pixel_to_hex_no_offset(mouse_pos);
                // Check if the clicked hex is actually on the board and is a valid move
                if game.board.cells.contains_key(&hex) && game.board.is_valid_move(&hex) {
                    clicked_hex = Some(hex);
                }
            }
        }

        for (hex, cell_state) in game.board.cells.iter() {
            let center_pixel_pos = self.transform_no_offset(*hex);
            let center_pixel_pos_with_offset = self.transform(center_pixel_pos);

            // Define the vertices of a hexagon (pointy top)
            let mut points: Vec<egui::Pos2> = Vec::with_capacity(6);
            for i in 0..6 {
                let angle_rad = (PI / 3.0) * i as f32; // Angles for pointy top hex
                let x = center_pixel_pos_with_offset.x + self.hex_size * angle_rad.cos();
                let y = center_pixel_pos_with_offset.y + self.hex_size * angle_rad.sin(); // Corrected for y axis
                points.push(egui::pos2(x, y));
            }

            let color = match cell_state {
                CellState::Empty => egui::Color32::from_gray(50),
                CellState::Red => egui::Color32::RED,
                CellState::Blue => egui::Color32::BLUE,
            };

            painter.add(egui::Shape::convex_polygon(points, color, egui::Stroke::new(1.0, egui::Color32::GRAY)));
        }
        clicked_hex
    }

    // Converts axial hex coordinates to pixel coordinates without applying any global offset.
    fn transform_no_offset(&self, hex: Hex) -> egui::Pos2 {
        let x = self.hex_size * (SQRT_3 * hex.q as f32 + SQRT_3 / 2.0 * hex.r as f32);
        let y = self.hex_size * (3.0 / 2.0 * hex.r as f32);
        egui::Pos2::new(x, y)
    }

    // Applies global offset to pixel coordinates.
    fn transform(&self, pos: egui::Pos2) -> egui::Pos2 {
        egui::Pos2::new(pos.x + self.x_offset, pos.y + self.y_offset)
    }

    // Removes global offset from pixel coordinates.
    fn inverse_transform(&self, pixel_pos: egui::Pos2) -> egui::Pos2 {
        egui::Pos2::new(pixel_pos.x - self.x_offset, pixel_pos.y - self.y_offset)
    }

    // Converts pixel coordinates (without global offset) to floating-point axial hex coordinates.
    fn pixel_to_hex_float_no_offset(&self, pixel_pos_no_offset: egui::Pos2) -> (f32, f32) {
        let q_float = (pixel_pos_no_offset.x * SQRT_3 / 3.0 - pixel_pos_no_offset.y / 3.0) / self.hex_size;
        let r_float = (pixel_pos_no_offset.y * 2.0 / 3.0) / self.hex_size;
        (q_float, r_float)
    }

    // Rounds floating-point axial hex coordinates to the nearest integer axial hex coordinates.
    fn hex_round(&self, q_float: f32, r_float: f32) -> Hex {
        let mut q = q_float.round();
        let mut r = r_float.round();
        let s = (-q_float - r_float).round();

        let q_diff = (q - q_float).abs();
        let r_diff = (r - r_float).abs();
        let s_diff = (s - (-q_float - r_float)).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q = (-r - s).round();
        } else if r_diff > s_diff {
            r = (-q - s).round();
        }

        Hex { q: q as i32, r: r as i32 }
    }

    // Combines inverse_transform and pixel_to_hex_float_no_offset and hex_round.
    fn pixel_to_hex_no_offset(&self, pixel_pos: egui::Pos2) -> Hex {
        let no_offset_pixel = self.inverse_transform(pixel_pos);
        let (q_float, r_float) = self.pixel_to_hex_float_no_offset(no_offset_pixel);
        self.hex_round(q_float, r_float)
    }
}
