use getopts::Options;
use std::env;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

static mut SEED: u32 = 0;

#[derive(Debug)]
struct ArgsStruct {
    start: u32,
    end: u32,
    quantity: u32,
}

unsafe fn rand_global(start: u32, end: u32) -> u32 {
    if SEED == 0 {
        let epoc = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        SEED = epoc.as_millis() as u32;
    }
    SEED ^= SEED << 13;
    SEED ^= SEED >> 17;
    SEED ^= SEED << 5;
    return SEED % (end - start + 1) + start;
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} START END QUANTITY", program);
    print!("{}", opts.usage(&brief));
}

fn input_arg() -> ArgsStruct {
    let args: Vec<String> = env::args().collect();
    let program = &args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    if matches.free.is_empty() {
        print_usage(&program, opts);
        process::exit(0);
    }
    if matches.opt_present("h") {
        print_usage(&program, opts);
        process::exit(0);
    }

    let start = &args[1];
    let end = &args[2];
    let quantity = &args[3];
    let start: u32 = start.parse().unwrap();
    let end: u32 = end.parse().unwrap();
    let quantity: u32 = quantity.parse().unwrap();

    let args_st = ArgsStruct {
        start: start,
        end: end,
        quantity: quantity,
    };
    return args_st;
}

fn main() {
    let arg = input_arg();
    unsafe {
        for _ in 0..arg.quantity {
            let v = rand_global(arg.start, arg.end);
            println!("{}", v);
        }
    }
}
