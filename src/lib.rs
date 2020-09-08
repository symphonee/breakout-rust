use ggez::*;

pub struct State {
    dt: std::time::Duration,
    pub ball_position: nalgebra::Point2<f32>,
    pub ball_velocity: nalgebra::Vector2<f32>,
}

impl State {
    pub fn new(
        ball_position: nalgebra::Point2<f32>,
        ball_velocity: nalgebra::Vector2<f32>,
    ) -> State {
        State {
            dt: std::time::Duration::new(0, 0),
            ball_position,
            ball_velocity,
        }
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
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
        graphics::present(ctx)
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
