mod actions;
mod app;
mod browse;
mod chapters;
mod config;
mod feed;
mod keybindings;
mod persistence;
mod playback;
mod theme;
mod ui;

use app::{App, InputMode};
use actions::Action;
use keybindings::{KeyMap, KeyBinding};
use playback::Player;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use simplelog::*;
use std::fs::File;
use std::io::{Result, stdout};

fn handle_adding_feed_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char(c) => {
            app.input_buffer.push(c);
        }
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        KeyCode::Enter => {
            let url = app.input_buffer.clone();
            app.input_mode = InputMode::Normal;
            app.input_buffer.clear();

            // Fetch and parse feed
            match feed::fetch_and_parse(&url) {
                Ok(podcast) => {
                    app.status_message = Some(format!("Added: {}", podcast.title));
                    app.add_podcast(podcast);
                }
                Err(e) => {
                    app.status_message = Some(format!("Error: {}", e));
                }
            }
        }
        KeyCode::Esc => {
            app.cancel_input();
        }
        _ => {}
    }
}

fn handle_search_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char(c) => {
            app.browse.search_query.push(c);
        }
        KeyCode::Backspace => {
            app.browse.search_query.pop();
        }
        KeyCode::Enter => {
            let query = app.browse.search_query.clone();
            app.cancel_search();

            // Perform search
            match browse::search_podcasts(&query) {
                Ok(results) => {
                    app.browse.search_results = results;
                    app.browse.selected_index = 0;
                    app.browse.showing_defaults = false;
                    app.status_message = Some(format!("Found {} podcasts", app.browse.search_results.len()));
                }
                Err(e) => {
                    app.status_message = Some(format!("Search error: {}", e));
                }
            }
        }
        KeyCode::Esc => {
            app.cancel_search();
        }
        _ => {}
    }
}

fn handle_browse_screen_key(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('/') => {
            app.start_search();
            true
        }
        KeyCode::Enter => {
            // Subscribe to selected podcast
            if let Some(result) = app.browse.selected_result() {
                let feed_url = result.feed_url.clone();
                match feed::fetch_and_parse(&feed_url) {
                    Ok(podcast) => {
                        app.status_message = Some(format!("Subscribed: {}", podcast.title));
                        app.add_podcast(podcast);
                        app.screen = app::AppScreen::Podcasts;
                    }
                    Err(e) => {
                        app.status_message = Some(format!("Error: {}", e));
                    }
                }
            }
            true
        }
        _ => false,
    }
}

