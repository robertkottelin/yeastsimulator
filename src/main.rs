use macroquad::prelude::*;

const CELL_SIZE: f32 = 10f32;
const NUTRITION_SIZE: f32 = 200f32;
const CELL_SPEED: f32 = 100f32;
const STARTING_POPULATION: i32 = 2i32;
const POPULATION_LIMIT: i32 = 1000i32;


pub enum LifecyclePhase {
    Alive,
    Dead
}

pub enum Gender {
    Male,
    Female
}

#[derive(Debug)]
pub struct Cell {
    rect: Rect,
    vel: Vec2,
    on_nutrition_x: bool,
    on_nutrition_y: bool,
}

impl Cell {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(
                // screen_width() * 0.5f32,
                pos.x,
                pos.y,
                // screen_height() * 0.5f32,
                CELL_SIZE,
                CELL_SIZE,
            ),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
            on_nutrition_x: false,
            on_nutrition_y: false,
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

        if self.rect.x == screen_width() * 0.1f32 {
            self.on_nutrition_x = true;
        }
        if self.rect.y == screen_height() * 0.1f32 {
            self.on_nutrition_y = true;
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

pub struct Nutrition {
    rect: Rect,
}

impl Nutrition {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(
                // screen_width() * 0.1f32,
                // screen_height() * 0.1f32,
                pos.x,
                pos.y,
                NUTRITION_SIZE,
                NUTRITION_SIZE,
            )
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GRAY);
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
    
    let mut cells = Vec::<Cell>::new();
    let nutrition = Nutrition::new(vec2(screen_width() * 0.1f32 - NUTRITION_SIZE * 0.1f32, 
    screen_height() * 0.1f32));
    let mut i: i32 = 0;
    while i < STARTING_POPULATION {
        cells.push(Cell::new(vec2(screen_width() * 0.5f32 - CELL_SIZE * 0.5f32, 
        screen_height() * 0.6f32)));
        i = i + 1;
    }

    // let mut a: i32 = 2;
    loop {
        clear_background(Color::new(0.,0.,0.1,1.0));
        nutrition.draw();
        for cell in cells.iter_mut() {
            cell.update(get_frame_time());
            cell.draw();
        }
        draw_text(&format!("FPS: {}, Yeast cells: {}", get_fps(), cells.len()),
            screen_width()-300., screen_height()-5.,
            24.,
            LIGHTGRAY);
        // draw_text(&format!("On nutrition: {}"),
        //     screen_width()-300., screen_height()-30.,
        //     24.,
        //     LIGHTGRAY);
        // if nutrition_position = cell_position --> multiply
        // if cells.on_nutrition
        // cells.push(Cell::new(vec2(screen_width() * 0.5f32 - CELL_SIZE * 0.5f32, screen_height() * 0.6f32)));


        next_frame().await
    }
}