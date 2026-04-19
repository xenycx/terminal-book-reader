use crate::app::App;
use crate::ui::theme::get_theme;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, ListState},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let theme = get_theme(app.theme);
    let area = f.area();

    // Center popup logic
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(area);

    let popup_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(vertical_layout[1])[1];

    f.render_widget(Clear, popup_area); // Clear background behind popup

    let block = Block::default()
        .title(" Table of Contents (Enter: jump, Esc: close) ")
        .borders(Borders::ALL)
        .style(Style::default().bg(theme.background).fg(theme.foreground))
        .border_style(
            Style::default()
                .fg(theme.border_active)
                .add_modifier(Modifier::BOLD),
        );

    let items: Vec<ListItem> = if let Some(book) = &app.current_book {
        book.chapters
            .iter()
            .enumerate()
            .map(|(i, chapter)| {
                let mut style = Style::default().fg(theme.foreground);
                if i == app.toc_selected_index {
                    style = style.add_modifier(Modifier::REVERSED);
                }
                ListItem::new(format!("{}: {}", i + 1, chapter.title)).style(style)
            })
            .collect()
    } else {
        vec![ListItem::new("No book open")]
    };

    let list = List::new(items).block(block);
    let mut state = ListState::default();
    state.select(Some(app.toc_selected_index));

    f.render_stateful_widget(list, popup_area, &mut state);
}
