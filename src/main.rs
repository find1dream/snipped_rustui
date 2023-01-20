use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, List, ListItem, Paragraph, BorderType, Borders},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

enum InputMode {
    Normal,
    Editing
}

struct App {
    input: String,
    input_mode: InputMode,
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => app.input_mode = InputMode::Editing,
                    KeyCode::Char('q') => {return Ok(());}
                    _ => {}

                }
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {app.messages.push(app.input.drain(..).collect());}
                    KeyCode::Char(c) => {app.input.push(c);}
                    KeyCode::Backspace => {app.input.pop();}
                    KeyCode::Esc => {app.input_mode = InputMode::Normal;}
                    _ => {}
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();

    // Surrounding block
    let block = Block::default();
    //    .title("Code Snippet Sync App")
    //    .title_alignment(Alignment::Center);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(40),Constraint::Length(2), Constraint::Percentage(58)].as_ref())
        .split(f.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow)
        })
        .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(input, left_chunks[0]);
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => {
            f.set_cursor(
                left_chunks[0].x + app.input.width() as u16 + 1,
                left_chunks[0].y + 1
            )
        }
    }

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(messages).block(Block::default().borders(Borders::ALL).title("List"));
    f.render_widget(messages, left_chunks[1]);

    // Bottom right block with styled left and right border
    let block = Block::default()
        .title("Code Snippet")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[2]);
}
