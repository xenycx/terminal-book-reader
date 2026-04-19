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

    let title = if app.library_filter.is_empty() {
        "Library".to_string()
    } else {
        format!("Library (Filter: {})", app.library_filter)
    };

    let mut block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().bg(theme.background).fg(theme.foreground));

    if matches!(app.active_pane, ActivePane::Library) {
        block = block.border_style(
            Style::default()
                .fg(theme.border_active)
                .add_modifier(Modifier::BOLD),
        );
    } else {
        block = block.border_style(Style::default().fg(theme.border));
    }

    let items: Vec<ListItem> = app
        .filtered_books()
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let mut style = Style::default().fg(theme.foreground);
            if Some(i) == app.selected_book_index {
                style = style.add_modifier(Modifier::REVERSED);
            }
            let label = if let Some(ref cat) = b.category {
                format!("[{}] {}", cat, b.title)
            } else {
                b.title.clone()
            };
            ListItem::new(label).style(style)
        })
        .collect();

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}
