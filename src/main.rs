use breakout_rust::State;
use ggez::*;

fn main() {
    let state = &mut State::new(
        nalgebra::Point2::new(200.0, 200.0),
        nalgebra::Vector2::new(0.0, -30.0),
    );
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("breakout", "bjandra")
        .conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
