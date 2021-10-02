extern crate crossterm;

use crossterm::{
    cursor::*,
    event::*,
    execute,
    style::*,
    terminal::{disable_raw_mode, enable_raw_mode, size},
    ErrorKind,
};
use std::io::stdout;

pub enum TerminalErrKind {
    ArrayTooLongErr,
    ConsoleWriteErr(ErrorKind),
}

pub struct TerminalError {
    pub err_type: TerminalErrKind,
    pub err_msg: String,
}

impl TerminalError {
    pub fn new(err_type: TerminalErrKind, err_msg: String) -> Self {
        Self { err_type, err_msg }
    }
}

/// clears the given amount of lines and moves the given amount of lines up
#[allow(unused)]
pub fn clear(length: &u16) -> Option<TerminalError> {
    let result_size = size();
    if result_size.is_err() {
        return Some(TerminalError::new(
            TerminalErrKind::ConsoleWriteErr(result_size.unwrap_err()),
            "An console write error fired!".to_string(),
        ));
    }
    let size = result_size.unwrap();
    for _ in 0..*length {
        let result_cl0 = execute!(
            stdout(),
            Print(format!("{}{}", " ".repeat(size.0 as usize), "\n\r"))
        );
        if result_cl0.is_err() {
            return Some(TerminalError::new(
                TerminalErrKind::ConsoleWriteErr(result_cl0.unwrap_err()),
                "An console write error fired!".to_string(),
            ));
        }
    }
    let result_cl1 = execute!(stdout(), MoveToPreviousLine(*length));
    if result_cl1.is_err() {
        return Some(TerminalError::new(
            TerminalErrKind::ConsoleWriteErr(result_cl1.unwrap_err()),
            "An console write error fired!".to_string(),
        ));
    }
    None
}

