//! File containing functionality of the timer
use indicatif::{ProgressBar, ProgressStyle};

// Represents the current timer mode work, break or long_break
enum Mode 
{
    Work, 
    Break,
    LongBreak,
}

// Contains all timer functionality.
pub struct Timer 
{
    // Work timer total length in seconds. 
    work_time: u32,
    // Break timer total length in seconds.
    break_time: u32,
    /// Contains the time passed 
    passed_time: u32,
    // Is the timer going ?
    play: bool,
    // Count of how many timer work, break or long_break.
    // Will be used to determine if break or long break  
    num_timer: u32,
    // Contains the information for the current mode 
    mode: Mode,
    // holds the progressbar struct. 
    bar: ProgressBar,
}

// Contains the functionality for timer 
impl Timer 
{
    // Public functions 

    // Constructs a new timer 
    pub fn new(work_time: u32, break_time: u32) -> Self 
    {
        let mut timer = Timer 
        {
            work_time,
            break_time,
            passed_time: 0,             // Pass time will always start at zero
            play: true,                 // Start the timer going 
            num_timer: 1,               // First timer 
            mode: Mode::Work,           // Timer will always start as work 
            bar: ProgressBar::new(1),   // Create a place holder progess bar 
            
        };
        timer.bar = create_bar(&mut timer);

        timer
    }
    
    // Play/pause switch 
    pub fn switch(&mut self)
    {   
        self.play = match self.play {
            true => false, 
            false => true,
        };
    }

    // Increment the timer if not paused 
    pub fn inc(&mut self) 
    {
        // If term is starter 
        if self.play {
            // Change the passed time 
            self.passed_time += 1;
            // Increment the bar 
            self.bar.inc(1);
        }
    }

    // Resets the timer if complete 
    pub fn reset_if_complete(&mut self) 
    {
        // If true reset the timer 
        if self.check_complete() {
            // Change the mode 
            self.mode = match self.mode {
                Mode::Work => {
                    Mode::Break     // TODO add logic for long breaks 
                }
                Mode::Break | Mode::LongBreak => Mode::Work,
            };
            self.play = false;      // Stop the timer 
            self.passed_time = 0;   // Reset the passed time 
            self.bar = create_bar(&self);
        }
    }

    // Private functions 

    // Returns true if the timer is complete false otherwise 
    fn check_complete(&self) -> bool 
    {
        match self.mode {
            Mode::Work => self.passed_time >= self.work_time,
            Mode::Break => self.passed_time >= self.break_time,
            Mode::LongBreak => self.passed_time >= self.break_time, // TODO complete implementation of long breaks
        }
    }
}


// Creates a progressbar based on Mode. 
fn create_bar(timer: &Timer) -> ProgressBar
{
    let bar = ProgressBar::new(timer.work_time.try_into().unwrap());
    
    // Create titles and stylelize the progress bar
    match timer.mode
    {
        Mode::Work => {
            println!("Why");
            bar.println("Work timer");
            bar.set_style(ProgressStyle::default_bar()
                .template("{bar:40.red/white} {elapsed_precise}"))
        }
        Mode::Break => {
            bar.println("Break timer");
            bar.set_style(ProgressStyle::default_bar()
                .template(""))
        }
        Mode::LongBreak => {
            bar.println("Long break timer");
            bar.set_style(ProgressStyle::default_bar()
                .template(""))
        }
    }
    bar
}
