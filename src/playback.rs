use std::fs::File;
use std::io::Write as _;
use tempfile::NamedTempFile;

pub struct Player {
    temp_file: Option<NamedTempFile>,
    sink: Option<rodio::Sink>,
    stream_handle: rodio::OutputStream,
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

        // Fetch the audio file
        let response = reqwest::blocking::get(&actual_url)
            .map_err(|e| format!("Failed to fetch audio: {}", e))?;

        let bytes = response.bytes()
            .map_err(|e| format!("Failed to read audio data: {}", e))?;

        // Save to temp file
        let mut temp_file = NamedTempFile::new()
            .map_err(|e| format!("Failed to create temp file: {}", e))?;

        temp_file.write_all(&bytes)
            .map_err(|e| format!("Failed to write temp file: {}", e))?;

        // Open the file for playback - using try_from with File
        let file = File::open(temp_file.path())
            .map_err(|e| format!("Failed to open temp file: {}", e))?;

        // Use try_from instead of new to enable backward seeking
        let source = rodio::Decoder::try_from(file)
            .map_err(|e| format!("Failed to decode audio: {}", e))?;

        // Create sink using existing stream
        let sink = rodio::Sink::connect_new(&self.stream_handle.mixer());
        sink.append(source);

        self.sink = Some(sink);
        self.temp_file = Some(temp_file);

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
