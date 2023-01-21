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

//#[allow(unused_must_use)]
//fn main() {
//
//    let env_data = match env::EnvData::check_env_file_exists() {
//        true  => env::EnvData::load().unwrap(),
//        false => env::EnvData::new().unwrap()
//    };
//
//    let git_path = env_data.get_git_folder_path();
//    let language = "python";
//    let title = "hello";
//    let contents = "print('hello world5')";
//    let note = note::Note::new(&git_path, 
//             language, 
//             title, 
//             contents);
//
//    if let Ok(path) = note.save() {
//        println!("{}", &git_path);
//        git::git_add_all(&git_path);
//        git::git_commit(&git_path, "push test3");
//        git::git_pull(&git_path);
//        git::git_push(&git_path);
//    }
//    
//}
