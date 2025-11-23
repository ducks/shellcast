use crate::app::{Episode, Podcast};
use atom_syndication::Feed as AtomFeed;
use rss::Channel;
use std::io::BufReader;
use std::time::Duration;

pub fn fetch_and_parse(url: &str) -> Result<Podcast, String> {
    log::debug!("Fetching feed from: {}", url);

    let response = reqwest::blocking::get(url)
        .map_err(|e| {
            log::error!("Failed to fetch feed {}: {}", url, e);
            format!("Failed to fetch feed: {}", e)
        })?;

    log::debug!("Feed fetched successfully, attempting RSS parse...");

    // Try parsing as RSS first
    let reader = BufReader::new(response);
    match Channel::read_from(reader) {
        Ok(channel) => {
            log::debug!("Successfully parsed as RSS feed");
            return Ok(parse_rss(channel, url));
        }
        Err(e) => {
            log::debug!("RSS parse failed: {}", e);
        }
    }

    // Try parsing as Atom
    log::debug!("Attempting Atom parse...");
    let response = reqwest::blocking::get(url)
        .map_err(|e| {
            log::error!("Failed to re-fetch feed for Atom: {}", e);
            format!("Failed to fetch feed: {}", e)
        })?;
    let reader = BufReader::new(response);
    match AtomFeed::read_from(reader) {
        Ok(feed) => {
            log::debug!("Successfully parsed as Atom feed");
            return Ok(parse_atom(feed, url));
        }
        Err(e) => {
            log::error!("Atom parse failed: {}", e);
        }
    }

    Err("Failed to parse feed as RSS or Atom. Check debug.log for details.".to_string())
}

/// Refresh a podcast feed, preserving played status of existing episodes
pub fn refresh_feed(podcast: &mut Podcast) -> Result<usize, String> {
    // Fetch fresh data
    let fresh = fetch_and_parse(&podcast.url)?;

    // Build a map of audio_url -> played status from existing episodes
    let played_map: std::collections::HashMap<String, bool> = podcast
        .episodes
        .iter()
        .map(|ep| (ep.audio_url.clone(), ep.played))
        .collect();

    // Count new episodes
    let old_count = podcast.episodes.len();

    // Update podcast metadata
    podcast.title = fresh.title;
    podcast.description = fresh.description;

    // Merge episodes, preserving played status
    podcast.episodes = fresh
        .episodes
        .into_iter()
        .map(|mut ep| {
            // Preserve played status if we've seen this episode before
            if let Some(&played) = played_map.get(&ep.audio_url) {
                ep.played = played;
            }
            ep
        })
        .collect();

    let new_count = podcast.episodes.len();
    let added = new_count.saturating_sub(old_count);

    Ok(added)
}

fn parse_rss(channel: Channel, url: &str) -> Podcast {
    log::debug!("Parsing RSS feed: {}, found {} episodes", channel.title(), channel.items().len());

    let episodes: Vec<Episode> = channel
        .items()
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            // Debug: Log extensions for first episode
            if idx == 0 {
                log::debug!("Extensions in first episode:");
                for (namespace, extensions) in item.extensions() {
                    log::debug!("  Namespace: {}", namespace);
                    for (name, values) in extensions {
                        log::debug!("    Tag: {} (count: {})", name, values.len());
                        for val in values {
                            log::debug!("      Attrs: {:?}", val.attrs);
                            if let Some(text) = &val.value {
                                log::debug!("      Value: {}", text);
                            }
                        }
                    }
                }
            }

            let duration = item
                .itunes_ext()
                .and_then(|ext| ext.duration())
                .and_then(parse_duration);

            let audio_url = item
                .enclosure()
                .map(|e| e.url().to_string())
                .unwrap_or_default();

            // Extract podcast:chapters URL from extensions
            let chapters_url = item
                .extensions()
                .get("podcast")
                .and_then(|podcast_ext| podcast_ext.get("chapters"))
                .and_then(|chapters| chapters.first())
                .and_then(|chapter_elem| chapter_elem.attrs.get("url"))
                .map(|s| s.to_string());

            if let Some(ref chapters) = chapters_url {
                log::debug!("Found chapters URL for '{}': {}", item.title().unwrap_or("Unknown"), chapters);
            }

            Episode {
                title: item.title().unwrap_or("Untitled").to_string(),
                description: item.description().unwrap_or("").to_string(),
                published: item.pub_date().unwrap_or("Unknown").to_string(),
                duration,
                audio_url,
                played: false,
                chapters_url,
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
                .and_then(extract_audio_url_from_html)
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
                chapters_url: None, // Atom feeds don't typically have chapters
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
