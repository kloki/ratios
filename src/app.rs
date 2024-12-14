use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use futures::StreamExt;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::Widget,
    DefaultTerminal, Frame,
};
use tokio::time::Duration;

#[derive(Debug)]
pub struct App {
    values: Vec<f64>,
    should_quit: bool,

    current: usize,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub fn new(values: Vec<f64>) -> Self {
        Self {
            values,
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
                _ = interval.tick() => { terminal.draw(|frame| self.draw(frame))?; },
                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }
        }
        Ok(())
    }

    fn build_layout(&self, body: Rect) -> Vec<Rect> {
        Layout::horizontal(vec![Constraint::Fill(1); self.values.len()])
            .split(body)
            .to_vec()
    }
    fn draw(&self, frame: &mut Frame) {
        let divs = self.build_layout(frame.area());
        for (div, value) in divs.into_iter().zip(self.values.clone()) {
            frame.render_widget(value.to_string(), div);
        }
    }

    fn handle_event(&mut self, event: &Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                    _ => {}
                }
            }
        }
    }
}
