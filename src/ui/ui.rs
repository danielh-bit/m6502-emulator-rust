use std::{
    io::{self, Write},
    thread::sleep,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode};

use crate::{
    assembler::Assembler,
    m6502::CPU,
    memory::Memory,
    ui::tokenizer::{Conifgurations, DotCommands, RunCommands, Token, TokenType},
};

pub fn start_inteface() {
    print_logo();

    sleep(Duration::from_secs(2));
    let mut inst_time = 0;

    loop {
        let mut input_line = String::new();

        print!("~ computer: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Faild to read user input");

        match Token::create(input_line.trim()) {
            Ok(token) => match token.command_type {
                TokenType::DotCommands(content) => match content {
                    DotCommands::Quit => break,
                    DotCommands::PrintCommands => print_all_commands(),
                },
                TokenType::RunTypes(content) => match content {
                    RunCommands::Default => run_program("default", inst_time),
                    RunCommands::UserDefined(program) => run_program(&program, inst_time),
                },
                TokenType::Configurations(content) => match content {
                    Conifgurations::Default => panic!("not implemented yed"),
                    Conifgurations::SetInstructionTime(time) => inst_time = time,
                    Conifgurations::Memory => panic!("not implemented yed"),
                },
            },
            Err(error) => println!("{}", error),
        };
    }
}

fn run_program(program: &str, inst_time: u64) {
    // the start location is normally defined by FFFE and FFFF but this will work like this because i cant bother.
    let (program, start_location) = Assembler::assemble(program);
    if program == Vec::new() {
        return;
    }
    let mem = Memory::default_init(program);
    let now = Instant::now();
    let mut cpu = CPU::new(mem, start_location);
    println!("      Press 'esc' to stop program");
    //program loop. Run until not let some.
    'program: loop {
        while event::poll(Duration::default()).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Esc => {
                        break 'program;
                    }
                    _ => {}
                }
            }
        }
        if let Some(x) = cpu.execute_instruction() {
            if inst_time != 0 {
                for _ in 0..x {
                    sleep(Duration::from_millis(inst_time));
                }
            }
        } else {
            println!("bye");
            break 'program;
        }
    }
    println!("Runtime: {:.2?}", now.elapsed());
}

fn print_all_commands() {
    println!("    run -df                             - runs the default program in the 'programs' folder
    run <enter program name>            - runs a program with the same name in the 'programs' folder
    .q                                  - quit the program
    .help                               - prints this
    configure -It *enter time in milis* - configures the delay between instructions (default is no delay)")
}

fn print_logo() {
    let logo = r"
        +--+--+  +-----+  +------
        |  |  |  |     |  |      
        |  |  |  |     |   \---   
        |  |  |  |     |       \ 
        |  |  |  +-----+  ------+  6502";
    println!("{} \n", logo)
}
