// source
// https://www.youtube.com/watch?v=79GyLlXAk-0&t=1016s

use bracket_lib::prelude::*;

enum  GameMode {
    Menu,
    Playing,
    End
}
struct State {
    mode: GameMode
}


impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu
        }
    }

    fn restart(&mut self)  {
        self.mode = GameMode::Playing
    }


    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "welcome to flappy dragon test");
        ctx.print_centered(8, "press p to play");
        ctx.print_centered(9, "press q to quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => ()
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "dead dead dead");
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "stubb sbuoinlkn  lkidl");

    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx)
        }
    }
}


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("flappy test")
        .build()?;

    main_loop(context, State::new())
}