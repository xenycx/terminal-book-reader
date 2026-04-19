use crate::app::{App, Theme};
use crate::ui::theme::get_theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let size = f.area();
    let theme = get_theme(app.theme);

    let area = centered_rect(60, 60, size);
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" Preferences ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border_active))
        .style(Style::default().bg(theme.background));

    f.render_widget(block.clone(), area);

    let inner_area = block.inner(area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(2)].as_ref())
        .split(inner_area);

    let settings = vec![
        format!(
            " Theme: {}",
            if matches!(app.theme, Theme::Dark) {
                "Dark"
            } else {
                "Light"
            }
        ),
        format!(" Text Alignment: {}", app.text_alignment),
        format!(" Margin Width: {}", app.margin_width),
        format!(" Startup View: {}", app.startup_view),
        format!(
            " Case Sensitive Search: {}",
            if app.case_sensitive_search {
                "On"
            } else {
                "Off"
            }
        ),
        format!(
            " Show Status Bar: {}",
            if app.show_status_bar { "Yes" } else { "No" }
        ),
        format!(" Target WPM: {}", app.wpm),
    ];

    let items: Vec<ListItem> = settings
        .into_iter()
        .enumerate()
        .map(|(i, s)| {
            if i == app.preferences_selected_index {
                ListItem::new(s).style(
                    Style::default()
                        .fg(theme.foreground)
                        .add_modifier(Modifier::REVERSED)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                ListItem::new(s).style(Style::default().fg(theme.foreground))
            }
        })
        .collect();

    let list = List::new(items);
    f.render_widget(list, chunks[0]);

    let help_text = Line::from(vec![
        Span::styled("Enter/Space", Style::default().fg(Color::Yellow)),
        Span::raw(" Toggle/Edit  "),
        Span::styled("+/-", Style::default().fg(Color::Yellow)),
        Span::raw(" Change value  "),
        Span::styled("Esc/q", Style::default().fg(Color::Yellow)),
        Span::raw(" Close"),
    ]);
    f.render_widget(
        Paragraph::new(help_text).alignment(ratatui::layout::Alignment::Center),
        chunks[1],
    );
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
