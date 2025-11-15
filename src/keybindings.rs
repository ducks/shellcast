use crossterm::event::{KeyCode, KeyModifiers};
use std::collections::HashMap;
use crate::actions::Action;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeyBinding {
    pub fn new(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::empty(),
        }
    }
}

pub struct KeyMap {
    bindings: HashMap<KeyBinding, Action>,
}

impl KeyMap {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn with_defaults() -> Self {
        let mut map = Self::new();
        map.load_defaults();
        map
    }

    pub fn bind(&mut self, key: KeyBinding, action: Action) {
        self.bindings.insert(key, action);
    }

    pub fn get_action(&self, key: &KeyBinding) -> Option<&Action> {
        self.bindings.get(key)
    }

    pub fn load_defaults(&mut self) {
        // Global
        self.bind(KeyBinding::new(KeyCode::Char('q')), Action::Quit);

        // Navigation - Arrow keys
        self.bind(KeyBinding::new(KeyCode::Up), Action::MoveUp);
        self.bind(KeyBinding::new(KeyCode::Down), Action::MoveDown);

        // Navigation - Vim keys
        self.bind(KeyBinding::new(KeyCode::Char('k')), Action::MoveUp);
        self.bind(KeyBinding::new(KeyCode::Char('j')), Action::MoveDown);
        self.bind(KeyBinding::new(KeyCode::Char('g')), Action::GoToTop);
        self.bind(KeyBinding::new(KeyCode::Char('G')), Action::GoToBottom);

        // Focus
        self.bind(KeyBinding::new(KeyCode::Tab), Action::SwitchFocus);

        // Feed Management
        self.bind(KeyBinding::new(KeyCode::Char('a')), Action::AddFeed);
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        Self::with_defaults()
    }
}
