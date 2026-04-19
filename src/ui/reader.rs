use crate::app::{ActivePane, App};
use crate::ui::theme::{get_theme, ThemeColors};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

fn parse_markdown_line<'a>(
    line: &'a str,
    theme: &ThemeColors,
    query: &str,
    case_sensitive: bool,
) -> Line<'a> {
    let mut spans = Vec::new();

    let is_header = line.starts_with("# ") || line.starts_with("## ");
    let base_style = if is_header {
        Style::default()
            .fg(theme.border_active)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.foreground)
    };

    // Fast path: no markdown and no search query
    if !line.contains('*') && !line.contains('_') && query.is_empty() {
        return Line::from(Span::styled(line, base_style));
    }

    let mut current_chunk = String::with_capacity(line.len());
    let mut in_bold = false;
    let mut in_italic = false;

    let push_chunk =
        |chunk: &mut String, spans: &mut Vec<Span<'a>>, in_bold: bool, in_italic: bool| {
            if chunk.is_empty() {
                return;
            }

            let mut s = base_style;
            if in_bold {
                s = s.add_modifier(Modifier::BOLD);
            }
            if in_italic {
                s = s.add_modifier(Modifier::ITALIC);
            }

            if query.is_empty() {
                spans.push(Span::styled(chunk.clone(), s));
            } else {
                let (search_query, text_to_search) = if case_sensitive {
                    (query.to_string(), chunk.clone())
                } else {
                    (query.to_lowercase(), chunk.to_lowercase())
                };

                let mut search_start = 0;

                while let Some(idx) = text_to_search[search_start..].find(&search_query) {
                    let real_idx = search_start + idx;
                    let before = &chunk[search_start..real_idx];
                    let match_text = &chunk[real_idx..real_idx + query.len()];

                    if !before.is_empty() {
                        spans.push(Span::styled(before.to_string(), s));
                    }
                    spans.push(Span::styled(
                        match_text.to_string(),
                        s.fg(theme.search_match_fg).bg(theme.search_match_bg),
                    ));

                    search_start = real_idx + query.len();
                }
                if search_start < chunk.len() {
                    spans.push(Span::styled(chunk[search_start..].to_string(), s));
                }
            }
            chunk.clear();
        };

    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '*' {
            if chars.peek() == Some(&'*') {
                chars.next();
                push_chunk(&mut current_chunk, &mut spans, in_bold, in_italic);
                in_bold = !in_bold;
            } else {
                push_chunk(&mut current_chunk, &mut spans, in_bold, in_italic);
                in_italic = !in_italic;
            }
        } else if c == '_' && (current_chunk.is_empty() || current_chunk.ends_with(' ')) {
            push_chunk(&mut current_chunk, &mut spans, in_bold, in_italic);
            in_italic = !in_italic;
        } else {
            current_chunk.push(c);
        }
    }

    push_chunk(&mut current_chunk, &mut spans, in_bold, in_italic);
    Line::from(spans)
}

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let start_time = std::time::Instant::now();
    let theme = get_theme(app.theme);

    if let Some(book) = &app.current_book {
        if let Some(chapter) = book.chapters.get(book.current_chapter) {
            let title = format!("Reader - {} ({})", book.title, chapter.title);
            let mut block = Block::default()
                .title(title)
                .borders(Borders::ALL)
                .style(Style::default().bg(theme.background).fg(theme.foreground));

            if matches!(app.active_pane, ActivePane::Reader) {
                block = block.border_style(
                    Style::default()
                        .fg(theme.border_active)
                        .add_modifier(Modifier::BOLD),
                );
            } else {
                block = block.border_style(Style::default().fg(theme.border));
            }

            let content = &chapter.content;

            let query = app.search_query.trim();
            let mut lines = Vec::new();

            if content.trim().is_empty() {
                lines.push(Line::from(Span::styled(
                    "(This chapter is empty or contains no readable text. Press 'n' to go to the next chapter.)",
                    Style::default().fg(theme.foreground),
                )));
            } else {
                for line in content.lines() {
                    lines.push(parse_markdown_line(
                        line,
                        &theme,
                        query,
                        app.case_sensitive_search,
                    ));
                }
            }

            let alignment = match app.text_alignment.as_str() {
                "center" => ratatui::layout::Alignment::Center,
                "right" => ratatui::layout::Alignment::Right,
                _ => ratatui::layout::Alignment::Left,
            };

            let paragraph = Paragraph::new(lines)
                .block(block)
                .alignment(alignment)
                .wrap(Wrap { trim: true })
                .scroll((book.current_position as u16, 0));

            let mut reader_area = area;
            if app.margin_width > 0 {
                let margin = app.margin_width;
                reader_area.x = area.x.saturating_add(margin);
                reader_area.width = area.width.saturating_sub(margin * 2);
                reader_area.y = area.y.saturating_add(margin / 2);
                reader_area.height = area.height.saturating_sub(margin);

                let bg_block = Block::default().style(Style::default().bg(theme.background));
                f.render_widget(bg_block, area); // clear the unused margin space
            }

            f.render_widget(paragraph, reader_area);

            let elapsed = start_time.elapsed();
            if elapsed.as_millis() > 5 {
                // we can't easily println in TUI, let's write to a log file or just assume it's fine
            }
            return;
        } else {
            // Book is open but has no chapters or chapter index is out of bounds
            let title = format!("Reader - {}", book.title);
            let mut block = Block::default()
                .title(title)
                .borders(Borders::ALL)
                .style(Style::default().bg(theme.background).fg(theme.foreground));

            if matches!(app.active_pane, ActivePane::Reader) {
                block = block.border_style(
                    Style::default()
                        .fg(theme.border_active)
                        .add_modifier(Modifier::BOLD),
                );
            } else {
                block = block.border_style(Style::default().fg(theme.border));
            }

            let empty = Paragraph::new("This book appears to have no readable chapters.")
                .block(block)
                .wrap(Wrap { trim: true });

            f.render_widget(empty, area);
            return;
        }
    }

    // No book open

    let mut block = Block::default()
        .title("Book Details")
        .borders(Borders::ALL)
        .style(Style::default().bg(theme.background).fg(theme.foreground));

    if matches!(app.active_pane, ActivePane::Reader) {
        block = block.border_style(
            Style::default()
                .fg(theme.border_active)
                .add_modifier(Modifier::BOLD),
        );
    } else {
        block = block.border_style(Style::default().fg(theme.border));
    }

    if let Some(selected_idx) = app.selected_book_index {
        if let Some(book) = app.filtered_books().get(selected_idx) {
            let mut info_lines = vec![
                Line::from(vec![
                    Span::styled("Title: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(&book.title),
                ]),
                Line::from(vec![
                    Span::styled("Author: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(book.author.as_deref().unwrap_or("Unknown")),
                ]),
                Line::from(vec![
                    Span::styled("Date: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(book.date.as_deref().unwrap_or("Unknown")),
                ]),
            ];

            if let Some(ref desc) = book.description {
                info_lines.push(Line::from(""));
                info_lines.push(Line::from(Span::styled(
                    "Description:",
                    Style::default().add_modifier(Modifier::BOLD),
                )));
                let desc_clean = desc.replace('\n', " ");
                info_lines.push(Line::from(Span::raw(desc_clean)));
            }

            let info_paragraph = Paragraph::new(info_lines)
                .block(block)
                .wrap(Wrap { trim: true });

            f.render_widget(info_paragraph, area);
            return;
        }
    }

    let empty = Paragraph::new("No book selected. Press 'Enter' in Library to open a book.")
        .block(block)
        .wrap(Wrap { trim: true });

    f.render_widget(empty, area);
}
