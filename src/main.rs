extern crate maze;
extern crate termion;

use maze::square_maze::{SquareMaze, WallDirection, MazeCell};
use maze::gen;
use maze::meta::{to_hex_string, MetaData};
use std::{thread, time, env, io};
use std::collections::{HashMap};

use termion::input::TermRead;
use termion::raw::IntoRawMode;

//                                  |
//                                  1
//                                  |
//counter-clockwise edge pattern:-2-|-4-
//                                  |
//                                  3
//                                  |
#[derive(Eq, PartialEq, Hash, Debug)]
struct EdgePattern {
    top: bool, //1
    left: bool, //2
    bottom: bool, //3
    right: bool //4
}

impl EdgePattern {
    fn new(top:bool, left:bool, bottom:bool, right:bool) -> EdgePattern {
        EdgePattern{top, left, bottom, right}
    }
}

fn init_edge_map() -> HashMap<EdgePattern, char> {
    let mut map = HashMap::new();
    map.insert(EdgePattern::new(false, false, false, false), ' ');
    map.insert(EdgePattern::new(false, false, false, true), '━');
    map.insert(EdgePattern::new(false, false, true, false), '╻');
    map.insert(EdgePattern::new(false, false, true, true), '┏');
    map.insert(EdgePattern::new(false, true, false, false), '━');
    map.insert(EdgePattern::new(false, true, false, true), '━');
    map.insert(EdgePattern::new(false, true, true, false), '┓');
    map.insert(EdgePattern::new(false, true, true, true), '┳');
    map.insert(EdgePattern::new(true, false, false, false), '╹');
    map.insert(EdgePattern::new(true, false, false, true), '┗');
    map.insert(EdgePattern::new(true, false, true, false), '┃');
    map.insert(EdgePattern::new(true, false, true, true), '┣');
    map.insert(EdgePattern::new(true, true, false, false), '┛');
    map.insert(EdgePattern::new(true, true, false, true), '┻');
    map.insert(EdgePattern::new(true, true, true, false), '┫');
    map.insert(EdgePattern::new(true, true, true, true), '╋');
    map
}

#[derive(Debug)]
struct GameState {
    player_x: usize,
    player_y: usize,
}

const SIMPLE_RENDER:bool = true;

