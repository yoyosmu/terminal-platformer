use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, size}, style::Print, cursor::{MoveTo, Hide, Show}};
use std::io::{self};
use std::time::Duration;

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
	
	let (width, height) = size()?;	
	let ground_y = height.saturating_sub(20);
	let ground_width = width;
    let mut stdout = io::stdout(); 
	
    enable_raw_mode()?;
	execute!(stdout, Hide).unwrap();
	execute!(io::stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All), MoveTo(0, 0))?; 
	draw_ground(&mut stdout, ground_y, ground_width)?;
   		
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
            player.x = player.x.saturating_add(1);
        }
        if left {
            player.x = player.x.saturating_sub(1);
        }

        let on_ground = player.y >= ground_y.saturating_sub(1);

        if up && on_ground {
            player.velocity_y = -3;
            player.y = player.y.saturating_add(1);
        }

        if down {
            player.velocity_y = player.velocity_y.saturating_add(1);
        }

        player.velocity_y = player.velocity_y.saturating_add(1);

        player.y = (player.y as i16 + player.velocity_y).max(0) as u16;

        if player.y >= ground_y.saturating_sub(1) {
            player.y = ground_y.saturating_sub(1);
            player.velocity_y = 0;
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

fn draw_ground(stdout: &mut io::Stdout, ground_y: u16, ground_width: u16) -> io::Result<()> {
    execute!(stdout, MoveTo(0, ground_y))?;
    for _ in 0..ground_width {
        execute!(stdout, Print('▔'))?;
    }
    Ok(())
}
