// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

use crate::Coord;
use log::info;
use serde_json::{Value, json};

use crate::{Battlesnake, Board, Game};

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    json!({
        "apiversion": "1",
        "author": "", // TODO: Your Battlesnake Username
        "color": "#21c473", // TODO: Choose color
        "head": "default", // TODO: Choose head
        "tail": "default", // TODO: Choose tail
    })
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

trait Mv {
    fn view(&self) -> MvView;
    fn coord(&self) -> &Coord;
}

struct Up(Coord);
impl Mv for Up {
    fn view(&self) -> MvView {
        MvView::Up
    }
    fn coord(&self) -> &Coord {
        &self.0
    }
}

struct Down(Coord);
impl Mv for Down {
    fn view(&self) -> MvView {
        MvView::Down
    }
    fn coord(&self) -> &Coord {
        &self.0
    }
}

struct Left(Coord);
impl Mv for Left {
    fn view(&self) -> MvView {
        MvView::Left
    }
    fn coord(&self) -> &Coord {
        &self.0
    }
}

struct Right(Coord);
impl Mv for Right {
    fn view(&self) -> MvView {
        MvView::Right
    }
    fn coord(&self) -> &Coord {
        &self.0
    }
}

#[derive(Debug, Clone)]
enum MvView {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Display for MvView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Left => "left",
            Self::Right => "right",
        };
        write!(f, "{s}")
    }
}

struct Moves {
    up: Option<Up>,
    down: Option<Down>,
    left: Option<Left>,
    right: Option<Right>,
}

impl Moves {
    fn new(head: &Coord) -> Self {
        Self {
            up: Some(Up(head.up())),
            down: Some(Down(head.down())),
            left: Some(Left(head.left())),
            right: Some(Right(head.right())),
        }
    }

    fn has_safe_move(&self) -> bool {
        self.up.is_some() || self.down.is_some() || self.left.is_some() || self.right.is_some()
    }

    fn get_all_safe_moves(self) -> Vec<MvView> {
        vec![
            self.up.map(|m| m.view()),
            self.down.map(|m| m.view()),
            self.left.map(|m| m.view()),
            self.right.map(|m| m.view()),
        ]
        .into_iter()
        .filter_map(|v| v)
        .collect::<Vec<MvView>>()
    }

    fn elide_collisions(mut self, coord: &Coord) -> Self {
        if self.down.as_ref().is_some_and(|m| m.coord() == coord) {
            self.down = None;
            return self;
        }
        if self.up.as_ref().is_some_and(|m| m.coord() == coord) {
            self.up = None;
            return self;
        }
        if self.left.as_ref().is_some_and(|m| m.coord() == coord) {
            self.left = None;
            return self;
        }
        if self.right.as_ref().is_some_and(|m| m.coord() == coord) {
            self.right = None;
            return self;
        }
        self
    }

    fn avoid_snake(self, snake: &[Coord]) -> Self {
        snake
            .iter()
            .fold(self, |acc, segment| acc.elide_collisions(segment))
    }

    fn avoid_snakes(self, snakes: &[Battlesnake]) -> Self {
        snakes
            .iter()
            .fold(self, |acc, opp| acc.avoid_snake(&opp.body))
    }

    fn dont_go_backwards(mut self, my_head: &Coord, my_neck: &Coord) -> Self {
        if my_neck.x < my_head.x {
            // Neck is left of head, don't move left
            self.left = None;
        } else if my_neck.x > my_head.x {
            // Neck is right of head, don't move right
            self.right = None;
        } else if my_neck.y < my_head.y {
            // Neck is below head, don't move down
            self.down = None;
        } else if my_neck.y > my_head.y {
            // Neck is above head, don't move up
            self.up = None;
        }
        self
    }

    fn stay_in_bounds(mut self, my_head: &Coord, board_width: &i32, board_height: &i32) -> Self {
        if my_head.x >= *board_width - 1 {
            self.right = None;
        } else if my_head.x <= 0 {
            self.left = None;
        }

        if my_head.y >= *board_height - 1 {
            self.up = None;
        } else if my_head.y <= 0 {
            self.down = None;
        }
        self
    }
    // TODO: navigate towards food
    fn find_food(self, _food: &[Coord]) -> Self {
        if !self.has_safe_move() {
            return self;
        }
        self
    }
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Value {
    // We've included code to prevent your Battlesnake from moving backwards
    let my_head = &you.body[0]; // Coordinates of your head
    let my_neck = &you.body[1]; // Coordinates of your "neck"

    let chosen = Moves::new(my_head)
        .dont_go_backwards(my_head, my_neck)
        .stay_in_bounds(my_head, &board.width, &board.height)
        .avoid_snake(&you.body)
        .avoid_snakes(&board.snakes)
        .find_food(&board.food)
        .get_all_safe_moves()
        .first()
        .cloned()
        .unwrap_or(MvView::Down);

    info!("MOVE {}: {}", turn, chosen);
    json!({ "move": chosen.to_string() })
}
