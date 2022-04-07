use std::borrow::Cow;
use std::fmt::format;
use std::time::Duration;

use eframe::{NativeOptions, run_native};
use eframe::egui::{CentralPanel, Context, CtxRef, FontDefinitions, FontFamily, Rgba, ScrollArea, TextStyle, Vec2};
use eframe::epi::{App, Frame, Storage};

struct Reader {
    articles: Vec<ArticleCard>,
}

impl Reader {
    fn new() -> Reader {
        let iter = (0..20).map(|i| ArticleCard {
            title: format!("Article {} title", i),
            desc: format!("This is a sample description for the article {}", i),
            url: format!("https://example.com/new/article/{}", i),
        });
        Reader { articles: Vec::from_iter(iter) }
    }

    fn configure_fonts(&self, ctx: &Context) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert("MesloLGS".to_string(), Cow::Borrowed(include_bytes!("../../MesloLGS_NF_Regular.ttf")));
        font_def.family_and_size.insert(TextStyle::Heading, (FontFamily::Proportional, 35.));
        font_def.family_and_size.insert(TextStyle::Body, (FontFamily::Proportional, 20.));
        font_def.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(0, "MesloLGS".to_string());
        ctx.set_fonts(font_def);
    }
}

impl App for Reader {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                for a in &self.articles {
                    ui.label(&a.title);
                    ui.label(&a.desc);
                    ui.label(&a.url);
                }
            });
        });
    }

    fn setup(&mut self, _ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {
        self.configure_fonts(_ctx);
    }

    fn name(&self) -> &str {
        "Reader"
    }
}

struct ArticleCard {
    title: String,
    desc: String,
    url: String,
}

fn main() {
    let app = Reader::new();
    let mut win = NativeOptions::default();
    win.initial_window_size = Some(Vec2::new(540., 960.));
    run_native(Box::new(app), win);
}
