use crate::collector::Function;
use std::fs;
use std::io;

#[allow(unused_imports, dead_code)]
use ratatui::backend::Backend;

#[allow(unused_imports, dead_code)]
use ratatui::{
    self,
    crossterm::{
        self,
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{self},
    },
    layout::*,
    widgets::*,
    Frame, Terminal,
};

pub enum State {
    FuncsList(Vec<Function>),
    DirsList(Vec<String>),
    FilesList(Vec<String>),
    TypesList(Vec<String>),
}

pub fn ui<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut inp_buf = String::new();

    let dirs = fs::read_dir("/usr/include")?;
    let files = dirs.map(|entry| entry.unwrap().path()).collect::<Vec<_>>();
    let mut list_state = ListState::default();

    loop {
        terminal.draw(|frame| {
            Layout::default()
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(frame.area());
            frame.render_widget(
                Paragraph::new(&*inp_buf).block(Block::default().title("Greeting")),
                frame.area(),
            );
            frame.render_stateful_widget(
                List::new(files.iter().map(|f| f.to_str().unwrap())),
                frame.area(),
                &mut list_state,
            );
        })?;
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('c') && key.modifiers == event::KeyModifiers::CONTROL {
                break;
            }
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up => {
                        list_state.select_next();
                    }
                    KeyCode::Down => {
                        list_state.select_previous();
                    }
                    KeyCode::Enter => {
                        break;
                    }
                    KeyCode::Backspace => {
                        inp_buf.pop();
                    }
                    _ => {}
                }
                if key.kind == KeyEventKind::Press {
                    let ch = key.code.to_string().chars().next().unwrap();
                    if ch.is_ascii_alphanumeric() {
                        inp_buf.push(ch);
                    }
                }
            }
        }
    }
    Ok(())
}
