use eframe::egui;
use eframe::egui::{FontFamily::*, FontId, TextStyle};
use rodio::{Sink, source::Source, OutputStream};
use std::{collections::BTreeMap, fs::File, io::BufReader, path::PathBuf};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 240.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "lift",
        options,
        Box::new(|_cc| Ok(Box::<MyEguiApp>::new(MyEguiApp::new(&_cc)))),
    )
}

enum Mode {
    FileSelectionMode,
    TranscribeMode,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::FileSelectionMode
    }
}

#[derive(Default)]
struct MyEguiApp {
    mode: Mode,
    path_to_audio_file: Option<PathBuf>,
    file_name: Option<String>,
    playing: bool,
    sink: Option<Sink>,
    stream_handle: Option<OutputStream>,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let text_styles: BTreeMap<TextStyle, FontId> = [
            (TextStyle::Heading, FontId::new(20.0, Proportional)),
            (TextStyle::Body, FontId::new(20.0, Monospace)),
            (TextStyle::Monospace, FontId::new(12.0, Monospace)),
            (TextStyle::Button, FontId::new(12.0, Proportional)),
            (TextStyle::Small, FontId::new(8.0, Proportional)),
        ]
        .into();
        cc.egui_ctx
            .all_styles_mut(move |style| style.text_styles = text_styles.clone());
        MyEguiApp {
            mode: Mode::FileSelectionMode,
            path_to_audio_file: None,
            file_name: None,
            playing: false,
            sink: None,
            stream_handle: None,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.mode {
            Mode::FileSelectionMode => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(40.0);
                        ui.label("Drag-and-drop mp3 file onto this window!");
                        if ui.button("Or click here to select a file...").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                match File::open(&path) {
                                    Ok(f) => {
                                        self.path_to_audio_file = Some(path.clone());
                                        self.file_name = Some(String::from(
                                            path.file_name().unwrap().to_str().unwrap(),
                                        ));
                                        self.mode = Mode::TranscribeMode;
                                    }
                                    Err(e) => {
                                        todo!("Implement IO error popup");
                                    }
                                }
                            }
                        }
                    });
                });

                preview_files_being_dropped(ctx);

                // Collect dropped files:
                ctx.input(|i| {
                    if !i.raw.dropped_files.is_empty() {
                        let path = &i.raw.dropped_files[0].path.clone().unwrap();
                        match File::open(&path) {
                            Ok(f) => {
                                self.path_to_audio_file = Some(path.clone());
                                self.file_name =
                                    Some(String::from(path.file_name().unwrap().to_str().unwrap()));
                                self.mode = Mode::TranscribeMode;
                            }
                            Err(e) => {
                                todo!("Implement file permission error or something");
                            }
                        }
                    }
                });
            }
            Mode::TranscribeMode => {
                if self.playing {
                    // Get an output stream handle to the default physical sound device.
                    // Note that the playback stops when the stream_handle is dropped.
                    if self.stream_handle.is_none() {
                        self.stream_handle = Some(rodio::OutputStreamBuilder::open_default_stream()
                            .expect("open default audio stream"));
                    }

                    let file = File::open(self.path_to_audio_file.as_mut().unwrap()).unwrap();

                    if self.sink.is_none() {
                        self.sink = Some(rodio::play(&self.stream_handle.as_mut().unwrap().mixer(), file).unwrap())
                    }
                }
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.label(format!("{}", self.file_name.clone().unwrap()));
                        let button_text = match self.playing {
                            false => "Play",
                            true => "Pause",
                        };
                        ui.columns(2, |columns| {
                            columns[0].vertical_centered(|ui| {
                                if ui.button(button_text).clicked() {
                                    self.playing = !self.playing;
                                }
                            });
                            columns[1].vertical_centered(|ui| {
                                if self.playing {
                                    ui.label("doo doo doo la la la music noises");
                                }
                            });
                        });
                    });
                });
            }
        }
    }
}

/// Preview hovering files:
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::{Align2, Color32, Id, LayerId, Order, TextStyle};
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}
