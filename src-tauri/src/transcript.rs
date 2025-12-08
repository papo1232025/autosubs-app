// use eyre::Result;
// use num::integer::div_floor;
fn split_japanese_text(text: &str, max_len: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    let mut count = 0;

    for ch in text.chars() {
        current.push(ch);
        count += 1;

        // 句読点で区切る
        if ch == '。' || ch == '、' || ch == '！' || ch == '？' {
            chunks.push(current.clone());
            current.clear();
            count = 0;
            continue;
        }

        // 自然な文節っぽい位置で改行（助詞）
        if ch == 'は' || ch == 'が' || ch == 'を' || ch == 'に' || ch == 'で' || ch == 'と' {
            if count >= max_len {
                chunks.push(current.clone());
                current.clear();
                count = 0;
                continue;
            }
        }

        // 指定文字数で改行
        if count >= max_len {
            chunks.push(current.clone());
            current.clear();
            count = 0;
        }
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}



use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WordTimestamp {
    pub word: String,
    pub start: f64,
    pub end: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<WordTimestamp>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColorModifier {
    pub enabled: bool,
    pub color: String,
}

impl Default for ColorModifier {
    fn default() -> Self {
        ColorModifier {
            enabled: false,
            color: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sample {
    pub start: f64,
    pub end: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Speaker {
    pub name: String,
    pub fill: ColorModifier,
    pub outline: ColorModifier,
    pub border: ColorModifier,
    pub sample: Sample,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transcript {
    pub processing_time_sec: u64,
    pub segments: Vec<Segment>,
    pub speakers: Vec<Speaker>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonSegment {
    id: usize,
    seek: usize,
    start: f64,
    end: f64,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    speaker_id: Option<String>,
    tokens: Vec<i32>,
    temperature: f32,
    avg_logprob: f64,
    compression_ratio: f64,
    no_speech_prob: f64,
    words: Vec<JsonWordTimestamp>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonWordTimestamp {
    word: String,
    start: f64,
    end: f64,
    probability: f32,
}
