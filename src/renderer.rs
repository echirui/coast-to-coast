use eframe::egui::{self, Sense, Ui};
use crate::board::{self, Hex};
use crate::game::{Game, HEX_DRAW_SIZE};

use resvg::{self, usvg::Tree, usvg::Transform};
use usvg::Options;
use tiny_skia::Pixmap;
use fontdb::Database;

const BOARD_AREA_SIZE: f32 = 500.0;
const X_OFFSET_ADJUSTMENT: f32 = 150.0;

pub struct BoardRenderer {
    x_offset: f32,
    y_offset: f32,
    empty_texture: egui::TextureHandle,
    red_texture: egui::TextureHandle,
    blue_texture: egui::TextureHandle,
    last_board_hash: u64, // Add last_board_hash
}

impl BoardRenderer {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            x_offset: 0.0,
            y_offset: 0.0,
            empty_texture: Self::load_svg(ctx, include_bytes!("../assets/hexagon_empty.svg"), "hexagon_empty"),
            red_texture: Self::load_svg(ctx, include_bytes!("../assets/hexagon_red.svg"), "hexagon_red"),
            blue_texture: Self::load_svg(ctx, include_bytes!("../assets/hexagon_blue.svg"), "hexagon_blue"),
            last_board_hash: 0, // Initialize last_board_hash
        }
    }

    fn load_svg(ctx: &egui::Context, svg_bytes: &[u8], id: &str) -> egui::TextureHandle {
        let fontdb = Database::new();
        let rtree = Tree::from_data(svg_bytes, &Options::default(), &fontdb).expect("Failed to parse SVG");
        let pixmap_size = rtree.size().to_int_size();
        let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height())
            .expect("Failed to create pixmap");

        resvg::render(
            &rtree,
            Transform::default(),
            &mut pixmap.as_mut(),
        );

        let image_data = pixmap.data().to_vec();
        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            [pixmap.width() as usize, pixmap.height() as usize],
            &image_data,
        );

        ctx.load_texture(id, color_image, egui::TextureOptions::LINEAR)
    }

    pub fn calculate_offsets(&mut self, board: &board::Board) {
        let current_board_hash = board.calculate_hash();
        if self.last_board_hash == current_board_hash {
            return; // ボードの状態が変わっていなければ再計算しない
        }
        self.last_board_hash = current_board_hash;

        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        let size = HEX_DRAW_SIZE;

        for (hex, _state) in &board.cells {
            let (px, py) = self.hex_to_pixel(*hex, size);
            let (final_px, final_py) = self.transform_no_offset(px, py, size);
            min_x = min_x.min(final_px);
            max_x = max_x.max(final_px);
            min_y = min_y.min(final_py);
            max_y = max_y.max(final_py);
        }

        self.x_offset = (BOARD_AREA_SIZE - (max_x - min_x)) / 2.0 - min_x + X_OFFSET_ADJUSTMENT;
        self.y_offset = (BOARD_AREA_SIZE - (max_y - min_y)) / 2.0 - min_y;
    }

    pub fn render_board(&mut self, ui: &mut Ui, game: &mut Game) -> Option<Hex> {
        let (_rect, response) = ui.allocate_exact_size(egui::Vec2::new(BOARD_AREA_SIZE, BOARD_AREA_SIZE), Sense::click());
        let mut clicked_hex: Option<Hex> = None;

        for (hex, state) in &game.board.cells {
            let (px, py) = self.hex_to_pixel(*hex, HEX_DRAW_SIZE);
            let (final_px, final_py) = self.transform(px, py, HEX_DRAW_SIZE);
            let center = egui::pos2(final_px, final_py);

            let texture_to_draw = match state {
                board::CellState::Empty => &self.empty_texture,
                board::CellState::Red => &self.red_texture,
                board::CellState::Blue => &self.blue_texture,
            };

            let image_size = egui::Vec2::new(HEX_DRAW_SIZE * 2.0, HEX_DRAW_SIZE * 2.0); // Adjust size as needed
            let image_x = center.x - image_size.x / 2.0;
            let image_y = center.y - image_size.y / 2.0;
            let image_rect = egui::Rect::from_min_size(egui::pos2(image_x, image_y), image_size);

            ui.painter().image(
                texture_to_draw.id(),
                image_rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), // UV coordinates
                egui::Color32::WHITE,
            );
        }

        if response.clicked() {
            if let Some(pos) = response.hover_pos() {
                let (inv_px, inv_py) = self.inverse_transform(pos.x, pos.y, HEX_DRAW_SIZE);
                clicked_hex = Some(self.pixel_to_hex_no_offset(inv_px, inv_py, HEX_DRAW_SIZE));
            }
        }
        clicked_hex
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