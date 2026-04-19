use crate::app::{ActivePane, App};
use crate::ui::theme::get_theme;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let theme = get_theme(app.theme);

    let title = if app.is_loading {
        " Downloader (Searching...) "
    } else {
        " Downloader "
    };
    let mut block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().bg(theme.background).fg(theme.foreground));

    if matches!(app.active_pane, ActivePane::Downloader) {
        block = block.border_style(
            Style::default()
                .fg(theme.border_active)
                .add_modifier(Modifier::BOLD),
        );
    } else {
        block = block.border_style(Style::default().fg(theme.border));
    }

    let items: Vec<ListItem> = app
        .online_books
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let mut style = Style::default().fg(theme.foreground);
            if Some(i) == app.selected_online_index {
                style = style.add_modifier(Modifier::REVERSED);
            }

            let author_names = b
                .authors
                .iter()
                .map(|a| a.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            let text = format!("{} - {}", b.title, author_names);

            ListItem::new(text).style(style)
        })
        .collect();

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}
