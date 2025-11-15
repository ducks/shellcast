use crate::app::{App, PaneFocus};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn draw_ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(frame.area());

    draw_podcast_list(frame, app, chunks[0]);
    draw_episode_list(frame, app, chunks[1]);
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
