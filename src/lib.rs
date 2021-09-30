extern crate crossterm;

use crossterm::{cursor::*, event::*, execute, style::*, terminal::{size, enable_raw_mode, disable_raw_mode}};
use std::io::stdout;

const DEBUG: bool = true;

/// clears the given amount of lines and move to the line started with
#[allow(unused)]
pub fn clear(length: &u16) -> bool {
    let result_size = size();
    if result_size.is_err() {
        if DEBUG {
            println!("{}", result_size.unwrap_err());
        }
        return false;
    }
    let size = result_size.unwrap();
    for _ in 0..*length {
        let result_cl0 = execute!(stdout(), Print(format!("{}{}", " ".repeat(size.0 as usize), "\n\r")));
        if result_cl0.is_err() {
            if DEBUG {
                println!("{}", result_cl0.unwrap_err());
            }
            return false;
        }
    }
    let result_cl1 = execute!(stdout(), MoveToPreviousLine(*length));
    if result_cl1.is_err() {
        if DEBUG {
            println!("{}", result_cl1.unwrap_err());
        }
        return false;
    }
    true
}

/// choose a string out of an array
/// maximum length: 16
/// minimum length: 2
#[allow(unused)]
pub fn choose(options: &[String]) -> Option<u8> {
    if options.len() > 16 || options.len() < 2 {
        if DEBUG {
            println!("Length of options needs to be  1 < len < 17 but is {}", options.len());
        }
        return None;
    }
    let result_ch0 = execute!(stdout(), Hide);
    if result_ch0.is_err() {
        if DEBUG {
            println!("{}", result_ch0.unwrap_err());
        }
        return None;
    }
    let result_ch1 = enable_raw_mode();
    if result_ch1.is_err() {
        if DEBUG {
            println!("{}", result_ch1.unwrap_err());
        }
        return None;
    }
    let mut selected: u8 = 0;
    fn prt(options: &[String], selected: &mut u8) -> bool {
        let mut i = 0;
        for option in options {
            if *selected == i {
                let result_pr0 = execute!(
                    stdout(),
                    PrintStyledContent(format!("> {}\n\r", option).blue())
                );
                if result_pr0.is_err() {
                    if DEBUG {
                        println!("{}", result_pr0.unwrap_err());
                    }
                    return false;
                }
            } else {
                let result_pr1 = execute!(
                    stdout(),
                    PrintStyledContent(format!("  {}\n\r", option).blue())
                );
                if result_pr1.is_err() {
                    if DEBUG {
                        println!("{}", result_pr1.unwrap_err());
                    }
                    return false;
                }
            }
            i += 1;
        }
        true
    }
    prt(options, &mut selected);
    loop {
        let result_ch2 = read();
        if result_ch2.is_err() {
            if DEBUG {
                println!("{}", result_ch2.unwrap_err());
            }
            return None;
        }
        let key = result_ch2.unwrap();
        match key {
            Event::Key(it) => {
                if it.code == KeyCode::Up {
                    if selected > 0 {
                        selected -= 1;
                        if execute!(stdout(), MoveToPreviousLine((options.len()) as u16), MoveToColumn(1)).is_err()
                            || !clear(&(options.len() as u16))
                            || !prt(options, &mut selected) {
                            if DEBUG {
                                println!("err/choose/match-key-1");
                            }
                            return None;
                        }
                    }
                } else if it.code == KeyCode::Down {
                    if selected < (options.len() - 1) as u8 {
                        selected += 1;
                        if execute!(stdout(), MoveToPreviousLine((options.len()) as u16), MoveToColumn(1)).is_err()
                            || !clear(&(options.len() as u16))
                            || !prt(options, &mut selected) {
                            if DEBUG {
                                println!("err/choose/match-key-2");
                            }
                            return None;
                        }
                    }
                } else if it.code == KeyCode::Enter {
                    if execute!(stdout(), MoveToPreviousLine(options.len() as u16), MoveToColumn(1)).is_err()
                        || !clear(&(options.len() as u16)) {
                        if DEBUG {
                            println!("err/choose/match-key-3");
                        }
                        return None;
                    }
                    let result_ch3 = execute!(stdout(), Show);
                    if result_ch3.is_err() {
                        if DEBUG {
                            println!("{}", result_ch3.unwrap_err());
                        }
                        return None;
                    }
                    let result_ch4 = disable_raw_mode();
                    if result_ch4.is_err() {
                        if DEBUG {
                            println!("{}", result_ch4.unwrap_err());
                        }
                        return None;
                    }
                    return Some(selected);
                }
            }
            _ => {}
        }
    }
}