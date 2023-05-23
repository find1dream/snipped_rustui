use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
};
use std::io;
use std::io::{stdout, Write};
use tui::{
    backend::{Backend},
    Terminal,
};
use walkdir::WalkDir;
use std::ffi::OsStr;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::thread;
use arboard::Clipboard;

use crate::view::{App, ui};
use crate::key::InputMode;
use crate::note::Note;
use crate::git::{git_add_all, git_commit, git_pull, git_push};


fn update_input_buffer(app: &mut App, buffer: &mut String, key: &KeyEvent, clipboard: &mut Clipboard) {
    match key {
        KeyEvent {code: KeyCode::Char('u'), modifiers: KeyModifiers::CONTROL, kind: _pressed, state: _none} => {
            // clear buffer
            buffer.clear();
        }
        KeyEvent {code: KeyCode::Enter, modifiers: KeyModifiers::NONE, kind: _pressed, state: _none} => {
        //      let input:String = app.input.drain(..).collect();
        //      app.list.items.push(Note{title: input.clone(), language:"python".to_string(), contents: input.clone()} );
        }
        KeyEvent {code: KeyCode::Char(c), modifiers: KeyModifiers::NONE, kind: _pressed, state: _none} => {
            buffer.push(*c);
        }
        KeyEvent {code: KeyCode::Char(c), modifiers: KeyModifiers::SHIFT, kind: _pressed, state: _none} => {
            buffer.push(*c);
        }
        KeyEvent {code: KeyCode::Backspace, modifiers: KeyModifiers::NONE, kind: _pressed, state: _none} => {
            buffer.pop();
        }
        KeyEvent {code: KeyCode::Tab, modifiers: KeyModifiers::NONE, kind: _pressed, state: _none} => {app.input_mode = app.input_mode.next_mode();}
        KeyEvent {code: KeyCode::Esc, modifiers: KeyModifiers::NONE, kind: _pressed, state: _none} => {
            app.input_mode = InputMode::Normal;
        }
        KeyEvent {code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, kind: _pressed, state: _none} => {
            // copy
            clipboard.set_text(buffer.as_str()).unwrap();
        }
        KeyEvent {code: KeyCode::Char('v'), modifiers: KeyModifiers::CONTROL, kind: _pressed, state: _none} => {
            // paste
            buffer.clear();
            buffer.push_str(clipboard.get_text().unwrap().as_str());
        }
        _ => {}
    }
}

fn refresh_ui() {
    stdout().flush().unwrap();
}

fn load_all_markdown(base_url: &str) -> Vec<Note> {
    let mut vecs: Vec<Note> = vec![];
    for entry in WalkDir::new(base_url) {
        let entry = entry.unwrap();
        let file_name = entry.path();
        let file_extension = file_name.extension().and_then(OsStr::to_str);
        if  file_extension == Some("md") {
            
            let note = match Note::load(&file_name.display().to_string()){
                Ok(note)  => note,
                Err(_) => panic!("load error")
            };
            vecs.push(note);
        }
    }
    vecs
}

