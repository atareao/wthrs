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

use itertools::Itertools;
use async_trait::async_trait;

use super::openmeteo::OpenMeteoClient;


type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;

struct APP<'a>{
    terminal: Terminal,
    omc: &'a OpenMeteoClient<'a>,
}


#[async_trait]
pub trait TUI{
    fn setup() -> Result<Terminal>;
    fn restore(&mut self) -> Result<()>;
    async fn run(&mut self, lon: String, lat: String, tz: String) -> Result<()>;
    fn handle_events(&mut self) -> Result<ControlFlow<()>>;
    async fn ui(frame: &mut Frame, omc: &OpenMeteoClient);
    fn render_title(frame: &mut Frame, area: Rect);
    fn calculate_layout(area: Rect) -> (Rect, Vec<Vec<Rect>>);
    fn placeholder_paragraph() -> Paragraph<'static>;
    fn render_borders(paragraph: &Paragraph, border: Borders, frame: &mut Frame, area: Rect);
    fn render_border_type( paragraph: &Paragraph, border_type: BorderType, frame: &mut Frame, area: Rect);
    fn render_styled_borders(paragraph: &Paragraph, frame: &mut Frame, area: Rect);
    fn render_styled_block(paragraph: &Paragraph, frame: &mut Frame, area: Rect);
    fn render_styled_title(paragraph: &Paragraph, frame: &mut Frame, area: Rect);
    fn render_styled_title_content(paragraph: &Paragraph, frame: &mut Frame, area: Rect);
    fn render_multiple_titles(paragraph: &Paragraph, frame: &mut Frame, area: Rect);
    fn render_padding(paragraph: &Paragraph, frame: &mut Frame, area: Rect);
    fn render_multiple_title_positions(paragraph: &Paragraph, frame: &mut Frame, area: Rect);
    fn render_nested_blocks(paragraph: &Paragraph, frame: &mut Frame, area: Rect);
}

#[async_trait]
impl TUI for Terminal{
    fn setup() -> Result<Terminal>{
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(terminal)
    }

