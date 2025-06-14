use reqwest::blocking;
use serde_json::Value;
use std::{collections::HashMap, io, process::Command, thread, time::Duration};

fn main() -> io::Result<()> {
    let mut player_name = String::new();
    let mut last_title = String::new();
    let mut lyrics_map = HashMap::new();
    let mut song_id = String::new();

    loop {
        // 获取当前播放器
        if let Ok(output) = Command::new("playerctl").arg("--list-all").output() {
            player_name = String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
        }

        if player_name.is_empty() {
            println!("");
            thread::sleep(Duration::from_millis(200));
            continue;
        }

        // 获取歌曲元数据
        let metadata = get_metadata(&player_name);
        let title = metadata.get("title").cloned().unwrap_or_default();
        let position = metadata.get("position").cloned().unwrap_or_default();
        let new_song_id = get_song_id(&player_name, &metadata);

        // 检查歌曲是否变化
        if title != last_title || new_song_id != song_id {
            last_title = title;
            song_id = new_song_id;
            lyrics_map.clear();

            // 获取新歌词
            if !song_id.is_empty() {
                if let Ok(lyrics) = fetch_lyrics(&song_id) {
                    parse_lyrics(&lyrics, &mut lyrics_map);
                }
            }
        }

        // 获取当前歌词
        let current_lyric = if !position.is_empty() && !lyrics_map.is_empty() {
            let Ok(pos_secs) = parse_duration(&position) else {
                println!("");
                thread::sleep(Duration::from_millis(200));
                continue;
            };
            find_current_lyric(pos_secs, &lyrics_map)
        } else {
            ""
        };

        // 歌词输出
        println!(
            "♫ {}",
            if current_lyric != "" {
                current_lyric
            } else {
                "_ z Z Z ♥"
            }
        );
        // 保持200ms间隔
        thread::sleep(Duration::from_millis(200));
    }
}

// 获取播放器元数据
fn get_metadata(player: &str) -> HashMap<String, String> {
    let mut metadata = HashMap::new();

    let keys = ["title", "artist", "album", "mpris:artUrl", "mpris:trackid"];

    for key in keys {
        let output = Command::new("playerctl")
            .args(["--player", player, "metadata", key])
            .output()
            .unwrap();

        let value = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if !value.is_empty() {
            metadata.insert(key.to_string(), value);
        }
    }

    // 获取播放位置
    if let Ok(output) = Command::new("playerctl")
        .args([
            "--player",
            player,
            "metadata",
            "--format",
            "{{ duration(position) }}",
        ])
        .output()
    {
        let position = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if !position.is_empty() {
            metadata.insert("position".to_string(), position);
        }
    }

    metadata
}

// 获取歌曲ID
fn get_song_id(player: &str, metadata: &HashMap<String, String>) -> String {
    let trackid = metadata.get("mpris:trackid").cloned().unwrap_or_default();

    match player {
        "ElectronNCM" | "musicfox" => trackid
            .split('/')
            .nth(4)
            .map(|s| s.replace("'", ""))
            .unwrap_or_default(),
        "feeluown" => trackid
            .split('/')
            .nth(6)
            .map(String::from)
            .unwrap_or_default(),
        "yesplaymusic" => {
            if let Ok(response) = blocking::get("http://127.0.0.1:27232/player") {
                if let Ok(json) = response.json::<Value>() {
                    return json["currentTrack"]["id"]
                        .as_str()
                        .map(String::from)
                        .unwrap_or_default();
                }
            }
            String::new()
        }
        "Qcm" => trackid
            .split('/')
            .nth(2)
            .map(|s| s.replace("'", ""))
            .unwrap_or_default(),
        "NeteaseCloudMusicGtk4" => trackid
            .split('/')
            .nth(5)
            .map(|s| s.replace("'", ""))
            .unwrap_or_default(),
        _ => String::new(),
    }
}

// 从网易云API获取歌词
fn fetch_lyrics(song_id: &str) -> Result<String, reqwest::Error> {
    let url = format!("http://music.163.com/api/song/media?id={}", song_id);
    let response = blocking::get(&url)?;
    let json: Value = response.json()?;

    Ok(json["lyric"].as_str().unwrap_or("").to_string())
}

// 解析歌词文本
fn parse_lyrics(content: &str, map: &mut HashMap<u32, String>) {
    for line in content.lines() {
        let parts: Vec<&str> = line.splitn(2, ']').collect();
        if parts.len() < 2 {
            continue;
        }

        if let Some(time_str) = parts[0].strip_prefix('[') {
            let time_str = time_str.split('.').next().unwrap_or(""); // 忽略毫秒部分
            if let Ok(secs) = parse_duration(time_str) {
                let lyric = parts[1].trim();
                if !lyric.is_empty() {
                    map.insert(secs, lyric.to_string());
                }
            }
        }
    }
}

// 时间字符串转秒数
fn parse_duration(time_str: &str) -> Result<u32, &'static str> {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid format");
    }

    let minutes = parts[0].parse::<u32>().map_err(|_| "Invalid number")?;
    let seconds = parts[1].parse::<u32>().map_err(|_| "Invalid number")?;

    Ok(minutes * 60 + seconds)
}

// 查找当前歌词
fn find_current_lyric(position: u32, lyrics: &HashMap<u32, String>) -> &str {
    let mut nearest_time = 0;
    let mut nearest_lyric = "";

    for (&time, lyric) in lyrics {
        if time <= position && time > nearest_time {
            nearest_time = time;
            nearest_lyric = lyric;
        }
    }

    nearest_lyric
}
