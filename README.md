# shellcast

![A scallop shell listening to podcasts on headphones](logo.png)

A terminal-based podcast player written in Rust.

## Features

- **Feed Management** - Subscribe to podcast RSS/Atom feeds (RSS and Atom format support)
- **Episode Browser** - Browse episodes in a clean two-pane TUI
- **Audio Playback** - Stream and play podcast episodes
- **Played Status** - Mark episodes as played/unplayed, synced to disk
- **Persistence** - Subscriptions and playback state saved automatically
- **TUI Interface** - Clean terminal interface using ratatui
- **Vim-style Navigation** - j/k for navigation, g/G for top/bottom
- **Keybindings** - Intuitive controls for all features

## Development Status

Active development - core features implemented and working. See TODO section for planned enhancements.

## Recent Changes

### Latest (In Progress)
- Footer UI improvements - moving status messages and input prompts from centered popups to footer notifications panel
- Refining layout for better information visibility

### v20251115.3
- Added played/unplayed status tracking with 'm' keybinding
- Status persists across sessions
- Visual indicators (● for unplayed, ○ for played)
- Unplayed episode counts shown in podcast list

### Earlier Releases
- GitHub Actions workflow for automated releases
- Audio playback with play/pause/stop controls
- RSS and Atom feed parsing
- Two-pane TUI with podcast and episode lists
- Automatic persistence of subscriptions
- Vim-style navigation keybindings

## Installation

### Download Pre-built Binary

Download the latest release from the [Releases page](https://github.com/ducks/shellcast/releases).

### Build from Source

```bash
cargo build --release
```

The binary will be available at `target/release/shellcast`.

## Usage

1. Run `shellcast` to start the application
2. Press `a` to add your first podcast feed (enter the RSS/Atom feed URL)
3. Use `j/k` or arrow keys to navigate between podcasts and episodes
4. Press `Tab` to switch between the podcast list and episode list
5. Press `Space` to play an episode
6. Press `m` to mark episodes as played/unplayed
7. Press `q` to quit

Podcasts and playback status are automatically saved to `~/.config/shellcast/podcasts.json`.

## Testing

```bash
cargo test
```

## Development Environment

If using Nix:

```bash
nix-shell
```

## Keybindings

- `j/k` or Arrow Keys - Navigate up/down in lists
- `g/G` - Jump to top/bottom of list
- `Tab` - Switch focus between podcast list and episode list
- `Space` - Play/pause selected episode
- `s` - Stop playback
- `m` - Mark episode as played/unplayed
- `a` - Add new podcast feed (enter URL)
- `d` - Delete selected podcast
- `q` - Quit

## TODO

### Core Features (Implemented ✓)
- [x] Feed subscription management
- [x] Parse RSS/Atom feeds
- [x] Extract audio URLs from feeds
- [x] HTTP streaming player
- [x] Episode list UI
- [x] Feed browser UI
- [x] Playback controls (play/pause/stop)
- [x] Played/unplayed tracking
- [x] Feed storage (JSON-based)
- [x] Keybindings system
- [x] Actions pattern
- [x] Persistence

### In Progress
- [ ] Footer UI refinements (notifications panel)
- [ ] Better error handling and user feedback

### Planned Enhancements
- [ ] Resume playback where you left off
- [ ] Seek controls (forward/backward)
- [ ] Episode download manager for offline listening
- [ ] Search functionality
- [ ] Speed control (1.5x, 2x playback)
- [ ] Episode queue
- [ ] Show notes viewer
- [ ] Episode artwork display
- [ ] OPML import/export
- [ ] Chapter support
- [ ] Theming system
- [ ] Better buffering status in UI

## Related Projects

- [shelltrax](https://github.com/yourusername/shelltrax) - Terminal music player (sister project)

## License

MIT OR Apache-2.0