/// choose a string out of an array
///
/// Parameters:
/// - options: &[String] - the options to choose one from
/// - selected_prefix: StyledContent<&str> - the prefix displayed in front of the selected option
/// - selected_suffix: StyledContent<&str> - the suffix displayed behind the selected option
/// - selected_color: (Option<Color>, Option<Color>) - the front- (0) and background- (1) color
///                                                    (Default if None)
/// - prefix: StyledContent<&str> - the prefix displayed in front of the not selected option(s)
/// - suffix: StyledContent<&str> - the suffix displayed behind the not selected option(s)
/// - color: (Option<Color>, Option<Color>) - the front- (0) and background- (1) color
///                                                    (Default if None)
/// - infinite_cycle: bool - whether to use an infinite loop or not
/// (first element selected -> push arrow-key up -> last element selected - and the other way around)
#[allow(unused)]
pub fn choose(
    options: &[String],
    selected_prefix: StyledContent<&str>,
    selected_suffix: StyledContent<&str>,
    selected_color: (Option<Color>, Option<Color>),
    prefix: StyledContent<&str>,
    suffix: StyledContent<&str>,
    color: (Option<Color>, Option<Color>),
    infinite_cycle: bool,
) -> Result<i8, TerminalError> {
    if options.len() > 16 || options.len() < 2 {
        return Err(TerminalError::new(
            TerminalErrKind::ArrayTooLongErr,
            format!(
                "Length of options needs to be  1 < len < 17 but is {}!",
                options.len()
            )
                .to_string(),
        ));
    }
    fn console_write_err(err: ErrorKind) -> Result<i8, TerminalError> {
        Err(TerminalError::new(
            TerminalErrKind::ConsoleWriteErr(err),
            "An console write error fired!".to_string(),
        ))
    }
    let result_ch0 = execute!(stdout(), Hide);
    if result_ch0.is_err() {
        return console_write_err(result_ch0.unwrap_err());
    }
    let result_ch1 = enable_raw_mode();
    if result_ch1.is_err() {
        return console_write_err(result_ch1.unwrap_err());
    }
    let mut selected: i8 = 0;
    fn prt(
        options: &[String],
        selected: &mut i8,
        selected_prefix: StyledContent<&str>,
        selected_suffix: StyledContent<&str>,
        selected_color: (Option<Color>, Option<Color>),
        prefix: StyledContent<&str>,
        suffix: StyledContent<&str>,
        color: (Option<Color>, Option<Color>),
    ) -> Option<ErrorKind> {
        let mut i = 0;
        for option in options {
            if *selected == i {
                let mut message = format!("{}", option).stylize();
                if selected_color.0.is_some() { message = message.with(selected_color.0.unwrap().clone()) }
                if selected_color.1.is_some() { message = message.on(selected_color.1.unwrap().clone()) }
                let result_pr0 = execute!(
                    stdout(),
                    PrintStyledContent(selected_prefix.clone()),
                    PrintStyledContent(message),
                    PrintStyledContent(selected_suffix.clone()),
                    Print("\n\r")
                );
                if result_pr0.is_err() {
                    return Some(result_pr0.unwrap_err());
                }
            } else {
                let mut message = format!("{}", option).stylize();
                if color.0.is_some() { message = message.with(color.0.unwrap().clone()) }
                if color.1.is_some() { message = message.on(color.1.unwrap().clone()) }
                let result_pr1 = execute!(
                    stdout(),
                    PrintStyledContent(prefix.clone()),
                    PrintStyledContent(message),
                    PrintStyledContent(suffix.clone()),
                    Print("\n\r")
                );
                if result_pr1.is_err() {
                    return Some(result_pr1.unwrap_err());
                }
            }
            i += 1;
        }
        None
    }
    let result_ch2 = prt(
        options,
        &mut selected,
        selected_prefix,
        selected_suffix,
        selected_color.clone(),
        prefix,
        suffix,
        color.clone(),
    );
    if result_ch2.is_some() {
        return console_write_err(result_ch2.unwrap());
    }
    loop {
        let result_ch3 = read();
        if result_ch3.is_err() {
            return console_write_err(result_ch3.unwrap_err());
        }
        let key = result_ch3.unwrap();
        match key {
            Event::Key(it) => {
                if it.code == KeyCode::Up {
                    if infinite_cycle {
                        selected -= 1;
                        if selected < 0 {
                            selected = (options.len() - 1) as i8;
                        }
                    } else {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    let result_ch4 = execute!(
                        stdout(),
                        MoveToPreviousLine((options.len()) as u16),
                        MoveToColumn(1)
                    );
                    if result_ch4.is_err() {
                        return console_write_err(result_ch4.unwrap_err());
                    }
                    let result_ch5 = clear(&(options.len() as u16));
                    if result_ch5.is_some() {
                        return Err(result_ch5.unwrap());
                    }
                    let result_ch6 = prt(
                        options,
                        &mut selected,
                        selected_prefix,
                        selected_suffix,
                        selected_color.clone(),
                        prefix,
                        suffix,
                        color.clone(),
                    );
                    if result_ch6.is_some() {
                        return console_write_err(result_ch6.unwrap());
                    }
                } else if it.code == KeyCode::Down {
                    if infinite_cycle {
                        selected += 1;
                        if selected >= options.len() as i8 {
                            selected = 0;
                        }
                    } else {
                        if selected < (options.len() - 1) as i8 {
                            selected += 1;
                        }
                    }
                    let result_ch4 = execute!(
                        stdout(),
                        MoveToPreviousLine((options.len()) as u16),
                        MoveToColumn(1)
                    );
                    if result_ch4.is_err() {
                        return console_write_err(result_ch4.unwrap_err());
                    }
                    let result_ch5 = clear(&(options.len() as u16));
                    if result_ch5.is_some() {
                        return Err(result_ch5.unwrap());
                    }
                    let result_ch6 = prt(
                        options,
                        &mut selected,
                        selected_prefix,
                        selected_suffix,
                        selected_color.clone(),
                        prefix,
                        suffix,
                        color.clone(),
                    );
                    if result_ch6.is_some() {
                        return console_write_err(result_ch6.unwrap());
                    }
                } else if it.code == KeyCode::Enter {
                    let result_ch4 = execute!(
                        stdout(),
                        MoveToPreviousLine((options.len()) as u16),
                        MoveToColumn(1),
                        Show
                    );
                    if result_ch4.is_err() {
                        return console_write_err(result_ch4.unwrap_err());
                    }
                    let result_ch5 = clear(&(options.len() as u16));
                    if result_ch5.is_some() {
                        return Err(result_ch5.unwrap());
                    }
                    let result_ch6 = disable_raw_mode();
                    if result_ch6.is_err() {
                        return console_write_err(result_ch6.unwrap_err());
                    }
                    return Ok(selected);
                }
            }
            _ => {}
        }
    }
}