fn handle_normal_key(
    app: &mut App,
    player: &mut Player,
    keymap: &KeyMap,
    key: KeyEvent,
) -> bool {
    // Clear status message on any keypress
    app.status_message = None;

    // Handle Esc key to close popups
    if key.code == KeyCode::Esc {
        if app.show_help {
            app.show_help = false;
            return false;
        }
        if app.show_info {
            app.show_info = false;
            return false;
        }
        if app.show_chapters {
            app.show_chapters = false;
            return false;
        }
    }

    // Handle chapter navigation when chapters popup is visible
    if app.show_chapters {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                // Use cached chapters to get max index
                if let Some(chapter_list) = &app.cached_chapters {
                    let max_idx = chapter_list.chapters.len().saturating_sub(1);
                    app.move_chapter_down(max_idx);
                }
                return false;
            }
            KeyCode::Char('k') | KeyCode::Up => {
                app.move_chapter_up();
                return false;
            }
            KeyCode::Enter => {
                // Jump to selected chapter from cache
                if let Some(chapter_list) = &app.cached_chapters {
                    if let Some(chapter) = chapter_list.chapters.get(app.selected_chapter_index) {
                        let start_secs = chapter.start_time as u64;
                        // Seek to the chapter start time
                        if app.playback.start.is_some() {
                            // Calculate current position and seek relative to it
                            let current_pos = player.get_position().as_secs();
                            if start_secs > current_pos {
                                let _ = player.seek_forward(start_secs - current_pos);
                            } else if start_secs < current_pos {
                                let _ = player.seek_backward(current_pos - start_secs);
                            }
                            app.status_message = Some(format!("Jumped to: {}", chapter.title));
                        } else {
                            app.status_message = Some("Start playback first".to_string());
                        }
                    }
                }
                app.show_chapters = false;
                return false;
            }
            _ => {}
        }
    }

    // Normal mode keybindings
    let binding = KeyBinding::new(key.code);

    if let Some(action) = keymap.get_action(&binding) {
        if matches!(action, Action::Quit) {
            return true;
        }

        // Handle playback actions
        match action {
            Action::PlayPause => {
                let selected_url = app.selected_episode_url();
                let is_different_episode = selected_url.as_ref() != app.playback.url.as_ref();

                // If user selected a different episode, stop current and play new one
                if is_different_episode || (!player.is_playing() && !player.is_paused()) {
                    // Get episode info before borrowing
                    let episode_info = app.selected_podcast()
                        .and_then(|p| p.episodes.get(app.selected_episode_index))
                        .map(|e| (e.audio_url.clone(), e.title.clone(), e.duration));

                    if let Some((audio_url, title, duration)) = episode_info {
                        if !audio_url.is_empty() {
                            match player.play(&audio_url) {
                                Ok(_) => {
                                    app.status_message = Some(format!("Playing: {}", title));
                                    app.playback.url = Some(audio_url);

                                    // Start playback tracking
                                    app.playback.start = Some(std::time::Instant::now());
                                    app.playback.duration_secs = duration.map(|d| d.as_secs()).unwrap_or(0);
                                    app.playback.paused_at = None;
                                    app.playback.paused_duration = std::time::Duration::ZERO;
                                }
                                Err(e) => {
                                    app.status_message = Some(format!("Error: {}", e));
                                    app.playback.url = None;
                                    app.playback.start = None;
                                }
                            }
                        } else {
                            app.status_message = Some("No audio URL for this episode".to_string());
                        }
                    }
                } else if player.is_paused() {
                    // Resume current episode
                    player.resume();
                    let title = app.selected_podcast()
                        .and_then(|p| p.episodes.get(app.selected_episode_index))
                        .map(|e| e.title.clone());
                    if let Some(title) = title {
                        app.status_message = Some(format!("Resumed: {}", title));
                    }

                    // Resume playback tracking
                    if let Some(paused_at) = app.playback.paused_at {
                        app.playback.paused_duration += std::time::Instant::now().duration_since(paused_at);
                        app.playback.paused_at = None;
                    }
                } else {
                    // Pause current episode
                    player.pause();
                    let title = app.selected_podcast()
                        .and_then(|p| p.episodes.get(app.selected_episode_index))
                        .map(|e| e.title.clone());
                    if let Some(title) = title {
                        app.status_message = Some(format!("Paused: {}", title));
                    }

                    // Mark pause time
                    app.playback.paused_at = Some(std::time::Instant::now());
                }
            }
            Action::Stop => {
                player.stop();
                app.status_message = Some("Stopped".to_string());
                app.playback.url = None;

                // Clear playback tracking
                app.playback.start = None;
                app.playback.paused_at = None;
                app.playback.paused_duration = std::time::Duration::ZERO;
            }
            Action::SeekForward => {
                if app.playback.start.is_some() {
                    match player.seek_forward(30) {
                        Ok(_) => {
                            app.status_message = Some("⏩ +30s".to_string());
                        }
                        Err(e) => {
                            app.status_message = Some(format!("Seek error: {}", e));
                        }
                    }
                }
            }
            Action::SeekBackward => {
                if app.playback.start.is_some() {
                    match player.seek_backward(30) {
                        Ok(_) => {
                            app.status_message = Some("⏪ -30s".to_string());
                        }
                        Err(e) => {
                            app.status_message = Some(format!("Seek error: {}", e));
                        }
                    }
                }
            }
            Action::RefreshFeed => {
                if let Some(podcast) = app.podcasts.get_mut(app.selected_podcast_index) {
                    match feed::refresh_feed(podcast) {
                        Ok(count) => {
                            app.status_message = if count > 0 {
                                Some(format!("Refreshed: {} new episode(s)", count))
                            } else {
                                Some("Refreshed: No new episodes".to_string())
                            };
                            app.needs_save = true;
                        }
                        Err(e) => {
                            app.status_message = Some(format!("Refresh error: {}", e));
                        }
                    }
                }
            }
            _ => {
                action.execute(app);
            }
        }
    }

    false
}

