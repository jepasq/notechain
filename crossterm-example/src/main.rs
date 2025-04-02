//importing in execute! macro
#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::stdout;

fn main() {
    let mut stdout = stdout();
    //going into raw mode
    enable_raw_mode().unwrap();

    //clearing the screen, going to top left corner and printing welcoming message
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), Print(r#"ctrl + q to exit, ctrl + h to print "Hello world", alt + t to print "crossterm is cool""#)).unwrap();

    // The acrual command we will push pressed character to
    let mut actual_command = String::new(); 
    
    //key detection
    loop {
        //going to top left corner
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
	
        //matching the key
        match read().unwrap() {
            //i think this speaks for itself
            Event::Key(KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => execute!(stdout, Clear(ClearType::All), Print("Hello world!")).unwrap(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('t'),
                modifiers: KeyModifiers::ALT,
            }) => execute!(stdout, Clear(ClearType::All), Print("crossterm is cool")).unwrap(),
	    Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
            }) => execute!(stdout, Clear(ClearType::CurrentLine),
			   Print("Up pressed!")).unwrap(),

            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            }) => break,

	    // Gérer toutes les autres touches
	    Event::Key(KeyEvent {
		code: KeyCode::Char(c),
		modifiers: KeyModifiers::NONE,
	    }) =>
	    {
		actual_command.push(c);
		execute!(stdout, cursor::MoveTo(0, 2), 
			 Print(format!("Texte actuel '{}'", actual_command)))
		    .unwrap();
	    }
            _ => (),
	}
    }

    //disabling raw mode
    disable_raw_mode().unwrap();
}
