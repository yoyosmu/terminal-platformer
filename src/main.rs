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
	
	draw_ground(&mut stdout)?;
   		
    loop {
   		let mut right = false;
   		let mut left = false;
   		let mut up = false;
   		let mut down = false;    
		let (cols, rows) = size()?;
		let max_x = cols.saturating_sub(1);
	    let max_y = rows.saturating_sub(1);

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
                x = x.saturating_add(1);
            }
            if left {
                x = x.saturating_sub(1);
            }
            if down {
                y = y.saturating_add(1);
                velocityy = velocityy.saturating_add(1);
            }
            if up && y == 27  {
                velocityy = velocityy.saturating_sub(4);
                y = y.saturating_sub(1);
            }

			y = y.saturating_add(1);
			velocityy = velocityy.saturating_add(1);
			y = (y as i16 + velocityy).max(0) as u16;

			if y >= max_y {
			    y = max_y;         
			    velocityy = 0;   
			}
			
			if y == 0 {
			    y = 0;            
			    if velocityy < 0 { 
			        velocityy = 0;
			    }
			}

			if y >= 27 {
				y = 27;
				velocityy = 0;
			}
			
			x = x.min(max_x);
            y = y.min(max_y);
                
			if x != prev_x || y != prev_y {
				execute!(io::stdout(), MoveTo(prev_x.max(0), prev_y.max(0)), Print(" "))?;
				execute!(io::stdout(), MoveTo(x.max(0), y.max(0)), Print("i"))?;
				prev_y = y;
				prev_x = x;
			}
    	}	  
    }
}

fn draw_ground(stdout: &mut io::Stdout) -> io::Result<()> {
	let (width, height) = size()?;
	let ground_y = height.saturating_sub(20);
	
    execute!(stdout, MoveTo(0, ground_y))?;
    for _ in 0..width {
        execute!(stdout, Print('▔'))?;
    }
    Ok(())
}
