extern crate crossterm;

use crossterm::{cursor::*, event::*, execute, style::*, terminal::size};
use std::io::stdout;
use std::process::exit;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

const DEBUG: bool = true;

/// clears the given amount of lines and move to the line started with
#[allow(unused)]
fn clear(length: &u16) -> bool {
    let result = size();
    if result.is_err() {
        if DEBUG {
            println!("line 12");
        }
        return false;
    }
    let result = result.unwrap();
    for _ in 0..*length {
        let result0 = execute!(stdout(), Print(format!("{}{}", " ".repeat(result.0 as usize), "\n\r")));
        if result0.is_err() {
            if DEBUG {
                println!("{}", result0.unwrap_err());
            }
            return false;
        }
    }
    let result0 = execute!(stdout(), MoveToPreviousLine(*length), MoveToColumn(1));
    if result0.is_err() {
        if DEBUG {
            println!("{}", result0.unwrap_err());
        }
        return false;
    }
    true
}

/// choose a string out of an array
/// maximum length: 16
/// minimum length: 2
#[allow(unused)]
fn choose(options: &[String]) -> Option<u8> {
    if options.len() > 16 || options.len() < 2 {
        if DEBUG {
            println!("Length of options needs to be  1 < len < 17 but is {}", options.len());
        }
        return None;
    }
    let result1 = execute!(stdout(), Hide);
    if result1.is_err() {
        if DEBUG {
            println!("{}", result1.unwrap_err());
        }
        return None;
    }
    let result3 = enable_raw_mode();
    if result3.is_err() {
        if DEBUG {
            println!("{}", result3.unwrap_err());
        }
        return None;
    }
    let mut selected: u8 = 0;
    fn prt(options: &[String], selected: &mut u8) -> bool {
        let mut i = 0;
        for option in options {
            if *selected == i {
                let result = execute!(
                    stdout(),
                    PrintStyledContent(format!("> {}\n\r", option).blue())
                );
                if result.is_err() {
                    if DEBUG {
                        println!("{}", result.unwrap_err());
                    }
                    return false;
                }
            } else {
                let result = execute!(
                    stdout(),
                    PrintStyledContent(format!("  {}\n\r", option).blue())
                );
                if result.is_err() {
                    if DEBUG {
                        println!("{}", result.unwrap_err());
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
        let result = read();
        if result.is_err() {
            if DEBUG {
                println!("{}", result.unwrap_err());
            }
            return None;
        }
        let result = result.unwrap();
        match result {
            Event::Key(it) => {
                if it.code == KeyCode::Up {
                    if selected > 0 {
                        selected -= 1;
                        if execute!(stdout(), MoveToPreviousLine((options.len()) as u16), MoveToColumn(1)).is_err()
                            || !clear(&(options.len() as u16))
                            || !prt(options, &mut selected) {
                            if DEBUG {
                                println!("line 105");
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
                                println!("line 117");
                            }
                            return None;
                        }
                    }
                } else if it.code == KeyCode::Enter {
                    if execute!(stdout(), MoveToPreviousLine(options.len() as u16), MoveToColumn(1)).is_err()
                        || !clear(&(options.len() as u16)) {
                        if DEBUG {
                            println!("line 127");
                        }
                        return None;
                    }
                    let result2 = execute!(stdout(), Show);
                    if result2.is_err() {
                        if DEBUG {
                            println!("{}", result2.unwrap_err());
                        }
                        return None;
                    }
                    let result3 = disable_raw_mode();
                    if result3.is_err() {
                        if DEBUG {
                            println!("{}", result3.unwrap_err());
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

fn main() {
    println!("Choose some:");
    let result = choose(&["a".to_string(), "b".to_string(), "c".to_string()]);
    if result.is_none() {
        println!("Err");
        exit(-1);
    }
    println!("{}", result.unwrap());
}
