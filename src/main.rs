mod actions;
mod app;
mod feed;
mod keybindings;
mod ui;

use app::{App, InputMode};
use actions::Action;
use keybindings::{KeyMap, KeyBinding};

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{Result, stdout};

fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::with_dummy_data();
    let keymap = KeyMap::with_defaults();

    loop {
        terminal.draw(|f| ui::draw_ui(f, &app))?;

        if event::poll(std::time::Duration::from_millis(200))?
            && let Event::Key(key) = event::read()?
        {
            // Handle input mode separately
            if app.input_mode == InputMode::AddingFeed {
                use crossterm::event::KeyCode;
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
            } else {
                // Clear status message on any keypress
                if app.status_message.is_some() {
                    app.status_message = None;
                    continue;
                }

                // Normal mode keybindings
                let binding = KeyBinding::new(key.code);

                if let Some(action) = keymap.get_action(&binding) {
                    if matches!(action, Action::Quit) {
                        break;
                    }
                    action.execute(&mut app);
                }
            }
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
