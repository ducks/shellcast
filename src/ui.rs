use crate::app::{App, InputMode, PaneFocus};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub fn draw_ui(frame: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Main content
            Constraint::Length(2), // Footer
        ])
        .split(frame.area());

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(main_layout[0]);

    draw_podcast_list(frame, app, chunks[0]);
    draw_episode_list(frame, app, chunks[1]);
    draw_footer(frame, app, main_layout[1]);

    // Draw input popup if in input mode
    if app.input_mode == InputMode::AddingFeed {
        draw_input_popup(frame, app);
    }

    // Draw status message if present
    if let Some(msg) = &app.status_message {
        draw_status_message(frame, msg);
    }
}

fn draw_podcast_list(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .podcasts
        .iter()
        .enumerate()
        .map(|(i, podcast)| {
            let unplayed_count = podcast.episodes.iter().filter(|e| !e.played).count();
            let label = if unplayed_count > 0 {
                format!("▸ {} ({})", podcast.title, unplayed_count)
            } else {
                format!("▸ {}", podcast.title)
            };
            
            let style = if i == app.selected_podcast_index {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            
            ListItem::new(label).style(style)
        })
        .collect();

    let border_style = if app.focus == PaneFocus::Left {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title("Podcasts")
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .highlight_symbol("➤ ")
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_stateful_widget(
        list,
        area,
        &mut ratatui::widgets::ListState::default().with_selected(Some(app.selected_podcast_index)),
    );
}

fn draw_episode_list(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = if let Some(podcast) = app.selected_podcast() {
        podcast
            .episodes
            .iter()
            .enumerate()
            .map(|(i, episode)| {
                let marker = if episode.played { "○" } else { "●" };
                let duration_str = if let Some(dur) = episode.duration {
                    let mins = dur.as_secs() / 60;
                    let secs = dur.as_secs() % 60;
                    format!(" [{:02}:{:02}]", mins, secs)
                } else {
                    String::new()
                };
                
                let label = format!("{} {}{}", marker, episode.title, duration_str);
                
                let style = if i == app.selected_episode_index {
                    Style::default().add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                
                ListItem::new(label).style(style)
            })
            .collect()
    } else {
        vec![]
    };

    let border_style = if app.focus == PaneFocus::Right {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let title = if let Some(podcast) = app.selected_podcast() {
        format!("Episodes - {}", podcast.title)
    } else {
        "Episodes".to_string()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .highlight_symbol("➤ ")
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_stateful_widget(
        list,
        area,
        &mut ratatui::widgets::ListState::default().with_selected(Some(app.selected_episode_index)),
    );
}

fn draw_input_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, frame.area());

    let block = Block::default()
        .title("Add Feed (Enter to submit, Esc to cancel)")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let input = Paragraph::new(app.input_buffer.as_str())
        .block(block)
        .style(Style::default().fg(Color::Yellow));

    frame.render_widget(Clear, area);
    frame.render_widget(input, area);
}

fn draw_status_message(frame: &mut Frame, message: &str) {
    let area = centered_rect(80, 10, frame.area());

    let paragraph = Paragraph::new(message)
        .block(
            Block::default()
                .title("Status")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black)),
        )
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan));

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}

fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::TOP);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let footer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(inner);

    // Left side: keybindings
    let keybindings = "j/k: Navigate | Tab: Switch | Space: Play | s: Stop | m: Mark | a: Add | d: Delete | q: Quit";
    let keybindings_widget = Paragraph::new(keybindings)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(keybindings_widget, footer_layout[0]);

    // Right side: playing status
    if let Some(podcast) = app.selected_podcast() {
        if let Some(episode) = podcast.episodes.get(app.selected_episode_index) {
            let status_text = format!("♫ {}", episode.title);
            let status_widget = Paragraph::new(status_text)
                .alignment(Alignment::Right)
                .style(Style::default().fg(Color::Cyan));
            frame.render_widget(status_widget, footer_layout[1]);
        }
    }
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
