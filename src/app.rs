use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use crate::browse::BrowseState;
use crate::chapters::ChapterList;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Podcast {
    pub title: String,
    pub description: String,
    pub url: String,
    pub episodes: Vec<Episode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub title: String,
    pub description: String,
    pub published: String,
    #[serde(with = "option_duration")]
    pub duration: Option<Duration>,
    pub audio_url: String,
    pub played: bool,
    pub chapters_url: Option<String>,
    #[serde(default)]
    pub position_secs: u64,
}

// Custom serialization for Option<Duration>
mod option_duration {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match duration {
            Some(d) => d.as_secs().serialize(serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<u64> = Option::deserialize(deserializer)?;
        Ok(opt.map(Duration::from_secs))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Podcasts,
    Browse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneFocus {
    Left,  // Podcast list
    Right, // Episode list
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    AddingFeed,
    Searching,
}

pub struct PlaybackState {
    pub url: Option<String>,
    pub duration_secs: u64,
    pub start: Option<Instant>,
    pub paused_at: Option<Instant>,
    pub paused_duration: Duration,
}

impl PlaybackState {
    pub fn new() -> Self {
        Self {
            url: None,
            duration_secs: 0,
            start: None,
            paused_at: None,
            paused_duration: Duration::ZERO,
        }
    }
}

pub struct App {
    pub screen: AppScreen,
    pub podcasts: Vec<Podcast>,
    pub needs_save: bool,
    pub selected_podcast_index: usize,
    pub selected_episode_index: usize,
    pub focus: PaneFocus,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub status_message: Option<String>,

    // Playback state
    pub playback: PlaybackState,

    // Browse state
    pub browse: BrowseState,

    // UI state
    pub show_help: bool,
    pub show_info: bool,
    pub show_chapters: bool,
    pub selected_chapter_index: usize,
    pub cached_chapters: Option<ChapterList>,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: AppScreen::Podcasts,
            podcasts: Vec::new(),
            needs_save: false,
            selected_podcast_index: 0,
            selected_episode_index: 0,
            focus: PaneFocus::Left,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            status_message: None,
            playback: PlaybackState::new(),
            browse: BrowseState::new(),
            show_help: false,
            show_info: false,
            show_chapters: false,
            selected_chapter_index: 0,
            cached_chapters: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_dummy_data() -> Self {
        let mut app = Self::new();
        
        // Add some dummy podcasts
        app.podcasts = vec![
            Podcast {
                title: "Radiolab".to_string(),
                description: "Investigating a strange world.".to_string(),
                url: "https://example.com/radiolab".to_string(),
                episodes: vec![
                    Episode {
                        title: "The Secret Life of Latency".to_string(),
                        description: "What happens in the milliseconds between clicking and loading?".to_string(),
                        published: "Nov 15, 2025".to_string(),
                        duration: Some(Duration::from_secs(45 * 60 + 32)),
                        audio_url: "https://example.com/ep1.mp3".to_string(),
                        played: false,
                        chapters_url: None,
                        position_secs: 0,
                    },
                    Episode {
                        title: "The Uncertainty Machine".to_string(),
                        description: "How randomness shapes our world.".to_string(),
                        published: "Nov 8, 2025".to_string(),
                        duration: Some(Duration::from_secs(52 * 60 + 15)),
                        audio_url: "https://example.com/ep2.mp3".to_string(),
                        played: true,
                        chapters_url: None,
                        position_secs: 0,
                    },
                    Episode {
                        title: "Numbers in the Wild".to_string(),
                        description: "Mathematical patterns in nature.".to_string(),
                        published: "Nov 1, 2025".to_string(),
                        duration: Some(Duration::from_secs(38 * 60 + 45)),
                        audio_url: "https://example.com/ep3.mp3".to_string(),
                        played: true,
                        chapters_url: None,
                        position_secs: 0,
                    },
                ],
            },
            Podcast {
                title: "99% Invisible".to_string(),
                description: "Design and architecture stories.".to_string(),
                url: "https://example.com/99pi".to_string(),
                episodes: vec![
                    Episode {
                        title: "The Power of Nothing".to_string(),
                        description: "Why empty space matters in design.".to_string(),
                        published: "Nov 14, 2025".to_string(),
                        duration: Some(Duration::from_secs(28 * 60 + 12)),
                        audio_url: "https://example.com/ep4.mp3".to_string(),
                        played: false,
                        chapters_url: None,
                        position_secs: 0,
                    },
                    Episode {
                        title: "Designed to Last".to_string(),
                        description: "Products built for eternity.".to_string(),
                        published: "Nov 7, 2025".to_string(),
                        duration: Some(Duration::from_secs(32 * 60 + 50)),
                        audio_url: "https://example.com/ep5.mp3".to_string(),
                        played: false,
                        chapters_url: None,
                        position_secs: 0,
                    },
                ],
            },
            Podcast {
                title: "The Daily".to_string(),
                description: "This is what the news should sound like.".to_string(),
                url: "https://example.com/thedaily".to_string(),
                episodes: vec![
                    Episode {
                        title: "Today's Top Stories".to_string(),
                        description: "Breaking news and analysis.".to_string(),
                        published: "Nov 15, 2025".to_string(),
                        duration: Some(Duration::from_secs(25 * 60)),
                        audio_url: "https://example.com/ep6.mp3".to_string(),
                        played: false,
                        chapters_url: None,
                        position_secs: 0,
                    },
                ],
            },
        ];
        
        app
    }

    pub fn is_browse_screen(&self) -> bool {
        matches!(self.screen, AppScreen::Browse)
    }

    pub fn selected_podcast(&self) -> Option<&Podcast> {
        self.podcasts.get(self.selected_podcast_index)
    }

    pub fn selected_episode_url(&self) -> Option<String> {
        self.selected_podcast()
            .and_then(|p| p.episodes.get(self.selected_episode_index))
            .map(|e| e.audio_url.clone())
    }

    pub fn selected_episode_mut(&mut self) -> Option<&mut Episode> {
        let p = self.selected_podcast_index;
        let e = self.selected_episode_index;
        self.podcasts
            .get_mut(p)
            .and_then(|podcast| podcast.episodes.get_mut(e))
    }

    pub fn move_podcast_up(&mut self) {
        if self.selected_podcast_index > 0 {
            self.selected_podcast_index -= 1;
            self.selected_episode_index = 0;
        }
    }

    pub fn move_podcast_down(&mut self) {
        if self.selected_podcast_index < self.podcasts.len().saturating_sub(1) {
            self.selected_podcast_index += 1;
            self.selected_episode_index = 0;
        }
    }

    pub fn move_episode_up(&mut self) {
        if self.selected_episode_index > 0 {
            self.selected_episode_index -= 1;
        }
    }

    pub fn move_episode_down(&mut self) {
        if let Some(podcast) = self.selected_podcast()
            && self.selected_episode_index < podcast.episodes.len().saturating_sub(1) {
                self.selected_episode_index += 1;
            }
    }

    pub fn switch_focus(&mut self) {
        self.focus = match self.focus {
            PaneFocus::Left => PaneFocus::Right,
            PaneFocus::Right => PaneFocus::Left,
        };
    }

    pub fn start_add_feed(&mut self) {
        self.input_mode = InputMode::AddingFeed;
        self.input_buffer.clear();
        self.status_message = None; // Clear any existing status messages
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
    }

    pub fn add_podcast(&mut self, podcast: Podcast) {
        self.podcasts.push(podcast);
        self.selected_podcast_index = self.podcasts.len() - 1;
        self.selected_episode_index = 0;
        self.needs_save = true;
    }

    pub fn delete_podcast(&mut self) {
        if !self.podcasts.is_empty() {
            self.podcasts.remove(self.selected_podcast_index);

            // Adjust selection after deletion
            if self.selected_podcast_index >= self.podcasts.len() && self.selected_podcast_index > 0 {
                self.selected_podcast_index -= 1;
            }
            self.selected_episode_index = 0;
            self.needs_save = true;
        }
    }

    pub fn toggle_played(&mut self) {
        if let Some(episode) = self.selected_episode_mut() {
            episode.played = !episode.played;
            self.needs_save = true;
        }
    }

    pub fn start_search(&mut self) {
        self.input_mode = InputMode::Searching;
        self.browse.is_searching = true;
        self.browse.search_query.clear();
    }

    pub fn cancel_search(&mut self) {
        self.input_mode = InputMode::Normal;
        self.browse.is_searching = false;
    }

    pub fn toggle_chapters(&mut self) {
        self.show_chapters = !self.show_chapters;
        if self.show_chapters {
            self.selected_chapter_index = 0;
            // Always clear cache first to prevent showing stale chapters
            self.cached_chapters = None;

            // Fetch chapters when opening popup
            if let Some(episode) = self.selected_podcast()
                .and_then(|p| p.episodes.get(self.selected_episode_index))
            {
                log::debug!("Opening chapters for episode: {}", episode.title);
                if let Some(chapters_url) = &episode.chapters_url {
                    log::debug!("Found chapters_url: {}", chapters_url);
                    use crate::chapters;
                    match chapters::fetch_chapters(chapters_url) {
                        Ok(chapters) => {
                            log::debug!("Cached {} chapters", chapters.chapters.len());
                            self.cached_chapters = Some(chapters);
                        }
                        Err(e) => {
                            log::error!("Failed to fetch chapters: {}", e);
                            self.cached_chapters = None;
                        }
                    }
                } else {
                    log::debug!("Episode has no chapters_url");
                }
            } else {
                log::error!("Could not find selected episode");
            }
        } else {
            // Clear cache when closing
            self.cached_chapters = None;
        }
    }

    pub fn move_chapter_up(&mut self) {
        if self.selected_chapter_index > 0 {
            self.selected_chapter_index -= 1;
        }
    }

    pub fn move_chapter_down(&mut self, max_index: usize) {
        if self.selected_chapter_index < max_index {
            self.selected_chapter_index += 1;
        }
    }
}
