use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind};
use futures::StreamExt;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders},
    DefaultTerminal, Frame,
};
use tokio::time::Duration;
use tui_textarea::{Input, TextArea};

#[derive(Debug)]
pub struct App<'a> {
    ratios: Vec<f64>,
    inputs: Vec<TextArea<'a>>,
    should_quit: bool,

    current: usize,
}

impl<'a> App<'a> {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub fn new(values: Vec<f64>) -> Self {
        let inputs = values
            .iter()
            .map(|x| TextArea::new(vec![x.to_string()]))
            .collect();

        let first = values[0];
        let ratios = values.iter().map(|x| x / first).collect();
        Self {
            ratios,
            inputs,
            should_quit: false,
            current: 0,
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), std::io::Error> {
        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => {
                    self.update();
                    terminal.draw(|frame| self.draw(frame))?; },
                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }
        }
        Ok(())
    }

    fn current_valid(&self) -> bool {
        self.inputs[self.current].lines()[0].parse::<f64>().is_ok()
    }

    fn get_value(&self, index: usize) -> f64 {
        self.inputs[index].lines()[0].parse::<f64>().unwrap()
    }
    fn set_value(&mut self, index: usize, value: f64) {
        self.inputs[index] = TextArea::new(vec![value.to_string()])
    }

    fn update(&mut self) {
        let current_valid = self.current_valid();

        if current_valid {
            let base_value = self.get_value(self.current) / self.ratios[self.current];
            for index in 0..self.ratios.len() {
                if index != self.current {
                    self.set_value(index, self.ratios[index] * base_value)
                }
            }
        }

        for (index, textarea) in self.inputs.iter_mut().enumerate() {
            match index == self.current {
                true if current_valid => style_focussed(textarea),
                true => style_invalid(textarea),
                false => style_unfocussed(textarea),
            }
        }
    }

    fn increment_focus(&mut self) {
        if self.current_valid() {
            self.current = (self.current + 1) % self.inputs.len()
        }
    }

    fn build_layout(&self, body: Rect) -> Vec<Rect> {
        Layout::horizontal(vec![Constraint::Fill(1); self.inputs.len()])
            .split(body)
            .to_vec()
    }

    fn draw(&self, frame: &mut Frame) {
        let divs = self.build_layout(frame.area());
        for (index, div) in divs.into_iter().enumerate() {
            frame.render_widget(&self.inputs[index], div);
        }
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                ..
            }) => self.should_quit = true,
            Event::Key(KeyEvent {
                code: KeyCode::Tab,
                kind: KeyEventKind::Press,
                ..
            }) => self.increment_focus(),
            _ => {
                self.inputs[self.current].input(Input::from(event.clone()));
            }
        }
    }
}
fn style_focussed(textarea: &mut TextArea) {
    let style = Style::default().underlined();
    textarea.set_cursor_style(style);
    textarea.set_cursor_line_style(Style::default());
    textarea.set_block(
        Block::default()
            .border_style(Color::LightGreen)
            .borders(Borders::ALL),
    );
}

fn style_invalid(textarea: &mut TextArea) {
    let style = Style::default().underlined();
    textarea.set_cursor_style(style);
    textarea.set_cursor_line_style(Style::default());
    textarea.set_block(
        Block::default()
            .border_style(Color::Red)
            .borders(Borders::ALL)
            .title("Not a valid number"),
    );
}

fn style_unfocussed(textarea: &mut TextArea) {
    textarea.set_cursor_style(Style::default());
    textarea.set_cursor_line_style(Style::default());
    textarea.set_block(Block::default().borders(Borders::ALL));
}
