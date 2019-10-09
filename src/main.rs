extern crate maze;

use maze::square_maze::{SquareMaze, WallDirection};
use maze::gen;
use maze::meta::{to_hex_string, MetaData};
use std::{thread, time, env};
use std::collections::{HashMap};

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

fn main() {

    let lines_str = env::var("LINES");
    let columns_str = env::var("COLUMNS");

    let height : usize = lines_str.expect("lines").parse().expect("invalid line number");
    let width : usize = columns_str.expect("columns").parse().expect("invalid column number");
    let mut maze = SquareMaze::new_filled((width-1)/3, (height-1)/2);
    let seed = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let mut meta = MetaData::new_empty();
    meta.seed = to_hex_string(seed).to_string();
    gen::recursive(&mut maze, seed);


    let edge_map = init_edge_map();

    print!("\x1B[2J");//clear
    print!("\x1B[1;1H");

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
            if x == maze.width-1 {
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

            if y == maze.height - 1 {
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

    print!("\x1B[2;2H");
    println!("\u{1F642}");

    //println!("\x1B[10A");//move

    thread::sleep(time::Duration::from_secs(300));
}
