use std::fs::File;
use std::io::{BufReader, Write as _};
use tempfile::NamedTempFile;

pub struct Player {
    temp_file: Option<NamedTempFile>,
    sink: Option<rodio::Sink>,
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
            sink: None,
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

        // Clone URL for the thread
        let url_for_thread = actual_url.clone();

        // Download first 10MB to buffer, then continue in background
        log::info!("Fetching initial buffer...");
        let mut response = reqwest::blocking::get(&url_for_thread)
            .map_err(|e| format!("Failed to fetch audio: {}", e))?;

        // Buffer 10MB
        let buffer_size = 10 * 1024 * 1024; // 10MB
        let mut initial_buffer = Vec::with_capacity(buffer_size);
        let mut total_read = 0;

        use std::io::Read;
        let mut chunk_buffer = vec![0u8; 8192];
        while total_read < buffer_size {
            match response.read(&mut chunk_buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    initial_buffer.extend_from_slice(&chunk_buffer[..n]);
                    total_read += n;
                }
                Err(e) => return Err(format!("Failed to buffer audio: {}", e)),
            }
        }

        log::info!("Initial buffer complete: {} bytes", total_read);

        // Write initial buffer to file
        let mut file = File::create(temp_file.path())
            .map_err(|e| format!("Failed to create temp file: {}", e))?;
        file.write_all(&initial_buffer)
            .map_err(|e| format!("Failed to write initial buffer: {}", e))?;

        // Spawn background thread to download the rest
        std::thread::spawn(move || {
            log::info!("Background download continuing...");
            let mut file = match std::fs::OpenOptions::new().append(true).open(&temp_path) {
                Ok(f) => f,
                Err(e) => {
                    log::error!("Failed to open file for appending: {}", e);
                    return;
                }
            };

            let mut chunk_buffer = vec![0u8; 8192];
            loop {
                match response.read(&mut chunk_buffer) {
                    Ok(0) => {
                        log::info!("Background download complete");
                        break;
                    }
                    Ok(n) => {
                        if let Err(e) = file.write_all(&chunk_buffer[..n]) {
                            log::error!("Failed to write chunk: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to read chunk: {}", e);
                        break;
                    }
                }
            }
        });

        // Open the file for playback with enough buffered data
        let playback_file = File::open(temp_file.path())
            .map_err(|e| format!("Failed to open temp file: {}", e))?;

        // Decode with seeking support
        let source = rodio::Decoder::new(BufReader::new(playback_file))
            .map_err(|e| format!("Failed to decode audio: {}", e))?;

        // Create sink using existing stream
        let sink = rodio::Sink::connect_new(self.stream_handle.mixer());
        sink.append(source);

        self.sink = Some(sink);
        self.temp_file = Some(temp_file);

        log::info!("Playback started with buffering");

        Ok(())
    }

    pub fn pause(&self) {
        if let Some(sink) = &self.sink {
            sink.pause();
        }
    }

    pub fn resume(&self) {
        if let Some(sink) = &self.sink {
            sink.play();
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = self.sink.take() {
            sink.stop();
        }
        // Keep stream_handle alive, only drop sink and temp file
        self.temp_file = None;
    }

    pub fn is_playing(&self) -> bool {
        self.sink
            .as_ref()
            .map(|s| !s.is_paused() && !s.empty())
            .unwrap_or(false)
    }

    pub fn is_paused(&self) -> bool {
        self.sink
            .as_ref()
            .map(|s| s.is_paused())
            .unwrap_or(false)
    }

    pub fn seek_forward(&self, seconds: u64) -> Result<(), String> {
        if let Some(sink) = &self.sink {
            let current_pos = sink.get_pos();
            let new_pos = current_pos + std::time::Duration::from_secs(seconds);
            sink.try_seek(new_pos)
                .map_err(|e| format!("Seek failed: {}", e))?;
        }
        Ok(())
    }

    pub fn seek_backward(&self, seconds: u64) -> Result<(), String> {
        if let Some(sink) = &self.sink {
            let current_pos = sink.get_pos();
            let new_pos = current_pos.saturating_sub(std::time::Duration::from_secs(seconds));
            sink.try_seek(new_pos)
                .map_err(|e| format!("Seek failed: {}", e))?;
        }
        Ok(())
    }

    pub fn get_position(&self) -> std::time::Duration {
        self.sink
            .as_ref()
            .map(|s| s.get_pos())
            .unwrap_or(std::time::Duration::ZERO)
    }
}
