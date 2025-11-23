use crate::app::{App, AppScreen, InputMode, PaneFocus};
use crate::playback::Player;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
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

    match app.screen {
        AppScreen::Podcasts => {
            draw_podcasts_screen(frame, app, main_layout[0]);
        }
        AppScreen::Browse => {
            draw_browse_screen(frame, app, main_layout[0]);
        }
    }

    draw_footer(frame, app, player, main_layout[1]);

    // Draw help popup on top if visible
    if app.show_help {
        draw_help_popup(frame);
    }

    // Draw info popup on top if visible
    if app.show_info {
        draw_info_popup(frame, app);
    }

    // Draw chapters popup on top if visible
    if app.show_chapters {
        draw_chapters_popup(frame, app);
    }
}

fn draw_podcasts_screen(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

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

                let date_str = if !episode.published.is_empty() {
                    format!("{} - ", episode.published)
                } else {
                    String::new()
                };

                let label = format!("{} {}{}{}", marker, date_str, episode.title, duration_str);
                
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

fn draw_browse_screen(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search box
            Constraint::Min(1),     // Results list
        ])
        .split(area);

    // Search box
    let search_block = Block::default()
        .title("Search Podcasts (gpodder.net)")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let search_text = if app.browse.is_searching {
        format!("Search: {}█", app.browse.search_query)
    } else {
        format!("Search: {} (Press '/' to search)", app.browse.search_query)
    };

    let search_para = Paragraph::new(search_text)
        .block(search_block);
    frame.render_widget(search_para, chunks[0]);

    // Results list
    let items: Vec<ListItem> = app.browse.search_results
        .iter()
        .map(|result| {
            let subs_text = if result.subscribers > 0 {
                format!(" ({} subs)", result.subscribers)
            } else {
                String::new()
            };
            let title = format!("▸ {}{}", result.title, subs_text);
            let author = format!("  by {}", result.author);
            let content = format!("{}\n{}", title, author);
            ListItem::new(content)
        })
        .collect();

    let title = if app.browse.showing_defaults {
        format!("Featured Podcasts ({})", app.browse.search_results.len())
    } else {
        format!("Results ({})", app.browse.search_results.len())
    };

    let results_list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_symbol("➤ ")
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_stateful_widget(
        results_list,
        chunks[1],
        &mut ratatui::widgets::ListState::default().with_selected(Some(app.browse.selected_index)),
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
    if app.playback.start.is_some() {
        // Get actual playback position from the player
        let elapsed = player.get_position();
        let total = std::time::Duration::from_secs(app.playback.duration_secs);

        let has_duration = app.playback.duration_secs > 0;
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
        let keybindings = "j/k: Navigate | Tab: Switch | Space: Play | s: Stop | m: Mark | a: Add | d: Delete | ?: Help | q: Quit";
        frame.render_widget(
            Paragraph::new(keybindings).style(Style::default().fg(Color::DarkGray)),
            footer_layout[1]
        );
    }
}

/// Helper function to create a centered rect
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

fn draw_help_popup(frame: &mut Frame) {
    let area = centered_rect(70, 80, frame.area());

    let help_text = r#"
SHELLCAST - KEYBINDINGS

Navigation:
  j/k or ↑/↓     Move up/down in lists
  g/G            Jump to top/bottom of list
  Tab            Switch focus between podcast list and episode list

Screen Switching:
  1              Switch to Podcasts view
  5              Switch to Browse/Search view

Browse Mode:
  /              Start searching (when in Browse mode)
  Enter          Subscribe to selected search result

Playback:
  Space          Play/pause selected episode
  s              Stop playback
  h or ←         Seek backward 30 seconds
  l or →         Seek forward 30 seconds

Management:
  m              Mark episode as played/unplayed
  a              Add new podcast feed (enter URL)
  d              Delete selected podcast
  i              Show episode info/description
  c              Show episode chapters (if available)

Help & Exit:
  ?              Toggle this help screen
  Esc            Close popups
  q              Quit application
"#;

    // Clear the area
    frame.render_widget(Clear, area);

    // Draw the help popup
    let block = Block::default()
        .title(" Help - Press ? or Esc to close ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

fn draw_info_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(70, 70, frame.area());

    // Get the currently selected episode's info
    let (title, published, description, duration) = if let Some(podcast) = app.selected_podcast() {
        if let Some(episode) = podcast.episodes.get(app.selected_episode_index) {
            let duration_str = if let Some(dur) = episode.duration {
                let mins = dur.as_secs() / 60;
                let secs = dur.as_secs() % 60;
                format!("Duration: {:02}:{:02}", mins, secs)
            } else {
                "Duration: Unknown".to_string()
            };

            (
                episode.title.clone(),
                episode.published.clone(),
                episode.description.clone(),
                duration_str,
            )
        } else {
            ("No episode selected".to_string(), String::new(), String::new(), String::new())
        }
    } else {
        ("No podcast selected".to_string(), String::new(), String::new(), String::new())
    };

    let info_text = format!(
        "{}\n\nPublished: {}\n{}\n\n{}\n",
        title,
        if published.is_empty() { "Unknown" } else { &published },
        duration,
        if description.is_empty() { "No description available." } else { &description }
    );

    // Clear the area
    frame.render_widget(Clear, area);

    // Draw the info popup
    let block = Block::default()
        .title(" Episode Info - Press i or Esc to close ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));

    let paragraph = Paragraph::new(info_text)
        .block(block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

fn draw_chapters_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(70, 70, frame.area());

    let content = if let Some(chapter_list) = &app.cached_chapters {
        if chapter_list.chapters.is_empty() {
            "No chapters available for this episode.".to_string()
        } else {
            let episode = app.selected_podcast()
                .and_then(|p| p.episodes.get(app.selected_episode_index));

            // Build chapter list with selection
            let mut lines = vec![
                format!("Episode: {}", episode.map(|e| e.title.as_str()).unwrap_or("Unknown")),
                String::new(),
                "Chapters:".to_string(),
                String::new(),
            ];

            for (idx, chapter) in chapter_list.chapters.iter().enumerate() {
                let mins = (chapter.start_time / 60.0) as u64;
                let secs = (chapter.start_time % 60.0) as u64;
                let prefix = if idx == app.selected_chapter_index {
                    "▶ "
                } else {
                    "  "
                };
                lines.push(format!("{}{:02}:{:02} - {}", prefix, mins, secs, chapter.title));
            }

            lines.push(String::new());
            lines.push("Navigation: j/k to move, Enter to jump, c or Esc to close".to_string());

            lines.join("\n")
        }
    } else {
        "No chapters available for this episode.".to_string()
    };

    // Clear the area
    frame.render_widget(Clear, area);

    // Draw the chapters popup
    let block = Block::default()
        .title(" Episode Chapters - Press c or Esc to close ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));

    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}
