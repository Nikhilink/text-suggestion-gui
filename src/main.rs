use eframe::egui::{self, pos2, vec2, Align2, Color32, FontId, Rect, TextEdit};

mod text_suggestion;

use text_suggestion::suggestion_engine;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Text suggestion Application Demo",
        options,
        Box::new(|_cc| Box::new(TextEditor::new())),
    )
}

struct TextEditor {
    language: String,
    text: String,
    current_word: String,
    suggestion_engine: suggestion_engine::SuggestionEngine,
    suggestion_words: Vec<String>
}

impl TextEditor {
    fn new() -> Self {
        Self {
            language: String::from("rs"),
            text: String::from(""),
            current_word: String::from(""),
            suggestion_words: Vec::new(),
            suggestion_engine: suggestion_engine::SuggestionEngine::new()
        }
    }
}

fn get_word(text: &str, index: usize) -> &str
{
    let word = &text[..index];

    if let Some(space_pos) = word.rfind(' ')
    {
        &word[space_pos + 1..]
    }
    else {
        word
    }
}

impl eframe::App for TextEditor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let text_editor = TextEdit::multiline(&mut self.text)
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .desired_rows(20)
                .lock_focus(true);

            let response = text_editor.show(ui);

            if response.response.changed() {

                let index: usize = match response.cursor_range {
                    Some(x) => x.primary.ccursor.index,
                    None => 0 as usize,
                };
                if index > 0 {
                    if self.text.chars().last() != Some(' ')
                    {

                    
                    self.current_word = get_word(&self.text, index).to_string();
                    self.suggestion_words = self.suggestion_engine.find_words(&self.current_word);
                    self.suggestion_words.sort_by_key(|s| s.len());
                    }
                    else {
                        self.current_word = "".to_string();
                    } 
                    
                }

                
            }

            if !self.current_word.is_empty()
                {
                    let text_editor_rect: egui::Rect = response.response.rect;

                    let below_position = pos2(text_editor_rect.left(), text_editor_rect.bottom() + 4.0);

                    let suggestion_rect = Rect::from_min_size(below_position, vec2(text_editor_rect.width(), 5.0 * 20.0));

                    // let layer_id = LayerId::new(Order::Foreground, Id::new("suggestion_list"));

                    let painter = ui.painter_at(suggestion_rect);

                    painter.rect_filled(suggestion_rect, 4.0, Color32::from_gray(30));

                    for (i, s) in self.suggestion_words.iter().enumerate()
                    {
                        let y = below_position.y + i as f32 * 20.0;

                        painter.text(pos2(below_position.x + 4.0, y), Align2::LEFT_TOP, s, FontId::monospace(14.0), Color32::WHITE);
                    }

                }
        });
    }
}
