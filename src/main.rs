use clap::Parser;
use bv::BitVec;
use rand::Rng;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Grid height
    #[clap(short, long, default_value_t = 20)]
    height: u64,

    /// Grid width
    #[clap(short, long, default_value_t = 20)]
    width: u64,

    // Initial state density
    #[clap(short, long, default_value_t = 50)]
    density: u8,
}

fn output_header(args: &Args) {
    println!("Conway's Game of Life");
    println!("");
    println!("Parameters");
    println!("---------------------------");
    println!("Grid Height: {}", args.height);
    println!("Grid Width: {}", args.width);
    println!("Initial State Density: {}", args.density);
    println!("---------------------------");
    println!("");
}

fn generate_initial_state(args: &Args, bv1: &mut BitVec) {
    let mut rng = rand::thread_rng();
    for i in 0..args.height * args.width {
        if rng.gen_range(0..100) < args.density {
            bv1.set(i, true);
        }
    }
}

fn output_state(args: &Args, state_number: u64, bv1: &BitVec) {
    println!("State: {}", state_number);
    for i in 0..args.width + 2 {
        print!("-");
    }
    print!("\n");
    for i in 0..args.height {
        print!("|");
        for j in 0..args.width {
            if bv1.get(i * args.width + j) == true {
                print!("O");
            } else {
                print!(" ");
            }
        }
        print!("|\n");
    }
    for i in 0..args.width + 2 {
        print!("-");
    }
    print!("\n");
}

fn main() {
    let args = Args::parse();
    output_header(&args);
    let mut bv1: BitVec = BitVec::new_fill(false, args.height * args.width);
    generate_initial_state(&args, &mut bv1);
    let mut state_number = 0;
    output_state(&args, state_number, &bv1);
}
