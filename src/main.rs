use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode}, style::Print, cursor::{MoveTo, Hide, Show}};
use std::io::{self, Write};
use std::time::Duration;


fn draw_ground() -> io::Result<(u16, u16)> {
    let mut stdout = io::stdout();
    let mut groundx: u16 = 0;
    let groundy: u16 = 12; 
    
    while groundx != 55 {
        execute!(stdout, MoveTo(groundx, groundy), Print("▔"))?;
        groundx += 1;    
    }
    
    Ok((groundx, groundy))
}

fn main() -> io::Result<()> {
	let mut stdout = io::stdout(); 
	let (groundx, groundy) = draw_ground()?; 
	let mut x: u16 = 0;
	let mut y: u16 = 0;
	let mut velocityx: i16 = 0;
	let mut velocityy: i16 = 0;
    let mut spaces: u16 = 0;
    let mut spaces2: u16 = 0;

    print!("\x1B[2J\x1B[1;1H");
	enable_raw_mode()?;
	execute!(stdout, Hide).unwrap();

	draw_ground()?;

	loop {		
		if poll(Duration::from_millis(120))? {
            let event = read()?;
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }

				draw_ground()?;

                if key.code == KeyCode::Char('d') {
                    x += 1;
                    velocityx += 2;
                } else if key.code == KeyCode::Char('a') {
                    if x > 0 {
                    	x -= 1; 
                    };
                    velocityx -= 2;
                }

                if key.code == KeyCode::Char('s') {
                    if y + 1 == 3 {
                        y += 1;
                    }
                    velocityy += 2;
                } else if key.code == KeyCode::Char('w') && y == (groundy - 1) {
                  	y -= 3; 
                  	velocityy -= 2;
                }             
            }         
        }

		if x > groundx {
            if y < 50 { y += 1; } 
        } else if y < (groundy - 1) {
            velocityy += 1;
            y += 1;
        } else {
            y = groundy - 1;
            velocityy = 0;  
        }

		x = (x as i16 + velocityx).max(0) as u16;
		y = (y as i16 + velocityy).max(0) as u16;

        if x != spaces || y != spaces2 {
           execute!(stdout, MoveTo(spaces.max(0) as u16, spaces2.max(0) as u16), Print(" "))?;
           execute!(stdout, MoveTo(x.max(0) as u16, y.max(0) as u16), Print("i"))?;
           stdout.flush()?;             
           spaces = x; 
           spaces2 = y; 
      }        

		velocityx = 0;
      
	}
	disable_raw_mode()?;
	execute!(stdout, Show).unwrap();
	Ok(())
}
