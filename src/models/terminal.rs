use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Padding, Paragraph, Wrap,
    },
};
use std::{
    error::Error,
    io::{stdout, Stdout},
    ops::ControlFlow,
    time::Duration,
};



type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;

pub trait TUI{
    fn setup() -> Result<Terminal>;
    fn restore_terminal(self) -> Result<()>;
    fn run(&self) -> Result<()>;
    fn handle_events() -> Result<ControlFlow<()>>;
    fn ui(frame: Frame);
    fn calculate_layout(area: Rect) -> (Rect, Vec<Vec<Rect>>);
}

impl TUI for Terminal{
    fn setup() -> Result<Terminal>{
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(terminal)
    }
    fn restore_terminal(mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(self.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
    fn handle_events() -> Result<ControlFlow<()>> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(ControlFlow::Break(()));
                }
            }
        }
        Ok(ControlFlow::Continue(()))
    }
    fn run(&mut self) -> Result<()> {
        loop {
            self.draw(self.ui)?;
            if self.handle_events()?.is_break() {
                return Ok(());
            }
        }
    }
    fn ui(frame: &mut Frame) {
        let (title_area, layout) = calculate_layout(frame.size());

        render_title(frame, title_area);

        let paragraph = placeholder_paragraph();

        render_borders(&paragraph, Borders::ALL, frame, layout[0][0]);
        render_borders(&paragraph, Borders::NONE, frame, layout[0][1]);
        render_borders(&paragraph, Borders::LEFT, frame, layout[1][0]);
        render_borders(&paragraph, Borders::RIGHT, frame, layout[1][1]);
        render_borders(&paragraph, Borders::TOP, frame, layout[2][0]);
        render_borders(&paragraph, Borders::BOTTOM, frame, layout[2][1]);

        render_border_type(&paragraph, BorderType::Plain, frame, layout[3][0]);
        render_border_type(&paragraph, BorderType::Rounded, frame, layout[3][1]);
        render_border_type(&paragraph, BorderType::Double, frame, layout[4][0]);
        render_border_type(&paragraph, BorderType::Thick, frame, layout[4][1]);

        render_styled_block(&paragraph, frame, layout[5][0]);
        render_styled_borders(&paragraph, frame, layout[5][1]);
        render_styled_title(&paragraph, frame, layout[6][0]);
        render_styled_title_content(&paragraph, frame, layout[6][1]);
        render_multiple_titles(&paragraph, frame, layout[7][0]);
        render_multiple_title_positions(&paragraph, frame, layout[7][1]);
        render_padding(&paragraph, frame, layout[8][0]);
        render_nested_blocks(&paragraph, frame, layout[8][1]);
    }
    fn calculate_layout(area: Rect) -> (Rect, Vec<Vec<Rect>>) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Min(0)])
            .split(area);
        let title_area = layout[0];
        let main_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(4); 9])
            .split(layout[1])
            .iter()
            .map(|&area| {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(area)
                    .to_vec()
            })
            .collect_vec();
        (title_area, main_areas)
    }
}
