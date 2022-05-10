extern crate ncurses;

use clap::Parser;
use bv::BitVec;
use rand::Rng;
use ncurses::*;
use std::time::Duration;
use std::thread::sleep;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Grid height
    #[clap(short, long, default_value_t = 20)]
    height: i32,

    /// Grid width
    #[clap(short, long, default_value_t = 40)]
    width: i32,

    // Initial state density
    #[clap(short, long, default_value_t = 50)]
    density: u8,

    // Time interval milliseconds
    #[clap(short, long, default_value_t = 1000)]
    interval: i32,

    // Generations
    #[clap(short, long, default_value_t = 10)]
    generations: i32,
}

fn generate_initial_state(args: &Args, bv1: &mut BitVec) {
    let mut rng = rand::thread_rng();
    for i in 0..args.height * args.width {
        if rng.gen_range(0..100) < args.density {
            bv1.set(i.try_into().unwrap(), true);
        }
    }
}

fn output_state(args: &Args, bv1: &BitVec, start_y: i32, start_x: i32) {
    // Set initial cursor position
    mv(start_y, start_x);

    for _i in 0..args.width + 2 {
        addstr("-");
    }
    mv(start_y + 1, start_x);
    for i in 0..args.height {
        addstr("|");
        for j in 0..args.width {
            if bv1.get((i * args.width + j).try_into().unwrap()) == true {
                addstr("O");
            } else {
                addstr(" ");
            }
        }
        addstr("|");
        mv (start_y + 1 + i, start_x);
    }
    for _i in 0..args.width + 2 {
        addstr("-");
    }
    refresh();
}

fn main() {
    // Get command line arguments
    let args = Args::parse();
    
    // Setup ncurses
    initscr();
    raw();
    
    // Allow for extended keyboard (like F1)
    keypad(stdscr(), true);
    noecho();

    // Invisible cursor
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // Status/help info
    addstr("Conway's Game of Life\n");
    addstr("Generation: 0");
    //mvprintw(LINES() - 1, 0, "Press q to exit");
    refresh();


    // Get the screen bounds
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    // Start in the center
    let start_y = (max_y - args.height) / 2;
    let start_x = (max_x - args.width) / 2;

    // Create Initial State
    let mut bv1: BitVec = BitVec::new_fill(false, (args.height * args.width).try_into().unwrap());
    generate_initial_state(&args, &mut bv1);
    
    let mut generation = 0;

    // Main loop
    while generation < args.generations
    {
        output_state(&args, &bv1, start_y, start_x);
        sleep(Duration::from_millis(args.interval.try_into().unwrap()));
        generation += 1;
        mvprintw(1,12, &generation.to_string());
        refresh();
    }
    endwin();
}
