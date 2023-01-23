use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{CrosstermBackend},
    Terminal,
};

mod app;
mod view;
mod key;
mod note;
mod git;
mod env;


fn main() -> Result<(), Box<dyn Error>> {
    // load env data
    let env_data = match env::EnvData::check_env_file_exists() {
        true  => env::EnvData::load().unwrap(),
        false => env::EnvData::new().unwrap()
    };

    let git_path = env_data.get_git_folder_path();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = view::App::default();

    let res = app::run_app(&git_path, &mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