#[allow(unused_variables)]
pub fn run_app<B: Backend>(base_url: &str, terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    // init git
    println!("");
    println!("  Sync with remote server...");
    git_pull(base_url);

    // init clipboard
    let mut clipboard = Clipboard::new().expect("cannot open clipboard");

    let matcher = SkimMatcherV2::default();
    let mut note = Note::new(base_url, "", "", "");
    app.list.items = load_all_markdown(base_url);
    let mut search_text = String::new();
    loop {
        terminal.draw(|f| ui(f, &mut app, &note, &search_text))?;

        if let Event::Key(key) = event::read()? {
            // adjust mode
            match app.input_mode {
                InputMode::Normal => match key {
                    KeyEvent {code: KeyCode::Enter, modifiers: KeyModifiers::NONE, kind: _pressed, state: _none} => {
                        app.input_mode = InputMode::EditingTitle;
                    }
                    KeyEvent {code: KeyCode::Tab, modifiers: KeyModifiers::NONE, kind: _pressed, state: _none} => {app.input_mode = app.input_mode.next_mode();}
                    KeyEvent {code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, kind: _pressed, state: _none} => return Ok(()),
                    KeyEvent {code: KeyCode::Char('n'), modifiers: KeyModifiers::CONTROL, kind: _pressed, state: _none} => {
                        // create new note
                        note = Note::new(base_url, "", "", "");
                        app.list.items.push(note.clone()); 
                        app.list.set_selected_num(app.list.items.len() - 1); // select last new item
                        refresh_ui();
                    }
                    KeyEvent {code: KeyCode::Char('d'), modifiers: KeyModifiers::CONTROL, kind: _pressed, state: _none} => {
                        // delete note
                        if let Some(num) = app.list.get_selected_num() {
                            app.list.items[num].delete().expect("deleted file do not exist");
                            app.list.delete(num);
                            match num > 0 {
                                true => {
                                    app.list.set_selected_num(num - 1);
                                    note = app.list.items[num - 1].clone();
                                } // select last new item
                                false => {
                                    app.list.set_selected_num(0);
                                    note = Note::new(base_url, "", "", "");
                                }
                            }
                        }
                        refresh_ui();
                    }
                    KeyEvent {code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, kind: _pressed, state: _none} => {
                        match app.list.get_selected_num() {
                            Some(index) => {
                                clipboard.set_text(note.contents.as_str()).unwrap();
                            },
                            None => {}
                        }
                    },
                    KeyEvent {code: KeyCode::Char('s'), modifiers: KeyModifiers::CONTROL, kind: _pressed, state: _none} => {
                        // save
                        match app.list.get_selected_num() {
                            Some(index) => {
                                match app.list.items[index].delete() {
									Ok(_) => {},
									Err(_) => {}
								};//.expect("deleted file do not exist");
                                app.list.items[index] = Note::new(base_url, &note.language, &note.title, &note.contents);
                                if let Ok(path) = app.list.items[index].save() {
                                    let commit_contents = format!("update: {}, {}", note.language, note.title);
                                    let path = base_url.to_string();
                                    thread::spawn(move || {
                                        git_add_all(&path);
                                        git_commit(&path, &commit_contents);
                                        git_push(&path);

                                    });
                                }
                            },
                            None => {}
                        }
                        refresh_ui();
                    }
                    KeyEvent {code: KeyCode::Left, modifiers: KeyModifiers::NONE, kind: _pressed, state: _none } => {
                        app.list.unselect();
                    }
                    KeyEvent {code: KeyCode::Down, modifiers: KeyModifiers::NONE, kind: _pressed, state: _none} => {
                        app.list.next();
                        match app.list.get_selected_num() {
                            Some(index) => note = app.list.items[index].clone(),
                            None => {}
                        }
                    }
                    KeyEvent {code: KeyCode::Up, modifiers: KeyModifiers::NONE, kind: _pressed, state: _none} => {
                        app.list.previous();
                        match app.list.get_selected_num() {
                            Some(index) => note = app.list.items[index].clone(),
                            None => {}
                        }
                    }
                    _ => {}

                }
                InputMode::EditingSearch => {
                    update_input_buffer(&mut app, &mut search_text, &key, &mut clipboard);
                    match key {
                        KeyEvent {code: KeyCode::Enter, modifiers: KeyModifiers::NONE, kind: _pressed, state: _none} => {
                            app.list.items = app.list.items.iter().enumerate().filter_map(|(i, x)| 
                                match matcher.fuzzy_indices(&x.title, &search_text) {
                                    Some((score, indices)) => Some(app.list.items[i].clone()),
                                    None => None
                                }
                            ).collect();
                        }
                        _ => {}
                    }
                    if search_text.len() == 0 {
                        app.list.items = load_all_markdown(base_url);
                    }
                }
                InputMode::EditingTitle => update_input_buffer(&mut app, &mut note.title, &key, &mut clipboard),
                InputMode::EditingLanguage => update_input_buffer(&mut app, &mut note.language, &key, &mut clipboard),
                InputMode::EditingCode => {
                    update_input_buffer(&mut app, &mut note.contents, &key, &mut clipboard);
                    match key {
                        KeyEvent {code: KeyCode::Enter, modifiers: KeyModifiers::NONE, kind: _pressed, state: _none} => {
                            note.contents.push('\n');
                        }
                        _ => {}
                    }
                }
            }

        }
    }
}


