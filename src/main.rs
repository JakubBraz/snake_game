use std::collections::{VecDeque};
use macroquad::input::KeyCode::{Escape, Q};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;
use macroquad::rand::rand;

#[derive(Debug)]
struct PlayerState {
    x: i32,
    y: i32,
    direction: u8,
    next_move: u8,
    body: VecDeque<(i32, i32)>,
    fruit: (i32, i32),
    killed: bool,
    score: u32,
    next_update: f32,
}

struct GameState {
    frames: i32,
    time: f32,
    total_time: f32,
    state: PlayerState,
}

struct GameProp {
    width: u32,
    height: u32,
    update_every: f32,
}

static BOARD: GameProp = GameProp {
    width: 15,
    height: 15,
    update_every: 0.075,
};

static DRAW_OFFSET: f32 = 20.0;
static DRAW_FIELD_SIZE: f32 = 20.0;

fn draw_game(state: &PlayerState) {
    clear_background(Color::from_rgba(50, 255, 150, 255));
    draw_rectangle(DRAW_OFFSET, DRAW_OFFSET, (BOARD.width as f32) * DRAW_FIELD_SIZE, (BOARD.height as f32) * DRAW_FIELD_SIZE, BLACK);
    for (x, y) in &state.body {
        let draw_x = DRAW_OFFSET + (*x as f32) * DRAW_FIELD_SIZE;
        let draw_y = DRAW_OFFSET + (*y as f32) * DRAW_FIELD_SIZE;
        draw_rectangle(draw_x, draw_y, DRAW_FIELD_SIZE, DRAW_FIELD_SIZE, SKYBLUE);
    }
    let draw_x = DRAW_OFFSET + (state.x as f32) * DRAW_FIELD_SIZE;
    let draw_y = DRAW_OFFSET + (state.y as f32) * DRAW_FIELD_SIZE;
    draw_rectangle(draw_x, draw_y, DRAW_FIELD_SIZE, DRAW_FIELD_SIZE, BLUE);
    let draw_x = DRAW_OFFSET * 1.5 + (state.fruit.0 as f32) * DRAW_FIELD_SIZE;
    let draw_y = DRAW_OFFSET * 1.5 + (state.fruit.1 as f32) * DRAW_FIELD_SIZE;
    draw_circle(draw_x, draw_y, DRAW_FIELD_SIZE / 2.0, YELLOW);
    let score = state.score;
    draw_text(format!("Score: {score}").as_str(), 20.0, 15.0, 20.0, DARKGRAY);
}

fn game_over(x: i32, y: i32, state: &PlayerState) -> bool {
    x < 0 || x >= BOARD.width as i32 || y < 0 || y >= BOARD.height as i32 || state.body.contains(&(x, y))
}

fn update_state(state: &mut PlayerState) {
    state.direction = match state.next_move {
        1 => if state.direction != 2 { 1 } else { state.direction },
        2 => if state.direction != 1 { 2 } else { state.direction },
        3 => if state.direction != 4 { 3 } else { state.direction },
        _ => if state.direction != 3 { 4 } else { state.direction },
    };

    let mut new_pos = match state.direction {
        1 => (state.x - 1, state.y),
        2 => (state.x + 1, state.y),
        3 => (state.x, state.y - 1),
        _ => (state.x, state.y + 1),
    };
    new_pos.0 = if new_pos.0 < 0 { BOARD.width as i32 - 1 } else { new_pos.0 };
    new_pos.0 = if new_pos.0 >= BOARD.width as i32 { 0 } else { new_pos.0 };
    new_pos.1 = if new_pos.1 < 0 { BOARD.height as i32 - 1 } else { new_pos.1 };
    new_pos.1 = if new_pos.1 >= BOARD.height as i32 { 0 } else { new_pos.1 };

    let eaten = new_pos == state.fruit;

    state.killed = game_over(new_pos.0, new_pos.1, state);

    if !state.killed {
        state.body.push_back((state.x, state.y));
        if !eaten {
            state.body.pop_front();
        } else {
            state.score += 1;
            state.fruit = new_fruit(new_pos, state.body.make_contiguous());
        }

        state.x = new_pos.0;
        state.y = new_pos.1;
    }
}

fn new_fruit(head: (i32, i32), body: &[(i32, i32)]) -> (i32, i32) {
    // println!(" ** BODY: {:?}", body);
    loop {
        let (new_x, new_y) = ((rand() % BOARD.width) as i32, (rand() % BOARD.height) as i32);
        if (new_x, new_y) != head && !body.contains(&(new_x, new_y)) {
            return (new_x, new_y);
        }
    }
}

fn reset_player_state() -> PlayerState {
    PlayerState {
        x: 3,
        y: 1,
        direction: 2,
        next_move: 2,
        body: VecDeque::from([(0, 1), (1, 1), (2, 1)]),
        fruit: new_fruit((3, 1), &vec![(0, 1), (1, 1), (2, 1)]),
        killed: false,
        next_update: BOARD.update_every,
        score: 0,
    }
}

fn reset_game() -> GameState {
    GameState {
        frames: 0,
        time: 0.0,
        total_time: 0.0,
        state: reset_player_state(),
    }
}

#[macroquad::main("Snake")]
async fn main() {
    rand::srand(now() as u64);

    let mut game_state = reset_game();

    loop {
        game_state.total_time += get_frame_time();

        if is_key_pressed(Escape) {
            game_state = reset_game();
        }
        if is_key_pressed(Q) {
            break;
        }

        game_state.state.next_move = if is_key_down(KeyCode::Left) {
            1
        } else if is_key_down(KeyCode::Right) {
            2
        } else if is_key_down(KeyCode::Up) {
            3
        } else if is_key_down(KeyCode::Down) {
            4
        } else {
            game_state.state.next_move
        };

        if !game_state.state.killed {
            if game_state.total_time >= game_state.state.next_update {
                game_state.state.next_update = game_state.total_time + BOARD.update_every - (game_state.total_time - game_state.state.next_update);
                update_state(&mut game_state.state);
            }
        }

        game_state.frames += 1;

        if game_state.total_time - game_state.time > 1.0 {
            println!("Frames: {}", game_state.frames);
            println!("Total time: {}", game_state.total_time);
            println!("Total time: {}", get_time());
            println!("{} {}", game_state.state.x, game_state.state.y);
            game_state.time = game_state.total_time;
            game_state.frames = 0;
        }

        draw_game(&game_state.state);

        next_frame().await
    }
}
