use eframe::egui::{self, Context, Ui};
use crate::board::{Board, CellState, Hex};
use crate::game::{Game, HEX_DRAW_SIZE};

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

        let board_width = max_x - min_x + self.hex_size * SQRT_3;
        let board_height = max_y - min_y + self.hex_size * 2.0;

        let window_width = 800.0;
        let window_height = 600.0;

        self.x_offset = (window_width - board_width) / 2.0 - min_x;
        self.y_offset = (window_height - board_height) / 2.0 - min_y;
    }

    pub fn render_board(&mut self, ui: &mut Ui, game: &Game) -> Option<Hex> {
        let (response, _painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click());
        let mut clicked_hex: Option<Hex> = None;

        if response.clicked() {
            if let Some(mouse_pos) = ui.input(|i| i.pointer.latest_pos()) {
                let hex = self.pixel_to_hex_no_offset(mouse_pos);
                if game.board.cells.contains_key(&hex) {
                    clicked_hex = Some(hex);
                }
            }
        }

        for (hex, cell_state) in game.board.cells.iter() {
            let center_pixel_pos = self.transform_no_offset(*hex);
            let center_pixel_pos_with_offset = self.transform(center_pixel_pos);

            let image = match cell_state {
                CellState::Empty => egui::Image::new(egui::include_image!("../assets/hexagon_empty.svg")),
                CellState::Red => egui::Image::new(egui::include_image!("../assets/hexagon_red.svg")),
                CellState::Blue => egui::Image::new(egui::include_image!("../assets/hexagon_blue.svg")),
            };

            let image_size = egui::Vec2::splat(self.hex_size * 2.0); // Adjust size as needed
            let image_rect = egui::Rect::from_center_size(center_pixel_pos_with_offset, image_size);

            ui.put(image_rect, image.fit_to_exact_size(image_size));
        }
        clicked_hex
    }

    fn transform_no_offset(&self, hex: Hex) -> egui::Pos2 {
        let x = self.hex_size * (SQRT_3 * hex.q as f32 + SQRT_3 / 2.0 * hex.r as f32);
        let y = self.hex_size * (3.0 / 2.0 * hex.r as f32);
        egui::pos2(x, y)
    }

    fn transform(&self, pos: egui::Pos2) -> egui::Pos2 {
        egui::Pos2::new(pos.x + self.x_offset, pos.y + self.y_offset)
    }

    fn inverse_transform(&self, pixel_pos: egui::Pos2) -> egui::Pos2 {
        egui::Pos2::new(pixel_pos.x - self.x_offset, pixel_pos.y - self.y_offset)
    }

    fn pixel_to_hex_no_offset(&self, pixel_pos: egui::Pos2) -> Hex {
        let no_offset_pixel = self.inverse_transform(pixel_pos);
        let q_float = (no_offset_pixel.x * SQRT_3 / 3.0 - no_offset_pixel.y / 3.0) / self.hex_size;
        let r_float = (no_offset_pixel.y * 2.0 / 3.0) / self.hex_size;
        self.hex_round(q_float, r_float)
    }

    fn hex_round(&self, q_float: f32, r_float: f32) -> Hex {
        let s_float = -q_float - r_float;
        let mut q = q_float.round();
        let mut r = r_float.round();
        let s = s_float.round();

        let q_diff = (q - q_float).abs();
        let r_diff = (r - r_float).abs();
        let s_diff = (s - s_float).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q -s;
        }

        Hex { q: q as i32, r: r as i32 }
    }
}
