use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, size}, style::Print, cursor::{MoveTo, Hide, Show}};
use std::io::{self, Write};
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout(); 
    let (groundx, groundy) = draw_ground()?; 

    print!("\x1B[2J\x1B[1;1H");
    enable_raw_mode()?;
    execute!(stdout, Hide).unwrap();

    draw_ground()?;
	
    run_char(stdout, groundx, groundy)?;

    disable_raw_mode()?;
    execute!(io::stdout(), Show).unwrap();
    Ok(())
}

fn draw_ground() -> io::Result<(u16, u16)> {
    let mut stdout = io::stdout();
    let groundy: u16 = 12; 
    let (width, _) = size()?;
    
    for groundx in 0..width {
        execute!(stdout, MoveTo(groundx, groundy), Print("▔"))?;
    }
    
    let groundx: u16 = width;
    Ok((groundx, groundy))
}

fn run_char(mut stdout: io::Stdout, groundx: u16, groundy: u16, ) -> io::Result<()> {    
	let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut velocityx: i16 = 0;
    let mut velocityy: i16 = 0;
    let mut spaces: u16 = 0;
    let mut spaces2: u16 = 0;

    loop {
    	let mut right = false;
    	let mut left = false;
    	let mut up = false;
    	let mut down = false;	
    			        	
        if poll(Duration::from_millis(120))? {
            if let Event::Key(key) = read()? {
                if key.kind == KeyEventKind::Press {
                	match key.code {
                    	KeyCode::Char('d') => right = true,
                        KeyCode::Char('a') => left = true,
                        KeyCode::Char('w') => up = true,
                        KeyCode::Char('s') => down = true,
                        KeyCode::Char('q') => return Ok(()),
                        _ => {}
                    }
                }                    

                draw_ground()?;



                if right {
                    x += 1;
                    velocityx += 2;
                } else if left {
                    if x > 0 { x -= 1; };
                    velocityx -= 2;
                }

                if down {
                    if y + 1 == 3 { y += 1; }
                    velocityy += 2;
                } else if up && y == (groundy - 1) {
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

        draw_ground()?;


        x = (x as i16 + velocityx).max(0) as u16;
        y = (y as i16 + velocityy).max(0) as u16;

        if x != spaces || y != spaces2 {
            execute!(stdout, MoveTo(spaces.max(0), spaces2.max(0)), Print(" "))?;
            execute!(stdout, MoveTo(x.max(0), y.max(0)), Print("i"))?;
            stdout.flush()?;            
            spaces = x; 
            spaces2 = y; 
        }      

        velocityx = 0;
    }
    Ok(())
}
