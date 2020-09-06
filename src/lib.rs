use ggez::*;

pub struct State {}

impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        draw_text(ctx, "Hej Björn!", 200.0, 200.0)?;
        draw_text(ctx, "Let's do this!", 200.0, 250.0)?;
        draw_text(ctx, "-------------------", 200.0, 300.0)?;
        draw_text(ctx, "Hell yes, let's go! :D", 200.0, 350.0)?;
        draw_text(ctx, "This is so exciting!!", 200.0, 400.0)?;
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
