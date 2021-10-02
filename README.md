# terminal-utils
A small library for creating CLIs using [crossterm-rs](https://github.com/crossterm-rs/crossterm).

## TODO
- Add yes/no decision function.

## How To
> use library
???
```rs
extern crate terminal_utils;
```

> clear function
```rs
use terminal_utils::clear;

clear(&2);
```
This will go up 2 lines from the current line, then clears the 
current line and the line beneath and will go up to the first 
cleared line again.

> choose function
```rs
use terminal_utils::choose;

let result = choose(&["A".to_string(), "B".to_string(), "C".to_string()], 
  " > ".dark_grey().on(Color::White),
  " < ".dark_grey().on(Color::White),
  (Some(Color::Black), Some(Color::White)),
  "   ".stylize(),
  "   ".stylize(),
  (None, None),
  true);
if result.is_ok() {
  println!("{}", result.unwrap_or_default());
} else {
  println!("{}", result.unwrap_err().err_msg);
}
```
![alt text](https://github.com/sqyyy-jar/terminal-utils/blob/master/images/look.png "Preview")  
This will let you choose in the console one of 'A', 'B' and 'C'.  
The maximum amount of options is 16 and the minimal is 2 but it
is changeable in the code.  
You have an infinite cycle, which means if you go above the first option 
you land back again at the bottom and the other way around.  
![alt text](https://github.com/sqyyy-jar/terminal-utils/blob/master/images/infinite_cycle.gif "Infinite cycle")  
You can cicle with the arrow keys and confirm with enter.  
It will return an error or the selected index off the selected
option as i8.  
After selecting it will delete all written lines.  
Here the chooser will look as follows:  
- if the option is selected, it will be written like this ' > option < '  
- the arrows will be dark grey and the text will be black  
- the background will be white  
- if the option is not selected, it will be written like this '   option   '  
- the background and foreground color will be the default of the console  
