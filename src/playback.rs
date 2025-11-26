use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use tempfile::NamedTempFile;

pub struct Player {
    temp_file: Option<NamedTempFile>,
    sink: Arc<Mutex<Option<rodio::Sink>>>,
    stream_handle: rodio::OutputStream,
    // Keep the response body alive for streaming
    _http_body: Option<Box<dyn std::io::Read + Send>>,
}

impl Player {
    pub fn new() -> Result<Self, String> {
        // Create the output stream once at initialization
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .map_err(|e| format!("Failed to open audio stream: {}", e))?;

        Ok(Self {
            temp_file: None,
            sink: Arc::new(Mutex::new(None)),
            stream_handle,
            _http_body: None,
        })
    }

    pub fn play(&mut self, url: &str) -> Result<(), String> {
        // Stop any current playback
        self.stop();

        // Check if this is an M3U playlist URL - just remove that parameter for now
        let actual_url = if url.contains("metafile=m3u") {
            url.replace("&metafile=m3u", "")
        } else {
            url.to_string()
        };

        log::info!("Starting playback from URL: {}", actual_url);

        // Create temp file first
        let temp_file = NamedTempFile::new()
            .map_err(|e| format!("Failed to create temp file: {}", e))?;
        let temp_path = temp_file.path().to_path_buf();
        let temp_path_for_playback = temp_path.clone();

        // Clone mixer and sink for background thread
        let mixer = self.stream_handle.mixer().clone();
        let sink_arc = self.sink.clone();

        // Spawn background thread to download and start playback
        std::thread::spawn(move || {
            use std::io::{Read, Write};

            log::info!("Background: Fetching audio...");
            let mut response = match reqwest::blocking::get(&actual_url) {
                Ok(r) => r,
                Err(e) => {
                    log::error!("Background: Failed to fetch audio: {}", e);
                    return;
                }
            };

            // Buffer 10MB before starting playback
            let buffer_size = 10 * 1024 * 1024; // 10MB
            let mut initial_buffer = Vec::with_capacity(buffer_size);
            let mut total_read = 0;

            let mut chunk_buffer = vec![0u8; 8192];
            while total_read < buffer_size {
                match response.read(&mut chunk_buffer) {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        initial_buffer.extend_from_slice(&chunk_buffer[..n]);
                        total_read += n;
                    }
                    Err(e) => {
                        log::error!("Background: Failed to buffer audio: {}", e);
                        return;
                    }
                }
            }

            log::info!("Background: Initial buffer complete: {} bytes", total_read);

            // Write initial buffer to file
            let mut file = match File::create(&temp_path) {
                Ok(f) => f,
                Err(e) => {
                    log::error!("Background: Failed to create temp file: {}", e);
                    return;
                }
            };
            if let Err(e) = file.write_all(&initial_buffer) {
                log::error!("Background: Failed to write initial buffer: {}", e);
                return;
            }

            // Start playback from the buffered file
            let playback_file = match File::open(&temp_path_for_playback) {
                Ok(f) => f,
                Err(e) => {
                    log::error!("Background: Failed to open temp file for playback: {}", e);
                    return;
                }
            };

            let source = match rodio::Decoder::new(BufReader::new(playback_file)) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("Background: Failed to decode audio: {}", e);
                    return;
                }
            };

            let new_sink = rodio::Sink::connect_new(&mixer);
            new_sink.append(source);

            // Store the sink in the shared Arc<Mutex<>>
            if let Ok(mut sink_guard) = sink_arc.lock() {
                *sink_guard = Some(new_sink);
            }

            log::info!("Background: Playback started, continuing download...");

            // Continue downloading the rest in this same thread
            let mut file = match std::fs::OpenOptions::new().append(true).open(&temp_path) {
                Ok(f) => f,
                Err(e) => {
                    log::error!("Background: Failed to open file for appending: {}", e);
                    return;
                }
            };

            loop {
                match response.read(&mut chunk_buffer) {
                    Ok(0) => {
                        log::info!("Background: Download complete");
                        break;
                    }
                    Ok(n) => {
                        if let Err(e) = file.write_all(&chunk_buffer[..n]) {
                            log::error!("Background: Failed to write chunk: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        log::error!("Background: Failed to read chunk: {}", e);
                        break;
                    }
                }
            }
        });

        // Store temp file reference (but playback happens in background thread)
        self.temp_file = Some(temp_file);

        log::info!("Playback thread spawned, UI remains responsive");

        Ok(())
    }

    pub fn pause(&self) {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                sink.pause();
            }
        }
    }

    pub fn resume(&self) {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                sink.play();
            }
        }
    }

    pub fn stop(&mut self) {
        if let Ok(mut sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.take() {
                sink.stop();
            }
        }
        // Keep stream_handle alive, only drop sink and temp file
        self.temp_file = None;
    }

    pub fn is_playing(&self) -> bool {
        if let Ok(sink_guard) = self.sink.lock() {
            sink_guard
                .as_ref()
                .map(|s| !s.is_paused() && !s.empty())
                .unwrap_or(false)
        } else {
            false
        }
    }

    pub fn is_paused(&self) -> bool {
        if let Ok(sink_guard) = self.sink.lock() {
            sink_guard
                .as_ref()
                .map(|s| s.is_paused())
                .unwrap_or(false)
        } else {
            false
        }
    }

    pub fn seek_forward(&self, seconds: u64) -> Result<(), String> {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                let current_pos = sink.get_pos();
                let new_pos = current_pos + std::time::Duration::from_secs(seconds);
                sink.try_seek(new_pos)
                    .map_err(|e| format!("Seek failed: {}", e))?;
            }
        }
        Ok(())
    }

    pub fn seek_backward(&self, seconds: u64) -> Result<(), String> {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                let current_pos = sink.get_pos();
                let new_pos = current_pos.saturating_sub(std::time::Duration::from_secs(seconds));
                sink.try_seek(new_pos)
                    .map_err(|e| format!("Seek failed: {}", e))?;
            }
        }
        Ok(())
    }

    pub fn get_position(&self) -> std::time::Duration {
        if let Ok(sink_guard) = self.sink.lock() {
            sink_guard
                .as_ref()
                .map(|s| s.get_pos())
                .unwrap_or(std::time::Duration::ZERO)
        } else {
            std::time::Duration::ZERO
        }
    }

    pub fn get_temp_file_path(&self) -> Option<std::path::PathBuf> {
        self.temp_file.as_ref().map(|f| f.path().to_path_buf())
    }
}
