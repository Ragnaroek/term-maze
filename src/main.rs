extern crate maze;

use maze::square_maze::{SquareMaze, WallDirection};
use maze::gen;
use maze::meta::{to_hex_string, MetaData};
use std::{thread, time, env};

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

    print!("\x1B[2J");//clear
    print!("\x1B[1;1H");

    for x in 0..maze.width {
        if maze.wall(WallDirection::NORTH, x, 0) {
            print!("\u{2533}\u{2501}\u{2501}");
        } else {
            print!("\u{2533}  ");
        }
    }
    println!("");

    for y in 0..maze.height {
        for x in 0..maze.width {
            if maze.wall(WallDirection::WEST, x, y) {
                print!("\u{2503}  ");
            } else {
                print!("   ")
            }
        }
        println!("");
        for x in 0..maze.width {
            if maze.wall(WallDirection::SOUTH, x, y) {
                print!("\u{254B}\u{2501}\u{2501}")
            } else {
                print!("\u{254B}  ")
            }
        }
        println!("");
    }

    print!("\x1B[2;2H");
    println!("\u{1F642}");

    //println!("\x1B[10A");//move

    thread::sleep(time::Duration::from_secs(300));
}
