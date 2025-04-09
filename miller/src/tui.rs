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

pub enum Event {
    Input(crossterm::event::KeyEvent),
}

pub struct App {
    pub exit: bool,
}

impl App {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        rx: mpsc::Receiver<Event>,
    ) -> io::Result<()> {
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

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
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

pub fn input_fetcher(tx: mpsc::Sender<Event>) {
    loop {
        // unwraps, bc what could go wrong?
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
            _ => (),
        }
    }
}
