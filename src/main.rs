use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, size}, style::Print, cursor::{MoveTo, Hide, Show}};
use std::io::{self};
use std::time::Duration;

fn main() -> io::Result<()> {
	let mut x: u16 = 0;
	let mut y: u16 = 0;
	let mut velocityy: i16 = 0;
	let mut prev_x: u16 = 0;
	let mut prev_y: u16 = 0;	
    execute!(io::stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All),  crossterm::cursor::MoveTo(0, 0))?; 
    let mut stdout = io::stdout(); 
    enable_raw_mode()?;
	execute!(stdout, Hide).unwrap();
   		
    loop {
   		let mut right = false;
   		let mut left = false;
   		let mut up = false;
   		let mut down = false;    

		if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = read()? {
                if key.kind == KeyEventKind::Press {
                	match key.code {
                    	KeyCode::Char('d') => right = true,
                        KeyCode::Char('a') => left = true,
                        KeyCode::Char('w') => up = true,
                        KeyCode::Char('s') => down = true,
                        KeyCode::Char('q') => {
                        	execute!(io::stdout(), Show).unwrap();
                            disable_raw_mode()?;
                            return Ok(());
                        },
                        _ => {}
                    }
                }
            }
			if right {
				x += 1;
			}
			if left {
				x = x.saturating_sub(1);
			}
			if down {
				y += 1;
				velocityy += 2;
			}
			if up {
				velocityy -= 2;
				y = y.saturating_sub(1);
			}

			y = (y as i16 + velocityy).max(0) as u16;
                
			if x != prev_x || y != prev_y {
				execute!(io::stdout(), MoveTo(prev_x.max(0), prev_y.max(0)), Print(" "))?;
				execute!(io::stdout(), MoveTo(x.max(0), y.max(0)), Print("i"))?;
				prev_y = y;
				prev_x = x;
			}
    	}	  
    }
}
