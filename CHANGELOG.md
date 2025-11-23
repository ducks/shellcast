# Changelog

All notable changes to shellcast will be documented in this file.

## [Unreleased]

### Theming System (2025-11-23)
- **TOML Configuration** - Config file support at `~/.config/shellcast/config.toml`
- **Built-in Themes** - Ten themes included: default, dark, gruvbox, solarized (dark & light), dracula, nord, monokai, tokyo-night, and catppuccin
- **Custom Themes** - Create your own color schemes with full customization
- **Theme Colors** - Customizable selection, borders, text states, popups, and more
- **Example Config** - See `config.example.toml` for all available options

### Chapter Support (2025-11-22)
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

## [v20251115.3] - 2025-11-15

### Added
- Played/unplayed status tracking with 'm' keybinding
- Status persists across sessions
- Visual indicators (● for unplayed, ○ for played)
- Unplayed episode counts shown in podcast list

## Earlier Releases

### Added
- GitHub Actions workflow for automated releases
- Audio playback with play/pause/stop controls
- RSS and Atom feed parsing
- Two-pane TUI with podcast and episode lists
- Automatic persistence of subscriptions
- Vim-style navigation keybindings
