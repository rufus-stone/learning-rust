use ggez::event::EventHandler;
use ggez::graphics::{Color, Mesh, Rect};
use ggez::{graphics, Context, GameError, GameResult};
use rand::RngCore;

use crate::entities::food::Food;
use crate::entities::grid::Grid;
use crate::entities::snek::Snek;
use crate::players::Move;
use crate::settings::SCREEN_HEIGHT;
use crate::types::Vec2;

use super::state::GameState;

impl<R, M> EventHandler<GameError> for GameState<R, M>
where
    R: RngCore,
    M: Move,
{
    /// Called every frame
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 5;

        // Check for new input without waiting
        let players_move = self.player.make_move(ctx);

        // Check to see if the player has made a new move, otherwise continue in the current direction
        if let Some(new_orientation) = players_move {
            self.snek.set_orientation(new_orientation);
        }

        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            self.step();

            // Finally, check if the the game has ended, and quit if so
            if !self.play {
                log::warn!("{}", self);
                ggez::event::quit(ctx);
            }
        }

        Ok(())
    }

    /// Draw the game screen
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Clear the screen to black
        graphics::clear(ctx, Color::from_rgba(0, 0, 0, 255));

        // Draw the Grid outline
        draw_grid(ctx, &self.grid);

        // Draw the Food
        draw_food(ctx, &self.food, &self.grid);

        // Draw the Snek
        draw_snek(ctx, &self.snek, &self.grid);

        // Draw the stats
        draw_stats(ctx, self.snek.len());

        // Update the screen
        graphics::present(ctx).expect("Error presenting graphics!");

        Ok(())
    }
}

/// How many pixels make up each Grid square?
fn pixels_per_grid_square(ctx: &mut Context, grid: &Grid) -> (f32, f32) {
    let (screen_width, screen_height) = graphics::drawable_size(ctx);

    (
        screen_width / grid.width() as f32,
        screen_height / grid.height() as f32,
    )
}

/// Convert grid positions into pixel positions
fn grid_position_to_pixels(ctx: &mut Context, pos: &Vec2, grid: &Grid) -> (f32, f32) {
    let (px, py) = pixels_per_grid_square(ctx, grid);

    (px * pos.x as f32, py * pos.y as f32)
}

/// Render the Grid outline
fn draw_grid(ctx: &mut Context, grid: &Grid) {
    // How big should each grid square appear
    let (px, py) = pixels_per_grid_square(ctx, grid);

    // Generate a vector of meshes
    let meshes: Vec<Mesh> = grid
        .into_iter()
        .map(|square| {
            let (x, y) = grid_position_to_pixels(ctx, &square, grid);
            let rect = Rect::new(x, y, px, py);
            Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(2.0),
                rect,
                Color::from_rgba(255, 255, 255, 255),
            )
            .expect("Error creating one of the meshes for the Grid!")
        })
        .collect();

    // Draw the meshes
    for mesh in meshes {
        graphics::draw(ctx, &mesh, graphics::DrawParam::default()).expect("Error drawing mesh!");
    }
}

/// Render the Food
fn draw_food(ctx: &mut Context, food: &Food, grid: &Grid) {
    // How big should the Food appear
    let (w, h) = pixels_per_grid_square(ctx, grid);

    // Where is the Food currently on the Grid?
    let pos = food.pos();

    // What is this in pixel-space?
    let (x, y) = grid_position_to_pixels(ctx, pos, grid);

    // Create the food mesh
    let food_rect = Rect::new(x, y, w, h);
    let food_mesh = Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        food_rect,
        Color::from_rgba(100, 255, 100, 255),
    )
    .expect("Error creating food_mesh!");

    graphics::draw(ctx, &food_mesh, graphics::DrawParam::default())
        .expect("Error drawing food_mesh!");
}

/// Render the Snek
fn draw_snek(ctx: &mut Context, snek: &Snek, grid: &Grid) {
    // How big should the each Snek part appear
    let (w, h) = pixels_per_grid_square(ctx, grid);

    // Generate a vector of meshes
    let mut meshes: Vec<Mesh> = snek
        .parts()
        .iter()
        .map(|square| {
            let (x, y) = grid_position_to_pixels(ctx, &square, grid);
            let rect = Rect::new(x, y, w, h);
            Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                rect,
                Color::from_rgba(255, 50, 50, 255),
            )
            .expect("Error creating one of the meshes for the Snek!")
        })
        .collect();

    // Change the colour of the head
    let head = snek.head();
    let (x, y) = grid_position_to_pixels(ctx, head, grid);
    let head_rect = Rect::new(x, y, w, h);
    let head_mesh = Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        head_rect,
        Color::from_rgba(50, 50, 200, 255),
    )
    .expect("Error creating one of the meshes for the Snek!");
    *meshes.last_mut().unwrap() = head_mesh;

    // Draw the meshes
    for mesh in meshes {
        graphics::draw(ctx, &mesh, graphics::DrawParam::default()).expect("Error drawing mesh!");
    }

    println!("Snek: {:?}", snek);
}

/// Render the current frames per second, elapsed time, and score
fn draw_stats(ctx: &mut Context, score: usize) {
    // Show the stats
    let fps = format!("[fps: {}]", ggez::timer::fps(ctx) as i64);
    let time = format!(
        "[t: {:.1}]",
        ggez::timer::duration_to_f64(ggez::timer::time_since_start(ctx))
    );
    let score_text = format!("[score: {}]", score);
    let debug_text = graphics::Text::new(format!("{}{}{}", fps, time, score_text));

    let params = graphics::DrawParam::default()
        .dest([20.0, SCREEN_HEIGHT - 20.0 - debug_text.height(ctx) as f32]);

    graphics::draw(ctx, &debug_text, params).expect("Error drawing debug text!");
}