    fn restore(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(self.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<ControlFlow<()>> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(ControlFlow::Break(()));
                }
            }
        }
        Ok(ControlFlow::Continue(()))
    }

    async fn run(&mut self, lon: String, lat: String, tz: String) -> Result<()> {
        let omc = OpenMeteoClient::new(&lon, &lat, &tz);
        loop {
            self.draw(|f| {
                Self::ui(f, &omc);
            })?;
            if self.handle_events()?.is_break() {
                return Ok(());
            }
        }
    }

    async fn ui(frame: &mut Frame, omc: &OpenMeteoClient) {
        let result = omc.get().await.unwrap();
        println!("Result: {:?}", result);
        let (title_area, layout) = Self::calculate_layout(frame.size());

        Self::render_title(frame, title_area);

        let paragraph = Self::placeholder_paragraph();

        Self::render_borders(&paragraph, Borders::ALL, frame, layout[0][0]);
        Self::render_borders(&paragraph, Borders::NONE, frame, layout[0][1]);
        Self::render_borders(&paragraph, Borders::LEFT, frame, layout[1][0]);
        Self::render_borders(&paragraph, Borders::RIGHT, frame, layout[1][1]);
        Self::render_borders(&paragraph, Borders::TOP, frame, layout[2][0]);
        Self::render_borders(&paragraph, Borders::BOTTOM, frame, layout[2][1]);

        Self::render_border_type(&paragraph, BorderType::Plain, frame, layout[3][0]);
        Self::render_border_type(&paragraph, BorderType::Rounded, frame, layout[3][1]);
        Self::render_border_type(&paragraph, BorderType::Double, frame, layout[4][0]);
        Self::render_border_type(&paragraph, BorderType::Thick, frame, layout[4][1]);

        Self::render_styled_block(&paragraph, frame, layout[5][0]);
        Self::render_styled_borders(&paragraph, frame, layout[5][1]);
        Self::render_styled_title(&paragraph, frame, layout[6][0]);
        Self::render_styled_title_content(&paragraph, frame, layout[6][1]);
        Self::render_multiple_titles(&paragraph, frame, layout[7][0]);
        Self::render_multiple_title_positions(&paragraph, frame, layout[7][1]);
        Self::render_padding(&paragraph, frame, layout[8][0]);
        Self::render_nested_blocks(&paragraph, frame, layout[8][1]);
    }

    /// Calculate the layout of the UI elements.
    ///
    /// Returns a tuple of the title area and the main areas.
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

    fn render_title(frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("Block example. Press q to quit")
                .dark_gray()
                .alignment(Alignment::Center),
            area,
        );
    }

    fn placeholder_paragraph() -> Paragraph<'static> {
        let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
        Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true })
    }

    fn render_borders(paragraph: &Paragraph, border: Borders, frame: &mut Frame, area: Rect) {
        let block = Block::new()
            .borders(border)
            .title(format!("Borders::{border:#?}", border = border));
        frame.render_widget(paragraph.clone().block(block), area);
    }

    fn render_border_type(
        paragraph: &Paragraph,
        border_type: BorderType,
        frame: &mut Frame,
        area: Rect,
    ) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(border_type)
            .title(format!("BorderType::{border_type:#?}"));
        frame.render_widget(paragraph.clone().block(block), area);
    }
    fn render_styled_borders(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_style(Style::new().blue().on_white().bold().italic())
            .title("Styled borders");
        frame.render_widget(paragraph.clone().block(block), area);
    }

    fn render_styled_block(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
        let block = Block::new()
            .borders(Borders::ALL)
            .style(Style::new().blue().on_white().bold().italic())
            .title("Styled block");
        frame.render_widget(paragraph.clone().block(block), area);
    }
    // Note: this currently renders incorrectly, see https://github.com/ratatui-org/ratatui/issues/349
    fn render_styled_title(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
        let block = Block::new()
            .borders(Borders::ALL)
            .title("Styled title")
            .title_style(Style::new().blue().on_white().bold().italic());
        frame.render_widget(paragraph.clone().block(block), area);
    }

    fn render_styled_title_content(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
        let title = Line::from(vec![
            "Styled ".blue().on_white().bold().italic(),
            "title content".red().on_white().bold().italic(),
        ]);
        let block = Block::new().borders(Borders::ALL).title(title);
        frame.render_widget(paragraph.clone().block(block), area);
    }

    fn render_multiple_titles(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
        let block = Block::new()
            .borders(Borders::ALL)
            .title("Multiple".blue().on_white().bold().italic())
            .title("Titles".red().on_white().bold().italic());
        frame.render_widget(paragraph.clone().block(block), area);
    }

    fn render_multiple_title_positions(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
        let block = Block::new()
            .borders(Borders::ALL)
            .title(
                Title::from("top left")
                    .position(Position::Top)
                    .alignment(Alignment::Left),
            )
            .title(
                Title::from("top center")
                    .position(Position::Top)
                    .alignment(Alignment::Center),
            )
            .title(
                Title::from("top right")
                    .position(Position::Top)
                    .alignment(Alignment::Right),
            )
            .title(
                Title::from("bottom left")
                    .position(Position::Bottom)
                    .alignment(Alignment::Left),
            )
            .title(
                Title::from("bottom center")
                    .position(Position::Bottom)
                    .alignment(Alignment::Center),
            )
            .title(
                Title::from("bottom right")
                    .position(Position::Bottom)
                    .alignment(Alignment::Right),
            );
        frame.render_widget(paragraph.clone().block(block), area);
    }

    fn render_padding(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
        let block = Block::new()
            .borders(Borders::ALL)
            .title("Padding")
            .padding(Padding::new(5, 10, 1, 2));
        frame.render_widget(paragraph.clone().block(block), area);
    }

    fn render_nested_blocks(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
        let outer_block = Block::new().borders(Borders::ALL).title("Outer block");
        let inner_block = Block::new().borders(Borders::ALL).title("Inner block");
        let inner = outer_block.inner(area);
        frame.render_widget(outer_block, area);
        frame.render_widget(paragraph.clone().block(inner_block), inner);
    }
}
