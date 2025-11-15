use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Podcast {
    pub title: String,
    pub description: String,
    pub url: String,
    pub episodes: Vec<Episode>,
}

#[derive(Debug, Clone)]
pub struct Episode {
    pub title: String,
    pub description: String,
    pub published: String,
    pub duration: Option<Duration>,
    pub audio_url: String,
    pub played: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Podcasts,
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
}

pub struct App {
    pub screen: AppScreen,
    pub podcasts: Vec<Podcast>,
    pub selected_podcast_index: usize,
    pub selected_episode_index: usize,
    pub focus: PaneFocus,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub status_message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: AppScreen::Podcasts,
            podcasts: Vec::new(),
            selected_podcast_index: 0,
            selected_episode_index: 0,
            focus: PaneFocus::Left,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            status_message: None,
        }
    }

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
                    },
                    Episode {
                        title: "The Uncertainty Machine".to_string(),
                        description: "How randomness shapes our world.".to_string(),
                        published: "Nov 8, 2025".to_string(),
                        duration: Some(Duration::from_secs(52 * 60 + 15)),
                        audio_url: "https://example.com/ep2.mp3".to_string(),
                        played: true,
                    },
                    Episode {
                        title: "Numbers in the Wild".to_string(),
                        description: "Mathematical patterns in nature.".to_string(),
                        published: "Nov 1, 2025".to_string(),
                        duration: Some(Duration::from_secs(38 * 60 + 45)),
                        audio_url: "https://example.com/ep3.mp3".to_string(),
                        played: true,
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
                    },
                    Episode {
                        title: "Designed to Last".to_string(),
                        description: "Products built for eternity.".to_string(),
                        published: "Nov 7, 2025".to_string(),
                        duration: Some(Duration::from_secs(32 * 60 + 50)),
                        audio_url: "https://example.com/ep5.mp3".to_string(),
                        played: false,
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
                    },
                ],
            },
        ];
        
        app
    }

    pub fn selected_podcast(&self) -> Option<&Podcast> {
        self.podcasts.get(self.selected_podcast_index)
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
        if let Some(podcast) = self.selected_podcast() {
            if self.selected_episode_index < podcast.episodes.len().saturating_sub(1) {
                self.selected_episode_index += 1;
            }
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
        self.status_message = None;
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
    }

    pub fn add_podcast(&mut self, podcast: Podcast) {
        self.podcasts.push(podcast);
        self.selected_podcast_index = self.podcasts.len() - 1;
        self.selected_episode_index = 0;
    }
}
