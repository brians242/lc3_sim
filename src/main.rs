pub mod components;
use components::vm::VM;

use termios::*;

use byteorder::{BigEndian, ReadBytesExt};

use std::{fs::File, io::BufReader};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let stdin = 0;
    let termios = termios::Termios::from_fd(stdin).unwrap();

    let mut new_termios = termios.clone();
    new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
    new_termios.c_lflag &= !(ICANON | ECHO);

    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();

    let mut vm = VM::new();

    let cli = Cli::from_args();

    let f = File::open(cli.path).expect("couldn't open file");
    let mut f = BufReader::new(f);

    // reading through binary
    let base_address = f.read_u16::<BigEndian>().expect("error");

    // utilize memory
    let mut address = base_address as usize;

    while let Ok(instruction) = f.read_u16::<BigEndian>() {
        vm.write_memory(address, instruction);
        address += 1;
    }
    
    if let Err(e) = f.read_u16::<BigEndian>() {
        if e.kind() == std::io::ErrorKind::UnexpectedEof {
            println!("checked!");
        } else {
            println!("fails: {}", e);
        }
    }    

    components::execute_program(&mut vm);

    // reset stdin
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}