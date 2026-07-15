use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, size}, style::Print, cursor::{MoveTo, Hide, Show}};
use std::io::{self};
use std::time::Duration;
use std::collections::HashSet;

fn main() -> io::Result<()> {
	struct Player {
	    x: u16,
	    y: u16,
	    velocity_y: i16,
	    prev_x: u16,
	    prev_y: u16,
	    main_char: char,
	}
	let mut player = Player {
	    x: 0,
	    y: 0,
	    velocity_y: 0,
	    prev_x: 0,
	    prev_y: 0,
	    main_char: 'i',
	};
	
	let mut occupied: HashSet<(u16, u16)> = HashSet::new();
	let (width, height) = size()?;	
	let ground_y = height.saturating_sub(20);
	let ground_width = width;
    let mut stdout = io::stdout(); 
	
    enable_raw_mode()?;
	execute!(stdout, Hide).unwrap();
	execute!(io::stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All), MoveTo(0, 0), Print(player.main_char))?; 
	draw_ground(&mut stdout, ground_y, ground_width, &mut occupied)?;
   	draw_terrain(&mut stdout, (width as f32 * 0.75) as u16, ground_y.saturating_sub(1), &mut occupied)?;
   	
    loop {
    	let mut right = false;
    	let mut left = false;
   		let mut up = false;
  		let mut down = false;		
		let max_x = width.saturating_sub(1);
	    let max_y = height.saturating_sub(1);
		if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = read()? {
                if key.kind == KeyEventKind::Press {
                	match key.code {
                    	KeyCode::Char('d') => right = true,
                        KeyCode::Char('a') => left = true,
                        KeyCode::Char('w') => up = true,
                        KeyCode::Char('s') => down = true,
                        KeyCode::Char('q') => {execute!(io::stdout(), Show).unwrap(); disable_raw_mode()?; return Ok(()); },
                        _ => {}
                    }
                }             
            }
    	}
        if right {
            let next_x = player.x.saturating_add(1);
            if !occupied.contains(&(next_x, player.y)) {
                player.x = next_x;
            }
        }
        if left {
            let next_x = player.x.saturating_sub(1);
            if !occupied.contains(&(next_x, player.y)) {
                player.x = next_x;
            }
        }
        
        let on_ground = occupied.contains(&(player.x, player.y + 1));
        
        if up && on_ground {
            player.velocity_y = -3;
            player.y = player.y.saturating_add(1);
        }
        if down {
            player.velocity_y = player.velocity_y.saturating_add(1);
        }
        
        player.velocity_y = player.velocity_y.saturating_add(1);

        let fall_steps = player.velocity_y.abs();
        let fall_dir = player.velocity_y.signum();
        for _ in 0..fall_steps {
            let next_y = player.y as i16 + fall_dir;
            if next_y < 0 {
                break;
            }
            let next_y = next_y as u16;

            if occupied.contains(&(player.x, next_y)) {
                player.velocity_y = 0;
                break;
            }

            player.y = next_y;
        }

        if player.y == 0 && player.velocity_y < 0 {
            player.velocity_y = 0;
        }
        
        player.x = player.x.min(max_x);
        player.y = player.y.min(max_y);
        
        if player.x != player.prev_x || player.y != player.prev_y {
            execute!(io::stdout(), MoveTo(player.prev_x, player.prev_y), Print(" "))?;
            execute!(io::stdout(), MoveTo(player.x, player.y), Print(player.main_char))?;
            player.prev_y = player.y;
            player.prev_x = player.x;
        }
    }
}
fn draw_ground(stdout: &mut io::Stdout, ground_y: u16, ground_width: u16, occupied: &mut HashSet<(u16, u16)>) -> io::Result<()> {
    execute!(stdout, MoveTo(0, ground_y))?;
    for x in 0..ground_width {
        execute!(stdout, Print('▔'))?;
        occupied.insert((x, ground_y));
    }
    Ok(())
}

fn draw_terrain(stdout: &mut io::Stdout, x: u16, y: u16, occupied: &mut HashSet<(u16, u16)>) -> io::Result<()> {
    execute!(stdout, MoveTo(x, y), Print("🮈▔▔▔▔▔▔▔▔▔▔▍"))?;
    for i in 0..12 {
        occupied.insert((x + i, y));
    }
    for dy in 1..=3 {
        occupied.insert((x, y + dy));
        occupied.insert((x + 11, y + dy));
    }
    Ok(())
}
