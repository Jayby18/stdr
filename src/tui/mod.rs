#[macro_export]
/// Set up `ratatui` terminal interface with `crossterm` backend and `mpsc` event handling
///
/// # Usage
/// 
/// ## Add dependencies to crate
/// `cargo add ratatui crossterm thread mpsc`
/// 
/// ## Define `Event` enum
/// ```
/// enum Event<I> {
///     Input(I),
///     Tick
/// }
/// ```
/// 
/// ## Example
/// ```
/// fn main -> Result<(), std::error::Error> {
///     let (mut terminal, rx) = stdr::setup_terminal!();
///     loop {
///         terminal.draw(...);
///         ...
///     }
/// 
///     Ok(())
/// }
/// ```

macro_rules! setup_terminal {
    () => {{
        // Imports
        use std::{
            thread,
            time::{Duration, Instant},
            sync::mpsc,
        };
        use ratatui::{
            backend::CrosstermBackend,
            Terminal,
        };
        use crossterm::{
            event::{self, EnableMouseCapture, Event as CEvent},
            execute,
            terminal::{enable_raw_mode, EnterAlternateScreen},
        };

        // Set up terminal
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        // User event handler
        // enum Event<I> {
        //     Input(I),
        //     Tick,
        // }

        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(200);
        thread::spawn(move | | {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(| | Duration::from_secs(0));

                if event::poll(timeout).expect("poll works") {
                    if let CEvent::Key(key) = event::read().expect("can read events") {
                        tx.send(Event::Input(key)).expect("can send events");
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if let Ok(_) = tx.send(Event::Tick) {
                        last_tick = Instant::now();
                    }
                }
            }
        });

        (terminal, rx)
    }};
}

#[macro_export]
/// Restore `ratatui`/`crossterm` terminal
macro_rules! restore_terminal {
    ( $terminal:expr ) => {
        // Imports
        use crossterm::{
            event::DisableMouseCapture,
            terminal::{disable_raw_mode, LeaveAlternateScreen},
            execute,
        };

        // Restore terminal
        disable_raw_mode()?;
        execute!($terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
        $terminal.show_cursor()?;
    };
}
