use ratatui::{
    prelude::*,
    backend::Backend,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use itertools::Itertools;

use crate::models::app::App;


/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App<'_>, frame: &mut Frame<'_, B>) {
    let (title_area, layout) = calculate_layout(frame.size());

    render_title(frame, title_area);

    let paragraph = placeholder_paragraph();

    let data = app.omc.get().unwrap();

    render_borders(&paragraph, Borders::ALL, frame, layout[0][0]);
    render_borders(&paragraph, Borders::NONE, frame, layout[0][1]);
    render_borders(&paragraph, Borders::LEFT, frame, layout[1][0]);
    render_borders(&paragraph, Borders::RIGHT, frame, layout[1][1]);
    render_borders(&paragraph, Borders::TOP, frame, layout[2][0]);
    render_borders(&paragraph, Borders::BOTTOM, frame, layout[2][1]);
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
fn render_title<B: Backend>(frame: &mut Frame<'_, B>, area: Rect) {
    frame.render_widget(
        Paragraph::new("Block example. Press q to quit")
            .dark_gray()
            .alignment(Alignment::Center),
        area,
    );
}

fn placeholder_paragraph() -> Paragraph<'static> {
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    Paragraph::new(text.light_red()).wrap(Wrap { trim: true })
}

fn render_borders<B: Backend>(paragraph: &Paragraph, border: Borders, frame: &mut Frame<'_, B>, area: Rect) {
    let block = Block::new()
        .borders(border)
        .title(format!("Borders::{border:#?}", border = border));
    frame.render_widget(paragraph.clone().block(block), area);
}


