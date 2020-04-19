mod c_lib;

use std::env;

use c_lib::{ c_main, c_Main_ReadParameters };

fn main() {
    let args = env::args();

    c_main(args.len(), args.collect());
}
