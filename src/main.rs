use breakout_rust::State;
use ggez::*;

fn main() {
    let state = &mut State {};
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("breakout", "bjandra")
        .conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
