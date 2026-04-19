use crate::app::{App, AppMode};
use crate::ui::theme::get_theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::Paragraph,
    Frame,
};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let theme = get_theme(app.theme);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(75)].as_ref())
        .split(area);

    let bg_color = match app.mode {
        AppMode::Normal | AppMode::Toc | AppMode::Preferences => theme.footer_bg,
        AppMode::Command => theme.border_active,
        AppMode::Search | AppMode::OnlineSearch | AppMode::LibrarySearch => theme.search_match_bg,
    };

    let fg_color = match app.mode {
        AppMode::Normal | AppMode::Toc | AppMode::Preferences => theme.footer_fg,
        AppMode::Command => theme.background,
        AppMode::Search | AppMode::OnlineSearch | AppMode::LibrarySearch => theme.search_match_fg,
    };

    match app.mode {
        AppMode::Normal | AppMode::Toc | AppMode::Preferences => {
            let mut status = "':' cmd, '/' search, 's' sort, 'Tab' switch, 'q' quit".to_string();
            if let Some(ref msg) = app.error_message {
                if msg.to_lowercase().contains("fail") || msg.to_lowercase().contains("error") {
                    status = format!("Error: {}", msg);
                } else {
                    status = msg.clone();
                }
            }

            let p = Paragraph::new(status)
                .style(Style::default().bg(bg_color).fg(fg_color))
                .alignment(Alignment::Left);
            f.render_widget(p, chunks[0]);
        }
        AppMode::Command => {
            let mut spans = vec![ratatui::text::Span::raw(format!(":{}", app.command_buffer))];

            if !app.command_suggestions.is_empty() {
                let idx = app.suggestion_index.unwrap_or(0);
                if let Some(sug) = app.command_suggestions.get(idx) {
                    if sug.starts_with(&app.command_buffer) {
                        let remainder = &sug[app.command_buffer.len()..];
                        spans.push(ratatui::text::Span::styled(
                            remainder,
                            Style::default().fg(ratatui::style::Color::DarkGray),
                        ));
                    } else {
                        spans.push(ratatui::text::Span::styled(
                            format!(" -> {}", sug),
                            Style::default().fg(ratatui::style::Color::DarkGray),
                        ));
                    }
                }
            }

            let text = ratatui::text::Line::from(spans);
            let p = Paragraph::new(text)
                .style(Style::default().bg(bg_color).fg(fg_color))
                .alignment(Alignment::Left);
            f.render_widget(p, chunks[0]);
        }
        AppMode::Search | AppMode::OnlineSearch | AppMode::LibrarySearch => {
            let text = format!("/{}", app.command_buffer);
            let p = Paragraph::new(text)
                .style(Style::default().bg(bg_color).fg(fg_color))
                .alignment(Alignment::Left);
            f.render_widget(p, chunks[0]);
        }
    }

    // Render stats
    let mut stats_text = String::new();
    if let Some(book) = &app.current_book {
        let ch_idx = book.current_chapter;
        let total_ch = book.chapters.len();

        let mut pct = 0;
        if let Some(chapter) = book.chapters.get(ch_idx) {
            let total_lines = chapter.content.lines().count().max(1);
            let current_line = book.current_position.min(total_lines);
            pct = (current_line * 100) / total_lines;
        }

        if let Some((mins_chapter, mins_book)) = app.calculate_reading_stats() {
            let h = book.time_spent_secs / 3600;
            let m = (book.time_spent_secs % 3600) / 60;
            let time_str = if h > 0 {
                format!("{}h {}m", h, m)
            } else {
                format!("{}m", m)
            };

            stats_text = format!(
                "Ch {}/{} ({}%) | {}m left ch | {}m left book | Time: {} ",
                ch_idx + 1,
                total_ch,
                pct,
                mins_chapter,
                mins_book,
                time_str
            );
        }
    }

    let stats_p = Paragraph::new(stats_text)
        .style(Style::default().bg(bg_color).fg(fg_color))
        .alignment(Alignment::Right);
    f.render_widget(stats_p, chunks[1]);
}
