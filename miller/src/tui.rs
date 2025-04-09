use std::{io, sync::mpsc};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        self,
        event::{KeyCode, KeyEventKind},
    },
    layout::Layout,
    style::Color as Colour,
    widgets::Widget,
};

enum Event {
    Input(crossterm::event::KeyEvent),
}

pub struct App {
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal, rx: mpsc::Receiver<Event>) -> io::Result<()> {
        while !self.exit {
            // Render frame.
            terminal.draw(|frame| self.draw(frame))?;

            // Event handler
            // unwraps, bc what could go wrong?
            match rx.recv().unwrap() {
                Event::Input(key_event) => self.handle_key_event(key_event)?,
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.exit = true;
        }

        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout: Layout;
    }
}
