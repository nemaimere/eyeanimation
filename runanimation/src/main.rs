use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal,
};
use std::io;
use std::{thread, time::Duration};

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?; // enabling raw mode makes the terminal not show input characters
                                  // and allows capturing special keys like arrows
    let mut isx = false; // tells us when the eye is in x state or O any other state

    loop {
        if let Event::Key(key_event) = event::read()? {
            // Only handle the initial key press, ignore repeats while key is held
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char('q') => {
                        println!("Quitting...");
                        break;
                    }
                    KeyCode::Up => {
                        animationeyeblink(isx);
                    }
                    KeyCode::Down => {
                        animationeyex(&mut isx);
                    }
                    KeyCode::Left => {
                        animationabsolutesolver(isx);
                    }
                    _ => {}
                }
            }
        }
    }

    terminal::disable_raw_mode()?; // Disable raw mode before exiting
    Ok(())
}

fn animationeyeblink(isx: bool) {
    if isx {
    } else {
        println!("blink start");

        //remove still eye
        //start animation blink                this is the time of the animation. substitute it
        thread::sleep(Duration::from_millis(500));
        //start still eye

        println!("blink fin");
    }
}

fn animationeyex(isx: &mut bool) {
    if *isx {
        *isx = false;

        println!("animation x-O start");
        //remove still x
        //start animation x-O                     this is the time of the animation. substitute it
        thread::sleep(Duration::from_millis(500));
        //start still O

        println!("animation x-O fin");
    } else {
        *isx = true;

        println!("animation O-x start");
        //remove still O
        //start animation O-x                     this is the time of the animation. substitute it
        thread::sleep(Duration::from_millis(500));
        //start still x

        println!("animation O-x fin");
    }
}

fn animationabsolutesolver(isx: bool) {
    if isx {
    } else {
        println!("animation absolute solver start");
        //remove still eye
        //start absolute solver                   this is the time of the animation. substitute it
        thread::sleep(Duration::from_millis(2000));
        //start still eye

        println!("animation absolute solver fin");
    }
}