fn main() {

    let lines_str = env::var("LINES");
    let columns_str = env::var("COLUMNS");

    let height : usize = lines_str.expect("lines").parse().expect("invalid line number");
    let width : usize = columns_str.expect("columns").parse().expect("invalid column number");
    let mut maze = SquareMaze::new_filled((width-2)/3, (height-1)/2);
    let seed = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let mut meta = MetaData::new_empty();
    meta.seed = to_hex_string(seed).to_string();
    gen::recursive(&mut maze, seed, MazeCell::new(0, 0));

    let edge_map = init_edge_map();
    let mut game_state = GameState{
        player_x: 0,
        player_y: 0,
    };

    print!("\x1B[2J");//clear
    print!("\x1B[?25l");
    print!("\x1B[1;1H");

    if SIMPLE_RENDER {

        for y in (0..maze.height).rev() {
            for x in 0..maze.width {
                print!("╋");
                if maze.wall(WallDirection::NORTH, x, y) {
                    print!("━━");
                } else {
                    print!("  ");
                }
            }
            println!("");

            for x in 0..maze.width {
                if maze.wall(WallDirection::WEST, x, y) {
                    print!("┃");
                } else {
                    print!(" ");
                }
                print!("  ")
            }
            println!("");
        }

        for _ in 0..maze.width {
            print!("╋━━");
        }

    } else {

    //top-row
    for x in 0..maze.width {
        if x == 0 {
            print!("┏");
        }

        let right_edge;
        if x == maze.width-1 {
            right_edge = '┓';
        } else {
            right_edge = if maze.wall(WallDirection::EAST, x, 0) {'┳'} else {'━'};
        }
        print!("━━{}", right_edge);
    }
    println!("");

    for y in 0..maze.height {
        for x in 0..maze.width {
            if maze.wall(WallDirection::WEST, x, y) {
                print!("┃  ");
            } else {
                print!("   ")
            }
            if x == maze.width-1 && y == maze.height-1 {
                print!("🚪");
            } else if x == maze.width-1 {
                print!("┃");
            }
        }
        println!("");
        for x in 0..maze.width {
            if x == 0 && y < (maze.height - 1) {
                if maze.wall(WallDirection::SOUTH, x, y) {
                    print!("┣");
                } else {
                    print!("┃");
                }
            } else if x == 0 && y == maze.height -1 {
                print!("┗")
            }

            if y == maze.height - 1 && x == maze.width - 1 {
                print!("━━┛");
            } else if y == maze.height - 1 {
                if maze.wall(WallDirection::EAST, x, y) {
                    print!("━━┻")
                } else {
                    print!("━━━");
                }
            } else {
                let right_edge;
                if x == maze.width - 1 {
                    if maze.wall(WallDirection::SOUTH, x, y) {
                        right_edge = '┫';
                    } else {
                        right_edge = '┃';
                    }
                } else {
                    let edge_pattern = EdgePattern::new(
                        maze.wall(WallDirection::EAST, x, y),
                        maze.wall(WallDirection::SOUTH, x, y),
                        maze.wall(WallDirection::EAST, x, y+1),
                        maze.wall(WallDirection::SOUTH, x+1, y)
                    );

                    right_edge = *edge_map.get(&edge_pattern).unwrap_or(&'o');
                }

                if maze.wall(WallDirection::SOUTH, x, y) {
                    print!("━━{}", right_edge);
                } else {
                    print!("  {}", right_edge);
                }
            }
        }
        println!("");
    }
    }

    let (start_x, start_y) = maze_pos(maze.entry.x, maze.entry.y, maze.height);
    print!("\x1B[{};{}H", start_y, start_x);
    println!("\u{1F642}");

    let _ = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();
    loop {
            let input = stdin.next();
            if let Some(Ok(key)) = input {
                match key {
                    // Exit if 'q' is pressed
                    termion::event::Key::Char('q') => break,
                    _ => update_player_pos(key, &maze, &mut game_state)
                }
            }
            thread::sleep(time::Duration::from_millis(50));
        }
}

fn update_player_pos(key: termion::event::Key, maze: &SquareMaze, state: &mut GameState) {
    let old_x = state.player_x;
    let old_y = state.player_y;
    if key == termion::event::Key::Left {
        if !maze.wall(WallDirection::WEST, state.player_x, state.player_y) && state.player_x >= 1 {
            state.player_x -= 1;
        }
    } else if key == termion::event::Key::Right {
        if !maze.wall(WallDirection::EAST, state.player_x, state.player_y) {
            state.player_x += 1;
        }
    } else if key == termion::event::Key::Up {
        if !maze.wall(WallDirection::NORTH, state.player_x, state.player_y) {
            state.player_y += 1;
        }
    }  else if key == termion::event::Key::Down  {
        if !maze.wall(WallDirection::SOUTH, state.player_x, state.player_y) && state.player_y >= 1 {
            state.player_y -= 1;
        }
    }

    if state.player_x != old_x || state.player_y != old_y {
        let (old_x_maze, old_y_maze) = maze_pos(old_x, old_y, maze.height);
        print!("\x1B[{};{}H", old_y_maze, old_x_maze);
        println!(" ");
        let (x_maze, y_maze) = maze_pos(state.player_x, state.player_y, maze.height);
        print!("\x1B[{};{}H", y_maze, x_maze);
        println!("\u{1F642}");
    }
}

//transforms maze (x,y) coordinates into terminal (x, y) coordinates
fn maze_pos(x: usize, y: usize, maze_height: usize) -> (usize, usize) {
    let h_maze = 2+maze_height*2;
    return (2+x*3, h_maze - (2+y*2))
}
