use crate::app::{Episode, Podcast};
use atom_syndication::Feed as AtomFeed;
use rss::Channel;
use std::io::BufReader;
use std::time::Duration;

pub fn fetch_and_parse(url: &str) -> Result<Podcast, String> {
    let response = reqwest::blocking::get(url)
        .map_err(|e| format!("Failed to fetch feed: {}", e))?;

    // Try parsing as RSS first
    let reader = BufReader::new(response);
    if let Ok(channel) = Channel::read_from(reader) {
        return Ok(parse_rss(channel, url));
    }

    // Try parsing as Atom
    let response = reqwest::blocking::get(url)
        .map_err(|e| format!("Failed to fetch feed: {}", e))?;
    let reader = BufReader::new(response);
    if let Ok(feed) = AtomFeed::read_from(reader) {
        return Ok(parse_atom(feed, url));
    }

    Err("Failed to parse feed as RSS or Atom".to_string())
}

fn parse_rss(channel: Channel, url: &str) -> Podcast {
    let episodes: Vec<Episode> = channel
        .items()
        .iter()
        .map(|item| {
            let duration = item
                .itunes_ext()
                .and_then(|ext| ext.duration())
                .and_then(|d| parse_duration(d));

            let audio_url = item
                .enclosure()
                .map(|e| e.url().to_string())
                .unwrap_or_default();

            Episode {
                title: item.title().unwrap_or("Untitled").to_string(),
                description: item.description().unwrap_or("").to_string(),
                published: item.pub_date().unwrap_or("Unknown").to_string(),
                duration,
                audio_url,
                played: false,
            }
        })
        .collect();

    Podcast {
        title: channel.title().to_string(),
        description: channel.description().to_string(),
        url: url.to_string(),
        episodes,
    }
}

fn parse_atom(feed: AtomFeed, url: &str) -> Podcast {
    let episodes: Vec<Episode> = feed
        .entries()
        .iter()
        .map(|entry| {
            // Try to extract audio URL from content or links
            let audio_url = entry
                .content()
                .and_then(|c| c.value())
                .and_then(|v| extract_audio_url_from_html(v))
                .or_else(|| {
                    entry.links().iter()
                        .find(|l| l.rel() == "enclosure" || l.mime_type().map(|m| m.starts_with("audio/")).unwrap_or(false))
                        .map(|l| l.href().to_string())
                })
                .unwrap_or_default();

            let published = entry
                .published()
                .or(Some(entry.updated()))
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "Unknown".to_string());

            Episode {
                title: entry.title().value.clone(),
                description: entry.summary().map(|s| s.value.clone()).unwrap_or_default(),
                published,
                duration: None, // Atom feeds don't typically have duration
                audio_url,
                played: false,
            }
        })
        .collect();

    Podcast {
        title: feed.title().value.clone(),
        description: feed.subtitle().map(|s| s.value.clone()).unwrap_or_default(),
        url: url.to_string(),
        episodes,
    }
}

fn extract_audio_url_from_html(html: &str) -> Option<String> {
    // Simple extraction - look for .m4a or .mp3 URLs
    html.split('"')
        .find(|s| s.contains(".m4a") || s.contains(".mp3"))
        .map(|s| s.to_string())
}

fn parse_duration(duration_str: &str) -> Option<Duration> {
    // Parse iTunes duration format (HH:MM:SS or MM:SS or seconds)
    let parts: Vec<&str> = duration_str.split(':').collect();
    
    let seconds = match parts.len() {
        1 => parts[0].parse::<u64>().ok()?,
        2 => {
            let mins = parts[0].parse::<u64>().ok()?;
            let secs = parts[1].parse::<u64>().ok()?;
            mins * 60 + secs
        }
        3 => {
            let hours = parts[0].parse::<u64>().ok()?;
            let mins = parts[1].parse::<u64>().ok()?;
            let secs = parts[2].parse::<u64>().ok()?;
            hours * 3600 + mins * 60 + secs
        }
        _ => return None,
    };
    
    Some(Duration::from_secs(seconds))
}
