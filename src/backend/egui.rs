use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

pub fn run(file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let markdown = std::fs::read_to_string(&file_path)
        .unwrap_or_else(|e| format!("# Error\nCould not read `{}`: {}", file_path.display(), e));

    let watcher_rx = crate::core::watcher::watch_file(&file_path)?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 900.0])
            .with_title(format!("mdr - {}", file_path.display())),
        ..Default::default()
    };

    let file_path_clone = file_path.clone();
    eframe::run_native(
        "mdr",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(MdrApp {
                markdown,
                cache: CommonMarkCache::default(),
                file_path: file_path_clone,
                watcher_rx,
            }))
        }),
    )
    .map_err(|e| e.to_string().into())
}

struct MdrApp {
    markdown: String,
    cache: CommonMarkCache,
    file_path: PathBuf,
    watcher_rx: Receiver<()>,
}

impl eframe::App for MdrApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for file changes
        if self.watcher_rx.try_recv().is_ok() {
            // Drain any extra signals
            while self.watcher_rx.try_recv().is_ok() {}
            if let Ok(content) = std::fs::read_to_string(&self.file_path) {
                self.markdown = content;
                self.cache = CommonMarkCache::default();
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                CommonMarkViewer::new().show(ui, &mut self.cache, &self.markdown);
            });
        });

        // Request repaint periodically to check for file changes
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}
