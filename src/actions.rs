use crate::app::App;
use crate::playback::Player;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    // Navigation
    MoveUp,
    MoveDown,
    GoToTop,
    GoToBottom,

    // Focus
    SwitchFocus,

    // Feed Management
    AddFeed,
    DeletePodcast,
    RefreshFeed,

    // Episode Management
    TogglePlayed,

    // Playback
    PlayPause,
    Stop,
    SeekForward,
    SeekBackward,

    // Screen/Mode
    SwitchToBrowse,
    SwitchToPodcasts,

    // Browse
    #[allow(dead_code)]
    StartSearch,
    #[allow(dead_code)]
    SubscribeFromBrowse,

    // Global
    Quit,
    ShowHelp,
    ShowInfo,
    ShowChapters,
}

impl Action {
    pub fn execute(&self, app: &mut App, player: &Player) {
        match self {
            Action::Quit => {
                // Handled in main loop
            }
            Action::MoveUp => {
                match app.screen {
                    crate::app::AppScreen::Browse => app.browse.move_up(),
                    crate::app::AppScreen::Podcasts => {
                        match app.focus {
                            crate::app::PaneFocus::Left => app.move_podcast_up(),
                            crate::app::PaneFocus::Right => app.move_episode_up(),
                        }
                    }
                }
            }
            Action::MoveDown => {
                match app.screen {
                    crate::app::AppScreen::Browse => app.browse.move_down(),
                    crate::app::AppScreen::Podcasts => {
                        match app.focus {
                            crate::app::PaneFocus::Left => app.move_podcast_down(),
                            crate::app::PaneFocus::Right => app.move_episode_down(),
                        }
                    }
                }
            }
            Action::GoToTop => {
                match app.focus {
                    crate::app::PaneFocus::Left => app.selected_podcast_index = 0,
                    crate::app::PaneFocus::Right => app.selected_episode_index = 0,
                }
            }
            Action::GoToBottom => {
                match app.focus {
                    crate::app::PaneFocus::Left => {
                        app.selected_podcast_index = app.podcasts.len().saturating_sub(1);
                    }
                    crate::app::PaneFocus::Right => {
                        if let Some(podcast) = app.selected_podcast() {
                            app.selected_episode_index = podcast.episodes.len().saturating_sub(1);
                        }
                    }
                }
            }
            Action::SwitchFocus => {
                app.switch_focus();
            }
            Action::AddFeed => {
                app.start_add_feed();
            }
            Action::DeletePodcast => {
                app.delete_podcast();
            }
            Action::RefreshFeed => {
                // Handled in main loop (needs feed fetching)
            }
            Action::TogglePlayed => {
                app.toggle_played();
            }
            Action::PlayPause => {
                // Handled in main loop (needs player reference)
            }
            Action::Stop => {
                // Handled in main loop (needs player reference)
            }
            Action::SeekForward => {
                // Handled in main loop (needs player reference)
            }
            Action::SeekBackward => {
                // Handled in main loop (needs player reference)
            }
            Action::SwitchToBrowse => {
                app.screen = crate::app::AppScreen::Browse;
            }
            Action::SwitchToPodcasts => {
                app.screen = crate::app::AppScreen::Podcasts;
            }
            Action::StartSearch => {
                app.start_search();
            }
            Action::SubscribeFromBrowse => {
                // Handled in main loop (needs feed fetching)
            }
            Action::ShowHelp => {
                app.show_help = !app.show_help;
            }
            Action::ShowInfo => {
                app.show_info = !app.show_info;
            }
            Action::ShowChapters => {
                let audio_path = player.get_temp_file_path();
                app.toggle_chapters(audio_path.as_deref());
            }
        }
    }
}
