use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   num: i32,

   // Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}

fn main() {
    let args = Args::parse();
    trinum(args.num);
}

fn trinum(f: i32) {
    // Create variables
    let mut trinum_count = f;
    let tri_num_perm = f * (f + 1) / 2;
    let mut tri_num = tri_num_perm;
   
    // Remove first layer
    println!("{}", tri_num);
    tri_num -= trinum_count; 
    println!("{}", tri_num);
    trinum_count -= 1;

    // Loop layer reduction
    while tri_num != tri_num_perm {
        tri_num -= trinum_count;
        println!("{}",tri_num);
        trinum_count -= 1;
    }
}
