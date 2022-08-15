//! A Second attempt at a pomo timer written in rust 
use pomo::Timer;
use clap::Parser;
use console::Term;
use std::thread;
use std::time::Duration;

/// Command line argument struct 
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args 
{
    /// Length of work time in minutes
    #[clap(value_parser, default_value_t = 25)]
    work_time: u32,
    /// Length of break time in minutes 
    #[clap(value_parser, default_value_t = 5)]
    break_time: u32,
}

fn main()
{
    // Get the arguments from the user putting them in the struct 
    let args = Args::parse();

    let input = Term::buffered_stdout();

    let mut timer = Timer::new(args.work_time, args.break_time);
    timer.inc();

    // Main control flow loop 
    // TODO Make this take run in paraellel 
    loop 
    {
        // Have the main thread sleep for 1 milisecond
        thread::sleep(Duration::from_millis(100));
        timer.inc();
        // Key listener will watch for q and space.
        // q -> quits the program
        // space -> pauses/unpauses the timer 
        if let Ok(character) = input.read_char() {
            match character {
                'q' => break,
                ' ' => timer.switch(),
                _ => continue,
            }
        }
    }
    println!("Exiting the pomo timer");
}
