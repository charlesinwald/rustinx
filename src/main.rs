use clap::{App, Arg};
use crossterm::{
    cursor::{Hide, Show},
    event::{self, KeyCode},
    execute,
    terminal::{self, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use nix::libc::geteuid;
use std::io::{self, BufRead, BufReader, Write};
use std::sync::mpsc::{self, Receiver};
use std::{collections::VecDeque, fs::File};
use std::{error::Error, process, thread};
use tokio::time::Duration;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Paragraph},
};
use tui::{widgets::Borders, Terminal};

// Function to monitor the Nginx error log file and send new lines to the UI thread
fn monitor_nginx_error_log(path: &str, tx: mpsc::Sender<String>) -> std::io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(0) => {
                // No new line, wait before trying again
                thread::sleep(Duration::from_millis(500));
            }
            Ok(_) => {
                tx.send(line.clone()).unwrap(); // Send the new line to the UI thread
                line.clear(); // Clear the line buffer for the next read
            }
            Err(e) => return Err(e),
        }
    }
}

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

    let (tx, rx): (mpsc::Sender<String>, Receiver<String>) = mpsc::channel();
    // Start the log monitoring thread
    let log_path = "/var/log/nginx/error.log"; // Adjust the path to your Nginx error log file
    thread::spawn(move || {
        monitor_nginx_error_log(log_path, tx)
            .unwrap_or_else(|e| eprintln!("Log monitoring error: {}", e));
    });
    let mut log_lines: VecDeque<String> = VecDeque::with_capacity(10);

    let server_name = matches.value_of("server-name").unwrap(); // Safely unwrap because it's required

    let mut stdout = io::stdout().lock();

    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let nginxv_stderr = std::process::Command::new("nginx")
        .arg("-v")
        .output()?
        .stderr;
    let nginx_version = String::from_utf8_lossy(&(nginxv_stderr));

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
        let output = std::process::Command::new("nginx").arg("-t").output()?;
        let combined_output = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr) // Capture stderr as well
        );

        let pid_out = std::process::Command::new("cat")
            .arg("/run/nginx.pid")
            .output()?
            .stdout;
        let pid = String::from_utf8_lossy(&(pid_out));

        let output_str = format!(
            "Monitoring server: {}\nNginx version: {}PID: {}\n{}",
            server_name, nginx_version, pid, combined_output
        );

        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical) // Change to Horizontal if you prefer side-by-side
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(50), // First box takes half the space
                        Constraint::Percentage(50), // Second box takes the remaining space
                    ]
                    .as_ref(),
                )
                .split(size);

            let nginx_status_block = Block::default()
                .title("Nginx Configuration Status")
                .borders(Borders::ALL);
            let nginx_status_paragraph =
                Paragraph::new(output_str.clone()).block(nginx_status_block);
            f.render_widget(nginx_status_paragraph, chunks[0]); // Render in the first section
                                                                // Attempt to receive new log entries without blocking
            while let Ok(log_entry) = rx.try_recv() {
                if log_lines.len() == 10 {
                    log_lines.pop_front(); // Remove the oldest log line if we have 10 lines
                }
                log_lines.push_back(log_entry); // Add the new line
            }

            let log_display = log_lines
                .iter()
                .cloned()
                .collect::<Vec<String>>()
                .join("\n");

            let other_info_block = Block::default().title("Logs").borders(Borders::ALL);
            let other_info_paragraph = Paragraph::new(log_display).block(other_info_block);
            f.render_widget(other_info_paragraph, chunks[1]); // Render in the second section
        })?;

        // Sleep for a fixed interval before the next update.
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}
