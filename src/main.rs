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
    /// Returns the left edge of the `Rect`
    pub fn left(&self) -> f32 {
        self.rect.x
    }

    /// Returns the right edge of the `Rect`
    pub fn right(&self) -> f32 {
        self.rect.x + self.rect.w
    }

    /// Returns the top edge of the `Rect`
    pub fn top(&self) -> f32 {
        self.rect.y
    }

    /// Returns the bottom edge of the `Rect`
    pub fn bottom(&self) -> f32 {
        self.rect.y + self.rect.h
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
    /// Returns the left edge of the `Rect`
    pub fn left(&self) -> f32 {
        self.rect.x
    }

    /// Returns the right edge of the `Rect`
    pub fn right(&self) -> f32 {
        self.rect.x + self.rect.w
    }

    /// Returns the top edge of the `Rect`
    pub fn top(&self) -> f32 {
        self.rect.y
    }

    /// Returns the bottom edge of the `Rect`
    pub fn bottom(&self) -> f32 {
        self.rect.y + self.rect.h
    }
     /// Checks whether the `Rect` overlaps another `Rect`
     pub fn overlaps(&self, other: &Cell) -> bool {
        self.rect.left() <= other.rect.right()
            && self.rect.right() >= other.rect.left()
            && self.rect.top() <= other.rect.bottom()
            && self.rect.bottom() >= other.rect.top()
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

#[macroquad::main("YeastSimulator")]
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


        
        let overlapping = nutrition.overlaps(&cells[0]);
        let overlapping2 = nutrition.overlaps(&cells[1]);

        if overlapping == true {
            cells[0].on_nutrition_x = true;
            cells.push(Cell::new(vec2(cells[0].left(), cells[0].top())));
        }
        if overlapping2 == true {
            cells[1].on_nutrition_y = true;
            cells.push(Cell::new(vec2(cells[1].left(), cells[1].top())));

        }

        draw_text(&format!("On nutrition: {:?}, {:?}", overlapping, overlapping2),
        screen_width()*0.5f32, screen_height()-30.,
        30.,
        LIGHTGRAY);
        draw_text(&format!("FPS: {}", get_fps()),
            screen_width()*0.5f32, screen_height()-5.,
            30.,
            LIGHTGRAY);
        draw_text(&format!("Yeast cells: {}", cells.len()),
            screen_width()*0.5f32, screen_height()-50.,
            30.,
        LIGHTGRAY);

        // println!("{:?}", overlapping);


        next_frame().await
    }
}