use std::fs;
use std::io::{stdin, Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
    #[doc="Print each line of text with placeholder character,\
    up to the maximum length of a line of text."]
//
struct Opts {
    #[structopt(parse(from_os_str))]
    infile: Option<PathBuf>,

    ///      use single CHARACTER to filling            
    #[structopt(name = "fill=CHARACTER", short = "f", long = "fill", default_value = " ")]
    char_fill: char,

    ///      use positive INTEGER to expand max lines    
    #[structopt(name = "number=INTEGER", short = "n", long = "number", default_value = "0")]
    number_fill: usize,

    ///      use STRING to filling                      
    #[structopt(name = "start=STRING", short = "s", long = "start", default_value = "")]
    str_start_fill: String,

    ///      use STRING to filling                      
    #[structopt(name = "end=STRING", short = "e", long = "end", default_value = "")]
    str_end_fill: String,

    #[structopt(short = "c", long = "create")]
    create: bool,
}

fn main() {
    let opts = Opts::from_args();
    let mut buff = String::new();

    if opts.create == false {
        match opts.infile {
            Some(path) => {buff = fs::read_to_string(path).expect("Open file error")}
            None => {stdin().read_to_string(&mut buff).expect("Read stdin error");}
        }

        let max_line: usize = match buff.lines().map(|line| line.chars().count()).max() {
            Some(value) => value,
            None => panic!("Open file is emty"),
        };

        for current_line in buff.lines() {
            let mut current_fill = String::new();

            for _ in 0..(max_line - current_line.chars().count() + opts.number_fill) {
                current_fill.push(opts.char_fill);
            }

            std::io::stdout()
                .write(format!("{}{current_line}{current_fill}{}\n", opts.str_start_fill, opts.str_end_fill)
                .as_bytes()).expect("Write to stdout error");
        } 
    } else {
        let mut create_flag_fill = String::new();

        for _ in 0..opts.number_fill {
            create_flag_fill.push(opts.char_fill);
        }

//        println!("{}{create_flag_fill}{}", opts.str_start_fill, opts.str_end_fill);
//        
        std::io::stdout()
            .write(format!("{}{create_flag_fill}{}\n", opts.str_start_fill, opts.str_end_fill)
            .as_bytes()).expect("Write to stdout error");
    }
}
