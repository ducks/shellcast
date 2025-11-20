use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub author: String,
    pub description: String,
    pub feed_url: String,
    pub artwork_url: Option<String>,
    pub subscribers: u64,
}

pub struct BrowseState {
    pub search_query: String,
    pub search_results: Vec<SearchResult>,
    pub selected_index: usize,
    pub is_searching: bool,
    pub showing_defaults: bool,
}

impl BrowseState {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            search_results: get_default_podcasts(),
            selected_index: 0,
            is_searching: false,
            showing_defaults: true,
        }
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index < self.search_results.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn selected_result(&self) -> Option<&SearchResult> {
        self.search_results.get(self.selected_index)
    }
}

/// Returns a curated list of popular/featured podcasts to show by default
pub fn get_default_podcasts() -> Vec<SearchResult> {
    vec![
        SearchResult {
            title: "This American Life".to_string(),
            author: "This American Life".to_string(),
            description: "This American Life is a weekly public radio show, heard by 2.2 million people on more than 500 stations.".to_string(),
            feed_url: "https://www.thisamericanlife.org/podcast/rss.xml".to_string(),
            artwork_url: None,
            subscribers: 1000000,
        },
        SearchResult {
            title: "Radiolab".to_string(),
            author: "WNYC Studios".to_string(),
            description: "A two-time Peabody Award-winner, Radiolab is an investigation told through sounds and stories.".to_string(),
            feed_url: "https://feeds.wnyc.org/radiolab".to_string(),
            artwork_url: None,
            subscribers: 950000,
        },
        SearchResult {
            title: "99% Invisible".to_string(),
            author: "Roman Mars".to_string(),
            description: "Design is everywhere in our lives, perhaps most importantly in the places where we've just stopped noticing.".to_string(),
            feed_url: "https://feeds.99percentinvisible.org/99percentinvisible".to_string(),
            artwork_url: None,
            subscribers: 900000,
        },
        SearchResult {
            title: "Planet Money".to_string(),
            author: "NPR".to_string(),
            description: "The economy explained. Imagine you could call up a friend and say, 'Meet me at the bar and tell me what's going on with the economy.'".to_string(),
            feed_url: "https://feeds.npr.org/510289/podcast.xml".to_string(),
            artwork_url: None,
            subscribers: 850000,
        },
        SearchResult {
            title: "Reply All".to_string(),
            author: "Gimlet".to_string(),
            description: "A show about the internet that is actually an unfailingly original exploration of modern life.".to_string(),
            feed_url: "https://feeds.megaphone.fm/replyall".to_string(),
            artwork_url: None,
            subscribers: 800000,
        },
        SearchResult {
            title: "The Daily".to_string(),
            author: "The New York Times".to_string(),
            description: "This is what the news should sound like. The biggest stories of our time, told by the best journalists in the world.".to_string(),
            feed_url: "https://feeds.simplecast.com/54nAGcIl".to_string(),
            artwork_url: None,
            subscribers: 750000,
        },
        SearchResult {
            title: "Hardcore History".to_string(),
            author: "Dan Carlin".to_string(),
            description: "In 'Hardcore History' journalist and broadcaster Dan Carlin takes his 'Martian', unorthodox way of thinking and applies it to the past.".to_string(),
            feed_url: "https://feeds.feedburner.com/dancarlin/history".to_string(),
            artwork_url: None,
            subscribers: 700000,
        },
        SearchResult {
            title: "Freakonomics Radio".to_string(),
            author: "Freakonomics Radio + Stitcher".to_string(),
            description: "Discover the hidden side of everything with Stephen J. Dubner, co-author of the Freakonomics books.".to_string(),
            feed_url: "https://feeds.simplecast.com/Y8lFbOT4".to_string(),
            artwork_url: None,
            subscribers: 650000,
        },
    ]
}

/// Normalize title for deduplication (lowercase, remove extra chars)
fn normalize_title(title: &str) -> String {
    title.to_lowercase()
        .replace("podcasts", "")
        .replace("podcast", "")
        .replace("(", "")
        .replace(")", "")
        .trim()
        .to_string()
}

/// Extract hostname from URL
fn get_hostname(url: &str) -> Option<String> {
    url.split("://")
        .nth(1)?
        .split('/')
        .next()
        .map(|s| s.to_string())
}

/// Search podcasts using gpodder.net API (free, open, no auth required)
pub fn search_podcasts(query: &str) -> Result<Vec<SearchResult>, String> {
    if query.is_empty() {
        return Ok(Vec::new());
    }

    // gpodder.net search API - completely free and open
    let url = format!("https://gpodder.net/search.json?q={}",
        urlencoding::encode(query));

    let response = reqwest::blocking::get(&url)
        .map_err(|e| format!("Search failed: {}", e))?;

    let json: serde_json::Value = response.json()
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let results_array = json.as_array()
        .ok_or("Invalid response format")?;

    let all_results: Vec<SearchResult> = results_array.iter()
        .filter_map(|item| {
            Some(SearchResult {
                title: item["title"].as_str()?.to_string(),
                author: item["author"].as_str().unwrap_or("Unknown").to_string(),
                description: item["description"].as_str().unwrap_or("").to_string(),
                feed_url: item["url"].as_str()?.to_string(),
                artwork_url: item["logo_url"].as_str().map(String::from),
                subscribers: item["subscribers"].as_u64().unwrap_or(0),
            })
        })
        .collect();

    // Deduplicate by normalized title + hostname, keeping the one with most subscribers
    let mut deduped: HashMap<(String, String), SearchResult> = HashMap::new();

    for result in all_results {
        if let Some(hostname) = get_hostname(&result.feed_url) {
            let key = (normalize_title(&result.title), hostname);

            deduped.entry(key)
                .and_modify(|existing| {
                    // Keep the one with more subscribers
                    if result.subscribers > existing.subscribers {
                        *existing = result.clone();
                    }
                })
                .or_insert(result);
        }
    }

    // Convert back to Vec and sort by subscriber count (descending)
    let mut results: Vec<SearchResult> = deduped.into_values().collect();
    results.sort_by(|a, b| b.subscribers.cmp(&a.subscribers));
    results.truncate(20); // Limit to 20 results

    Ok(results)
}
