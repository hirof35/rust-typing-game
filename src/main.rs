use iced::widget::{column, text, text_input, Container};
use iced::{Alignment, Element, Length, Sandbox, Settings, Font};
use rand::seq::SliceRandom;
use std::time::Instant;

pub fn main() -> iced::Result {
    // 四角文字（豆腐）対策：デフォルトフォントを設定
    let mut settings = Settings::default();
    
    #[cfg(target_os = "windows")]
    {
        settings.default_font = Font::with_name("Yu Gothic");
    }
    #[cfg(target_os = "macos")]
    {
        settings.default_font = Font::with_name("Hiragino Kaku Gothic ProN");
    }
    #[cfg(target_os = "linux")]
    {
        settings.default_font = Font::with_name("Noto Sans CJK JP");
    }

    TypingGame::run(settings)
}

struct TypingGame {
    words: Vec<&'static str>,
    current_word: String,
    user_input: String,
    score: u32,
    miss_count: u32,
    question_count: u32,
    start_time: Option<Instant>,
    total_time: f32,
    is_game_over: bool,
    is_missed: bool,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Submit,
}

impl Sandbox for TypingGame {
    type Message = Message;

    fn new() -> Self {
        // ジェイソン（Friday the 13th）にちなんだ英語ワード・フレーズ
        let words = vec![
            "jason",
            "friday the 13th",
            "hockey mask",
            "machete",
            "crystal lake",
            "horror movie",
            "ch-ch-ch ah-ah-ah", // あの有名な効果音
            "run away",
        ];
        let mut game = Self {
            words,
            current_word: String::new(),
            user_input: String::new(),
            score: 0,
            miss_count: 0,
            question_count: 0,
            start_time: None,
            total_time: 0.0,
            is_game_over: false,
            is_missed: false,
        };
        game.next_word();
        game
    }

    fn title(&self) -> String {
        String::from("Jason Typing Game (Rust)")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                // 最初の入力でタイマースタート
                if self.start_time.is_none() && !value.is_empty() {
                    self.start_time = Some(Instant::now());
                }
                
                self.user_input = value;
                self.is_missed = false; // 入力を再開したらミス表示（赤文字）をリセット
            }
            Message::Submit => {
                if self.is_game_over {
                    return;
                }

                // 英語タイピングなので、入力された文字列がそのまま完全一致するか判定
                if self.user_input == self.current_word {
                    self.score += 1;
                    self.question_count += 1;
                    self.is_missed = false;

                    if self.question_count >= 5 {
                        self.is_game_over = true;
                        if let Some(start) = self.start_time {
                            self.total_time = start.elapsed().as_secs_f32();
                        }
                    } else {
                        self.next_word();
                    }
                } else {
                    // 空打ちのEnterはミスにしない
                    if !self.user_input.is_empty() {
                        self.miss_count += 1;
                        self.is_missed = true;
                        self.user_input.clear();
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = if self.is_game_over {
            column![
                text("GAME CLEAR!").size(40).style(iced::Color::from_rgb(0.0, 0.6, 0.0)),
                text(format!(
                    "SCORE: {} | MISS: {} | TIME: {:.2}s",
                    self.score, self.miss_count, self.total_time
                ))
                .size(20)
            ]
            .spacing(20)
            .align_items(Alignment::Center)
        } else {
            let word_color = if self.is_missed {
                iced::Color::from_rgb(1.0, 0.0, 0.0) // ミス時は赤字
            } else {
                iced::Color::BLACK
            };

            let display_word = if self.is_missed {
                "MISS! Try Again".to_string()
            } else {
                self.current_word.clone()
            };

            column![
                text(display_word).size(40).style(word_color),
                text(format!(
                    "WORD: {}/5 | MISS: {}",
                    self.question_count + 1,
                    self.miss_count
                ))
                .size(16)
                .style(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                text_input("Type here and press Enter...", &self.user_input)
                    .on_input(Message::InputChanged)
                    .on_submit(Message::Submit)
                    .padding(10)
                    .size(24)
            ]
            .spacing(20)
            .align_items(Alignment::Center)
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl TypingGame {
    fn next_word(&mut self) {
        let mut rng = rand::thread_rng();
        if let Some(&word) = self.words.choose(&mut rng) {
            self.current_word = word.to_string();
            self.user_input.clear();
        }
    }
}