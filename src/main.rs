use macroquad::prelude::*;

const CELL_SIZE: f32 = 10f32;
const CELL_SPEED: f32 = 100f32;

pub enum LifecyclePhase {
    Alive,
    Dead
}

pub enum Gender {
    Male,
    Female
}

pub struct Cell {
    rect: Rect,
    vel: Vec2,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32,
                screen_height() * 0.5f32,
                CELL_SIZE,
                CELL_SIZE,
            ),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }
    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * CELL_SPEED;
        self.rect.y += self.vel.y * dt * CELL_SPEED;
        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1f32;
        }
        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
        if self.rect.y > screen_height() - self.rect.w {
            self.vel.y = -1f32;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GREEN);
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
fn window_conf() -> Conf {
    Conf {
        window_title: "YeastSimulator".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[macroquad::main(window_conf())]
async fn main() {
    let mut cell = Cell::new();

    loop {
        clear_background(Color::new(0.,0.,0.1,1.0));
        draw_text(&format!("FPS: {}, Yeast cells: 1", get_fps()),
            screen_width()-300., screen_height()-5.,
            26.,
            LIGHTGRAY);
        cell.draw();
        cell.update(get_frame_time());
        next_frame().await
    }
}