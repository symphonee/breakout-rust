use ggez::event::{KeyCode, KeyMods};
use ggez::*;

pub mod input;
use input::*;

pub struct State {
    dt: std::time::Duration,
    pub ball_position: nalgebra::Point2<f32>,
    pub ball_velocity: nalgebra::Vector2<f32>,
    pub player_positon: nalgebra:: Point2<f32>,
    pub input_state: InputState,
}

impl State {
    pub fn new(
        ball_position: nalgebra::Point2<f32>,
        ball_velocity: nalgebra::Vector2<f32>,
        player_positon: nalgebra::Point2<f32>,
    ) -> State {
        State {
            dt: std::time::Duration::new(0, 0),
            ball_position,
            ball_velocity,
            player_positon,
            input_state: InputState::default(),
        }
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);

        let mut delta_move = nalgebra::Vector2::<f32>::zeros();
        
        if self.input_state.is_key_pressed(Key::A) || self.input_state.is_key_pressed(Key::Left){
            delta_move.x += -1.0;
        }

        if self.input_state.is_key_pressed(Key::D) || self.input_state.is_key_pressed(Key::Right) {
            delta_move.x += 1.0
        }

        let speed = 100.0;
        self.player_positon += delta_move * self.dt.as_secs_f32() * speed;
        
        self.ball_position = nalgebra::Point2::new(
            self.ball_position.x + self.ball_velocity.x * self.dt.as_secs_f32(),
            self.ball_position.y + self.ball_velocity.y * self.dt.as_secs_f32(),
        );
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        draw_text(ctx, "Hej Björn!", 200.0, 200.0)?;
        draw_text(ctx, "Let's do this!", 200.0, 250.0)?;
        draw_text(ctx, "-------------------", 200.0, 300.0)?;
        draw_text(ctx, "Hell yes, let's go! :D", 200.0, 350.0)?;
        draw_text(ctx, "This is so exciting!!", 200.0, 400.0)?;
        draw_ball(ctx, &self.ball_position)?;
        draw_player(ctx, &self.player_positon)?;
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        if !repeat {
            self.input_state.set_key_pressed(keycode, true);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.input_state.set_key_pressed(keycode, false);
    }
}

fn draw_text(ctx: &mut Context, message: &str, x: f32, y: f32) -> GameResult<()> {
    let font = graphics::Font::default();
    let mut text = graphics::Text::new(message);
    text.set_font(font, graphics::Scale::uniform(40.0));
    text.set_bounds(nalgebra::Point2::new(400.0, 100.0), graphics::Align::Center);
    graphics::draw(
        ctx,
        &text,
        graphics::DrawParam::default()
            .dest(nalgebra::Point2::new(x, y))
            .color(graphics::WHITE),
    )?;
    Ok(())
}

fn draw_ball(ctx: &mut Context, ball_position: &nalgebra::Point2<f32>) -> GameResult<()> {
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        *ball_position,
        10.0,
        2.0,
        graphics::WHITE,
    )
    .unwrap();
    graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
    Ok(())
}

fn draw_player(ctx: &mut Context, player_positon: &nalgebra::Point2<f32>) -> GameResult<()> {
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        *player_positon,
        30.0,
        2.0,
        graphics::WHITE,
    )
    .unwrap();
    graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
    Ok(())
}
