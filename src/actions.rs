use crate::app::App;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    // Navigation
    MoveUp,
    MoveDown,
    GoToTop,
    GoToBottom,
    
    // Focus
    SwitchFocus,
    
    // Global
    Quit,
}

impl Action {
    pub fn execute(&self, app: &mut App) {
        match self {
            Action::Quit => {
                // Handled in main loop
            }
            Action::MoveUp => {
                match app.focus {
                    crate::app::PaneFocus::Left => app.move_podcast_up(),
                    crate::app::PaneFocus::Right => app.move_episode_up(),
                }
            }
            Action::MoveDown => {
                match app.focus {
                    crate::app::PaneFocus::Left => app.move_podcast_down(),
                    crate::app::PaneFocus::Right => app.move_episode_down(),
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
        }
    }
}
