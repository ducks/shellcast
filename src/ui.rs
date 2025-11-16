use crate::app::{App, InputMode, PaneFocus};
use crate::playback::Player;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame,
};

pub fn draw_ui(frame: &mut Frame, app: &App, player: &Player) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Main content
            Constraint::Length(4), // Footer (border + status + keybindings + padding)
        ])
        .split(frame.area());

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(main_layout[0]);

    draw_podcast_list(frame, app, chunks[0]);
    draw_episode_list(frame, app, chunks[1]);
    draw_footer(frame, app, player, main_layout[1]);
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

fn draw_footer(frame: &mut Frame, app: &App, player: &Player, area: Rect) {
    let block = Block::default().borders(Borders::TOP);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    // If in input mode, just show the input prompt
    if app.input_mode == InputMode::AddingFeed {
        let text = format!("Add Feed: {}", app.input_buffer);
        frame.render_widget(
            Paragraph::new(text).style(Style::default().fg(Color::Yellow)),
            inner
        );
        return;
    }

    // Check if we're playing something
    if app.playback_start.is_some() {
        // Get actual playback position from the player
        let elapsed = player.get_position();
        let total = std::time::Duration::from_secs(app.playback_duration);

        let has_duration = app.playback_duration > 0;
        let ratio = if has_duration && total.as_secs_f64() > 0.0 {
            elapsed.as_secs_f64() / total.as_secs_f64()
        } else {
            0.0
        };

        let footer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Progress bar
                Constraint::Length(1), // Time display
                Constraint::Length(1), // Keybindings
            ])
            .split(inner);

        // Progress bar
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Cyan))
            .ratio(ratio.min(1.0));
        frame.render_widget(gauge, footer_layout[0]);

        // Time display or status message
        if let Some(msg) = &app.status_message {
            // Show status message
            frame.render_widget(
                Paragraph::new(msg.as_str()).style(Style::default().fg(Color::Yellow)),
                footer_layout[1]
            );
        } else {
            // Show time
            let elapsed_secs = elapsed.as_secs();
            let time_text = if has_duration {
                let total_secs = total.as_secs();
                format!(
                    "{:02}:{:02} / {:02}:{:02}",
                    elapsed_secs / 60,
                    elapsed_secs % 60,
                    total_secs / 60,
                    total_secs % 60
                )
            } else {
                format!(
                    "{:02}:{:02} / --:--",
                    elapsed_secs / 60,
                    elapsed_secs % 60
                )
            };
            frame.render_widget(Paragraph::new(time_text), footer_layout[1]);
        }

        // Keybindings
        let keybindings = "j/k: Navigate | Tab: Switch | Space: Pause | s: Stop | m: Mark | a: Add | d: Delete | q: Quit";
        frame.render_widget(
            Paragraph::new(keybindings).style(Style::default().fg(Color::DarkGray)),
            footer_layout[2]
        );
    } else {
        // Not playing - show status message or keybindings
        let footer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Status message line
                Constraint::Length(1), // Keybindings line
                Constraint::Min(0),    // Remaining space
            ])
            .split(inner);

        // Line 1: Status message (if present)
        if let Some(msg) = &app.status_message {
            frame.render_widget(
                Paragraph::new(msg.as_str()).style(Style::default().fg(Color::Yellow)),
                footer_layout[0]
            );
        }

        // Line 2: Keybindings (always visible)
        let keybindings = "j/k: Navigate | Tab: Switch | Space: Play | s: Stop | m: Mark | a: Add | d: Delete | q: Quit";
        frame.render_widget(
            Paragraph::new(keybindings).style(Style::default().fg(Color::DarkGray)),
            footer_layout[1]
        );
    }
}
