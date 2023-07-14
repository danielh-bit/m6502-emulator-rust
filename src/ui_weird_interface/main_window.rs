use std::{io, error::Error, sync::mpsc, thread, time::{Duration, Instant}};

use crossterm::{terminal::{self, EnterAlternateScreen}, ExecutableCommand, cursor::Hide, event::{KeyCode, self, Event}};

use super::{frame::{self}, render};

pub fn show_ui() -> Result<(), Box<dyn Error>> {
    // println!("Hello UWU");
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //render loop in separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut instant = Instant::now();
    //graphics loop
    'ui_loop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        let curr_frame = frame::new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc => {
                        break 'ui_loop;
                    }
                    _ => {}
                }
            }
        }

        let _ = render_tx.send(curr_frame.clone());
        thread::sleep(Duration::from_millis(1));
    }

    drop(render_tx);
    render_handle.join().unwrap();

    Ok(())
}