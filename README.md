# shellcast

<img alt="A scallop shell listening to podcasts on headphones" src="logo.png"
  width="300" style="text-align: center;" />

A terminal-based podcast player written in Rust.

## Features

- **Podcast Discovery** - Browse and search thousands of podcasts via gpodder.net (free, no API keys required)
- **Smart Deduplication** - Search results deduplicated by title and hostname, sorted by popularity
- **Feed Management** - Subscribe to podcast RSS/Atom feeds (RSS and Atom format support)
- **Episode Browser** - Browse episodes with publish dates in a clean two-pane TUI
- **Episode Info** - View full episode descriptions and metadata in popup (i key)
- **Chapter Support** - Navigate podcast chapters with timestamps (Podcasting 2.0 spec)
- **Help Screen** - Built-in keybindings reference (? key)
- **Audio Playback** - Stream and play podcast episodes with seek controls (±30s)
- **Played Status** - Mark episodes as played/unplayed, synced to disk
- **Persistence** - Subscriptions and playback state saved automatically
- **TUI Interface** - Clean terminal interface using ratatui
- **Vim-style Navigation** - j/k for navigation, g/G for top/bottom
- **Keybindings** - Intuitive controls for all features
- **Responsive** - 50ms event polling for snappy keyboard input

## Development Status

Active development - core features implemented and working. See TODO section for planned enhancements.

## Recent Changes

### Latest - Chapter Support (2025-11-22)
- **Podcasting 2.0 Chapters** - Full support for podcast chapters with timestamps
- **Chapter Navigation** - Press `c` to view chapter list, `j/k` to navigate, Enter to jump
- **Chapter Caching** - Chapters fetched once and cached for instant navigation
- **Debug Logging** - Added detailed logging to `shellcast-debug.log` for feed parsing issues
- **Feed Error Handling** - Better error messages when feeds fail to parse

### Help, Info & Dates (2025-11-16)
- **Help Screen** - Press `?` to view comprehensive keybindings reference
- **Episode Info Popup** - Press `i` to view full episode description, publish date, and duration
- **Publish Dates** - Episode list now shows publication dates for each episode
- **Popup Navigation** - All popups close with `Esc` or their toggle key

### Browse & Search (2025-11-16)
- **Browse Mode** - Press `5` to discover new podcasts
- **Podcast Search** - Search via gpodder.net's free API (no authentication required)
- **Smart Deduplication** - Results deduplicated by normalized title + hostname
- **Subscriber Rankings** - Results sorted by popularity, subscriber counts shown
- **Auto-subscribe** - Press Enter on search results to instantly subscribe
- **Improved Responsiveness** - Event polling reduced from 200ms to 50ms for snappier input
- **Seek Controls** - Jump forward/backward 30 seconds during playback (h/l or arrow keys)

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
2. **Discover podcasts** - Press `5` to enter Browse mode, then `/` to search
3. **Subscribe** - Press `Enter` on a search result to subscribe
4. **Manual add** - Press `a` to add a podcast feed by URL
5. Use `j/k` or arrow keys to navigate between podcasts and episodes
6. Press `Tab` to switch between the podcast list and episode list
7. Press `Space` to play an episode
8. Press `h/l` or arrow keys to seek backward/forward 30 seconds
9. Press `m` to mark episodes as played/unplayed
10. Press `1` to return to Podcasts view, `5` for Browse
11. Press `q` to quit

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

### Navigation
- `j/k` or Arrow Keys - Navigate up/down in lists
- `g/G` - Jump to top/bottom of list
- `Tab` - Switch focus between podcast list and episode list

### Screen Switching
- `1` - Switch to Podcasts view
- `5` - Switch to Browse/Search view

### Browse Mode
- `/` - Start searching (when in Browse mode)
- `Enter` - Subscribe to selected search result

### Playback
- `Space` - Play/pause selected episode
- `s` - Stop playback
- `h` or Left Arrow - Seek backward 30 seconds
- `l` or Right Arrow - Seek forward 30 seconds

### Management
- `m` - Mark episode as played/unplayed
- `a` - Add new podcast feed (enter URL)
- `d` - Delete selected podcast

### Help & Info
- `?` - Show help screen with all keybindings
- `i` - Show episode info/description popup
- `c` - Show episode chapters (if available)
- `Esc` - Close popups
- `q` - Quit application

## TODO

### Core Features (Implemented ✓)
- [x] Feed subscription management
- [x] Parse RSS/Atom feeds
- [x] Extract audio URLs from feeds
- [x] HTTP streaming player
- [x] Episode list UI
- [x] Feed browser UI
- [x] Playback controls (play/pause/stop)
- [x] Seek controls (forward/backward 30s)
- [x] Played/unplayed tracking
- [x] Feed storage (JSON-based)
- [x] Keybindings system
- [x] Actions pattern
- [x] Persistence
- [x] **Podcast discovery and search** (gpodder.net integration)
- [x] **Smart deduplication** (by title + hostname)
- [x] **Subscriber rankings** (sorted search results)
- [x] **Help screen** (comprehensive keybindings reference)
- [x] **Episode info popup** (view descriptions and metadata)
- [x] **Publish dates** (shown in episode list)
- [x] **Chapter support** (Podcasting 2.0 chapters with navigation)

### In Progress
- [ ] Better error handling and user feedback
- [ ] Resume playback where you left off

### Planned Enhancements
- [ ] Episode download manager for offline listening
- [ ] Speed control (1.5x, 2x playback)
- [ ] Episode queue
- [ ] Episode artwork display
- [ ] OPML import/export
- [ ] Theming system
- [ ] Better buffering status in UI
- [ ] Podcast refresh/update functionality
- [ ] Filter episodes (show unplayed only)
- [ ] Auto-mark as played when episode finishes

## Related Projects

- [shelltrax](https://github.com/yourusername/shelltrax) - Terminal music player (sister project)

## License

MIT OR Apache-2.0
