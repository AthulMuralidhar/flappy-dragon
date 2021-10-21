// source
// https://www.youtube.com/watch?v=79GyLlXAk-0&t=1016s
// https://github.com/byoboo/flappy-dragon/blob/master/src/main.rs

use bracket_lib::prelude::*;

enum  GameMode {
    Menu,
    Playing,
    End
}
struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: Player::new(5,25),
            frame_time: 0.0
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
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > 30.0 {
            self.frame_time = 0.;
            self.player.gravity_and_move();
        }


        if Some(VirtualKeyCode::Space) == ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print(0,0, "press space to flap");

        if self.player.y > 50 {
            self.mode = GameMode::End;
        }


    }
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            0,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@')
        )
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            // adding velocity here means u fall down
            self.velocity += 0.2;
        }

        self.y += self.velocity as i32;
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        //  substractiong velocity means u move up
        self.velocity = -2.0
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