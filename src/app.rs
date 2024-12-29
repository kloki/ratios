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

use crate::value::Value;

#[derive(Debug)]
pub struct Item<'a> {
    ratio: f64,
    title: Option<String>,
    textarea: TextArea<'a>,
}

impl<'a> Item<'a> {
    pub fn is_valid(&self) -> bool {
        self.textarea.lines()[0].parse::<f64>().is_ok()
    }
    fn get_value(&self) -> f64 {
        self.textarea.lines()[0].parse::<f64>().unwrap()
    }
    fn set_new_value(&mut self, base_value: f64) {
        self.textarea = TextArea::new(vec![(base_value * self.ratio).to_string()])
    }
    fn get_base_value(&mut self) -> f64 {
        self.get_value() / self.ratio
    }

    fn style_focussed(&mut self) {
        let style = Style::default().underlined();
        self.textarea.set_cursor_style(style);
        self.textarea.set_cursor_line_style(Style::default());
        self.textarea.set_block(
            Block::default()
                .border_style(Color::LightGreen)
                .borders(Borders::ALL)
                .title(self.title.clone().unwrap_or_default()),
        );
    }

    fn style_invalid(&mut self) {
        let style = Style::default().underlined();
        self.textarea.set_cursor_style(style);
        self.textarea.set_cursor_line_style(Style::default());
        self.textarea.set_block(
            Block::default()
                .border_style(Color::Red)
                .borders(Borders::ALL)
                .title("Not a valid number"),
        );
    }

    fn style_unfocussed(&mut self) {
        self.textarea.set_cursor_style(Style::default());
        self.textarea.set_cursor_line_style(Style::default());
        self.textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(self.title.clone().unwrap_or_default()),
        );
    }
}

#[derive(Debug)]
pub struct App<'a> {
    items: Vec<Item<'a>>,
    should_quit: bool,
    current: usize,
}

impl<'a> App<'a> {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub fn new(values: Vec<Value>) -> Self {
        let first = values[0].value;
        let items = values
            .iter()
            .map(|x| Item {
                title: x.name.clone(),
                ratio: x.value / first,
                textarea: TextArea::new(vec![x.value.to_string()]),
            })
            .collect();
        Self {
            items,
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
        self.items[self.current].is_valid()
    }

    fn update(&mut self) {
        let current_valid = self.current_valid();

        if current_valid {
            let base_value = self.items[self.current].get_base_value();
            for index in 0..self.items.len() {
                if index != self.current {
                    self.items[index].set_new_value(base_value);
                }
            }
        }

        for (index, item) in self.items.iter_mut().enumerate() {
            match index == self.current {
                true if current_valid => item.style_focussed(),
                true => item.style_invalid(),
                false => item.style_unfocussed(),
            }
        }
    }

    fn increment_focus(&mut self) {
        if self.current_valid() {
            self.current = (self.current + 1) % self.items.len()
        }
    }

    fn build_layout(&self, body: Rect) -> Vec<Rect> {
        Layout::horizontal(vec![Constraint::Fill(1); self.items.len()])
            .split(body)
            .to_vec()
    }

    fn draw(&self, frame: &mut Frame) {
        let divs = self.build_layout(frame.area());
        for (index, div) in divs.into_iter().enumerate() {
            frame.render_widget(&self.items[index].textarea, div);
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
                self.items[self.current]
                    .textarea
                    .input(Input::from(event.clone()));
            }
        }
    }
}
