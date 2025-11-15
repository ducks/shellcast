# shellcast

A terminal-based podcast player written in Rust.

## Features (Planned)

- **Feed Management** - Subscribe to podcast RSS/Atom feeds
- **Episode Browser** - Browse and search episodes
- **Streaming Playback** - Stream episodes directly without downloading
- **Progress Tracking** - Resume playback where you left off
- **Offline Mode** - Download episodes for offline listening
- **Search** - Find podcasts and episodes
- **TUI Interface** - Clean terminal interface using ratatui

## Development Status

Early development - basic feed parsing works, player not yet implemented.

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```

## Development Environment

If using Nix:

```bash
nix-shell
```

## TODO

### Core Features
- [ ] Feed subscription management
- [ ] Parse RSS/Atom feeds (basic parsing done)
- [ ] Extract audio URLs from feeds
- [ ] HTTP streaming player
- [ ] Episode list UI
- [ ] Feed browser UI
- [ ] Playback controls (play/pause/seek)
- [ ] Progress persistence
- [ ] Episode download manager
- [ ] Search functionality

### Architecture
- [ ] Design app state structure
- [ ] Copy keybindings system from shelltrax
- [ ] Copy actions pattern from shelltrax
- [ ] Copy theming system from shelltrax
- [ ] Implement feed storage (SQLite or JSON?)
- [ ] Implement progress tracking storage

### Streaming
- [ ] Modify player to accept HTTP stream
- [ ] Buffer management for streaming
- [ ] Network error handling and retry logic
- [ ] Show buffering status in UI

### Nice to Have
- [ ] Speed control (1.5x, 2x playback)
- [ ] Episode queue
- [ ] Show notes viewer
- [ ] Episode artwork display
- [ ] OPML import/export
- [ ] Chapter support

## Related Projects

- [shelltrax](https://github.com/yourusername/shelltrax) - Terminal music player (sister project)

## License

MIT OR Apache-2.0
