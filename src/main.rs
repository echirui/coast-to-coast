use eframe::{self, egui};

const DEFAULT_WINDOW_WIDTH: f32 = 800.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 600.0;

mod board;
mod game;
mod renderer;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT]),
        ..Default::default()
    };
    eframe::run_native(
        "Hex Game",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp {
    game: game::Game,
    board_renderer: renderer::BoardRenderer,
}

impl Default for MyApp {
    fn default() -> Self {
        panic!("MyApp::new(cc: &eframe::CreationContext<'_>) must be called");
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            game: game::Game::new(),
            board_renderer: renderer::BoardRenderer::new(&cc.egui_ctx),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.board_renderer.calculate_offsets(&self.game.board);

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
                    if let Some(clicked_hex) = self.board_renderer.render_board(ui, &mut self.game) {
                        self.game.handle_click(clicked_hex);
                    }
                }
            }
        });
    }
}