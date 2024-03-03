use clap::{App, Arg};
use crossterm::{
    cursor::{Hide, Show},
    event::{self, KeyCode},
    execute,
    terminal::{self, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use nix::libc::geteuid;
use std::io::{self, Write};
use std::{error::Error, process, thread};
use tokio::time::Duration;
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Paragraph},
};
use tui::{widgets::Borders, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    // Check if the current user is root
    if unsafe { geteuid() == 0 } {
        println!("Running with root privileges.");
    } else {
        eprintln!("This program needs to be run as root. Please use sudo.");
        process::exit(1); // Exit if not root
    }
    let matches = App::new("Nginx Monitor")
        .arg(
            Arg::new("server-name")
                .required(true)
                .help("Name of the nginx server to monitor"),
        )
        // Add other arguments as required
        .get_matches();

    let server_name = matches.value_of("server-name").unwrap(); // Safely unwrap because it's required

    let mut stdout = io::stdout().lock();

    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let output = std::process::Command::new("nginx").arg("-t").output()?;
    'mainloop: loop {
        // Check for keyboard events
        if event::poll(Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        // Quit the application
                        break 'mainloop;
                    }
                    _ => {} // Handle other keys here
                }
            }
        }
        let combined_output = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr) // Capture stderr as well
        );

        let output_str = format!(
            "Monitoring server: {}\nCommand output: {}",
            server_name, combined_output
        );

        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Nginx Configuration Status")
                .borders(Borders::ALL);
            let paragraph = Paragraph::new(output_str).block(block);
            f.render_widget(paragraph, size);
        })?;

        // Sleep for a fixed interval before the next update.
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}
