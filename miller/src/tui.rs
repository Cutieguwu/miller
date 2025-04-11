use std::{io, sync::mpsc};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        self,
        event::{KeyCode, KeyEventKind},
    },
    layout::{Constraint, Layout},
    style::Color as Colour,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
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

// impl on ref to avoid accidentally mutating the struct.
impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let [teams_area, main_area, instruction_area] =
            Layout::vertical([Constraint::Max(3), Constraint::Fill(1), Constraint::Max(1)])
                .areas(area);

        let [left_area, right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(main_area);

        let [upper_left_area, common_play_area, common_config_area] = Layout::vertical([
            Constraint::Percentage(60),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .areas(left_area);

        let [quicks_area, pattern_area] =
            Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
                .areas(upper_left_area);

        let [trends_area, graph_area, lower_right_area] = Layout::vertical([
            Constraint::Percentage(20),
            Constraint::Percentage(55),
            Constraint::Percentage(25),
        ])
        .areas(right_area);

        let [legend_area, _, right_lower_right_area] = Layout::horizontal([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .areas(lower_right_area);

        let [_, compare_area] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(right_lower_right_area);

        let teams_block = Block::bordered()
            .title(Line::from(" Teams "))
            .border_set(border::THICK);
        teams_block.render(teams_area, buf);

        let instructions = Line::from(vec![
            " ".into(),
            "Quit <q>".into(),
            " | ".into(),
            "Function <a>".into(),
            " | ".into(),
            "Function <b>".into(),
            " | ".into(),
            "Function <c>".into(),
            " | ".into(),
            "Function <d>".into(),
            " | ".into(),
            "Function <e>".into(),
            " | ".into(),
            "Function <f>".into(),
            " | ".into(),
            "Function <g>".into(),
        ]);
        instructions.render(instruction_area, buf);

        let quicks_block = Block::bordered()
            .title(" Quicks ")
            .border_set(border::THICK);
        quicks_block.render(quicks_area, buf);

        let pattern_block = Block::bordered()
            .title(" Pattern ")
            .border_set(border::THICK);
        pattern_block.render(pattern_area, buf);

        let common_play_block = Block::bordered()
            .title(" Most Freq. Play ")
            .border_set(border::THICK);
        common_play_block.render(common_play_area, buf);

        let common_config_block = Block::bordered()
            .title(" Most Freq. Configuration ")
            .border_set(border::THICK);
        common_config_block.render(common_config_area, buf);

        let trends_block = Block::bordered()
            .title(" Trends ")
            .border_set(border::THICK);
        trends_block.render(trends_area, buf);

        let graph_block = Block::bordered().title(" Graph ").border_set(border::THICK);
        graph_block.render(graph_area, buf);

        let legend_block = Block::bordered()
            .title(" Legend ")
            .border_set(border::THICK);
        legend_block.render(legend_area, buf);

        let compare_block = Block::bordered()
            .title(" Compare ")
            .border_set(border::THICK);
        compare_block.render(compare_area, buf);
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
