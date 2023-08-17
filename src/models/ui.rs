use ratatui::{
    prelude::*,
    backend::Backend,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph, Wrap, Table, Row, Cell},
    Frame,
};
use itertools::Itertools;

use crate::models::app::App;


/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App<'_>, frame: &mut Frame<'_, B>) {
    let (title_area, layout) = calculate_layout(frame.size());

    render_title(frame, title_area);

    let paragraph = placeholder_paragraph();

    //let data = app.omc.get().unwrap();

    render_borders(&paragraph, Borders::ALL, frame, layout[0][0]);
    render_borders(&paragraph, Borders::NONE, frame, layout[0][1]);
    render_borders(&paragraph, Borders::LEFT, frame, layout[1][0]);
    render_borders(&paragraph, Borders::RIGHT, frame, layout[1][1]);
    render_borders(&paragraph, Borders::TOP, frame, layout[2][0]);
    render_borders(&paragraph, Borders::BOTTOM, frame, layout[2][1]);
    render_table(frame, layout[3][0]);
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
        .constraints(vec![Constraint::Max(10); 9])
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

fn render_table<B: Backend>(frame: &mut Frame<'_, B>, area: Rect) {
    let table = Table::new(vec![
        // Row can be created from simple strings.
        Row::new(vec!["Row11", "Row12", "Row13"]),
        // You can style the entire row.
        Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::default().fg(Color::Blue)),
        // If you need more control over the styling you may need to create Cells directly
        Row::new(vec![
            Cell::from("Row31"),
            Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
            Cell::from(Line::from(vec![
                Span::raw("Row"),
                Span::styled("33", Style::default().fg(Color::Green))
            ])),
        ]),
        // If a Row need to display some content over multiple lines, you just have to change
        // its height.
        Row::new(vec![
            Cell::from("Row\n41"),
            Cell::from("Row\n42"),
            Cell::from("Row\n43"),
        ]).height(2),
    ])
        // You can set the style of the entire Table.
        .style(Style::default().fg(Color::White))
        // It has an optional header, which is simply a Row always visible at the top.
        .header(
            Row::new(vec!["Col1", "Col2", "Col3"])
                .style(Style::default().fg(Color::Yellow))
                // If you want some space between the header and the rest of the rows, you can always
                // specify some margin at the bottom.
                .bottom_margin(1)
        )
        // As any other widget, a Table can be wrapped in a Block.
        .block(Block::default().borders(Borders::ALL).title("Table"))
        // Columns widths are constrained in the same way as Layout...
        .widths(&[Constraint::Length(5), Constraint::Length(5), Constraint::Length(10)])
        // ...and they can be separated by a fixed spacing.
        .column_spacing(1)
        // If you wish to highlight a row in any specific way when it is selected...
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        // ...and potentially show a symbol in front of the selection.
        .highlight_symbol(">>");
    frame.render_widget(table, area);
}