fn main() -> Result<()> {
    // Initialize logging
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        File::create("shellcast-debug.log").unwrap(),
    )])
    .unwrap();

    log::info!("Shellcast starting...");

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // Load config and theme
    let config = config::Config::load();
    let theme = config.get_theme();
    log::info!("Using theme: {}", config.theme.name);

    // Load podcasts from file, or use empty app if none exist
    let mut app = match persistence::load_podcasts() {
        Ok(podcasts) => {
            let mut app = App::new();
            app.podcasts = podcasts;
            app
        }
        Err(_) => App::new(),
    };
    let keymap = KeyMap::with_defaults();

    // Initialize audio player
    let mut player = Player::new().expect("Failed to initialize audio player");

    loop {
        terminal.draw(|f| ui::draw_ui(f, &app, &player, &theme))?;

        if event::poll(std::time::Duration::from_millis(50))?
            && let Event::Key(key) = event::read()?
        {
            match app.input_mode {
                InputMode::AddingFeed => {
                    handle_adding_feed_input(&mut app, key);
                }
                InputMode::Searching => {
                    handle_search_input(&mut app, key);
                }
                InputMode::Normal => {
                    // Handle browse-specific keys first
                    if app.is_browse_screen() && handle_browse_screen_key(&mut app, key) {
                        continue;
                    }

                    // Handle normal mode keys
                    if handle_normal_key(&mut app, &mut player, &keymap, key) {
                        break;
                    }
                }
            }
        }

        // Centralized persistence
        if app.needs_save {
            if let Err(e) = persistence::save_podcasts(&app.podcasts) {
                app.status_message = Some(format!("Save error: {}", e));
            }
            app.needs_save = false;
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use atom_syndication::Feed as AtomFeed;
    use rss::Channel;
    use std::io::BufReader;

    #[test]
    fn test_parse_podcast_feed() {
        // Test with Swedish Radio podcast feed (Atom format)
        let feed_url = "https://api.sr.se/api/rss/program/4916";

        let response = reqwest::blocking::get(feed_url)
            .expect("Failed to fetch feed");

        let reader = BufReader::new(response);
        let feed = AtomFeed::read_from(reader)
            .expect("Failed to parse Atom feed");

        // Check basic feed info
        println!("Title: {}", feed.title().value);
        if let Some(subtitle) = feed.subtitle() {
            println!("Subtitle: {}", subtitle.value);
        }
        println!("Link: {:?}", feed.links().first().map(|l| l.href()));

        assert!(!feed.title().value.is_empty());
        assert!(!feed.entries().is_empty());

        // Check first episode
        if let Some(episode) = feed.entries().first() {
            println!("\nFirst episode:");
            println!("  Title: {}", episode.title().value);
            println!("  Published: {:?}", episode.published());

            // Check for links (audio file might be in links)
            for link in episode.links() {
                println!("  Link: {} (rel: {:?})", link.href(), link.rel());
            }

            // Check content
            if let Some(content) = episode.content() {
                let content_str = content.value().unwrap_or("");
                if content_str.contains("http://api.sr.se/api/radio/") {
                    println!("  Has audio link in content");
                }
            }
        }
    }

    #[test]
    fn test_parse_local_feed() {
        // Simple test with inline RSS
        let rss_str = r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <rss version="2.0" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd">
              <channel>
                <title>Test Podcast</title>
                <description>A test podcast</description>
                <link>https://example.com</link>
                <item>
                  <title>Episode 1</title>
                  <description>First episode</description>
                  <enclosure url="https://example.com/episode1.mp3" type="audio/mpeg" length="12345678"/>
                  <itunes:duration>30:00</itunes:duration>
                </item>
              </channel>
            </rss>
        "#;

        let channel = Channel::read_from(rss_str.as_bytes())
            .expect("Failed to parse RSS");

        assert_eq!(channel.title(), "Test Podcast");
        assert_eq!(channel.items().len(), 1);

        let episode = &channel.items()[0];
        assert_eq!(episode.title().unwrap(), "Episode 1");
        assert!(episode.enclosure().is_some());
    }
}
