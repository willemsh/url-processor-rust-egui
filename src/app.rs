use std::io::BufRead;
use std::io::BufReader;
use std::ops::DerefMut;
use std::process::Command;
use std::process::Stdio;
use std::thread;

use std::sync::{Arc, Mutex};
/*
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    url: String,

    log: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}
*/

pub struct InnerApp {
    url: String,
    log: String,
}

impl Default for InnerApp {
    fn default() -> Self {
        Self {
            url: "".to_owned(),
            log: "".to_owned(),
        }
    }
}

pub struct TemplateApp<T> {
    // Example stuff:
    arc: Arc<T>,
}

impl Default for TemplateApp<Mutex<InnerApp>> {
    fn default() -> Self {
        let app = InnerApp::default();

        Self {
            arc: Arc::new(Mutex::new(app)),
        }
    }
}


fn process(url: &str, cloned: Arc<Mutex<InnerApp>>, ctx: egui::Context) {
    println!("Processing {} from another thread", url);

    let command = "PATH_TO_COMMAND";

    let mut program = match Command::new(command)
        .arg(url)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => {
            println!("Cannot run program '{}'.", command);
            return;
        }
    };

    println!("Running");

    match program.stdout.as_mut() {
        Some(out) => {
            let buf_reader = BufReader::new(out);

            for line in buf_reader.lines() {
                let mut app = cloned.lock().unwrap();

                match line {
                    Ok(l) => {
                        app.log = format!("{}\n{}", l,app.log);
                        drop(app);
                        ctx.request_repaint();
                        //app.update_log(format!("{}\n", l.as_str()).as_ref());
                    }
                    Err(_) => return,
                };

                thread::yield_now();
            }

        }
        None => return,
    }

    let mut app = cloned.lock().unwrap();
    app.log = format!("Done processing {} from another thread", url);
    drop(app);
    ctx.request_repaint();

}


fn noop(url: &str, cloned: Arc<Mutex<InnerApp>>, ctx: egui::Context) {
    println!("Nooping {} from another thread", url);



    for  i in 1..=100 {
        let mut app = cloned.lock().unwrap();
        // destruct
        //let InnerApp { url, log } = &mut app2.deref_mut();

        app.log = format!("{}%", i);
        drop(app);
        ctx.request_repaint();
        thread::yield_now();
    }
}
/*


impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            url: "".to_owned(),
            log: "".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    pub fn default() -> Self {
        Default::default()
    }

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}
*/

impl eframe::App for TemplateApp<Mutex<InnerApp>> {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        //eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // clone
        let cloned = Arc::clone(&mut self.arc);

        // lock and unwrap
        let mut app = cloned.lock().unwrap();

        // destruct
        let InnerApp { url, log } = &mut app.deref_mut();

        // let Self { url, log } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Log");

            //ui.hyperlink("https://github.com/emilk/egui_template");
            ui.text_edit_multiline(log);

            //egui::warn_if_debug_build(ui);
        });
        /*
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });
        */
        let mut clicked = false;
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui.horizontal(|ui| {
                ui.label("Important URL");
                ui.text_edit_singleline(url);
            });

            if ui.button("Process!").clicked() {
                clicked = true;
            }

            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }

        if clicked {
            let ctx = ctx.clone();
            let cloned = Arc::clone(&mut self.arc);
            //let cloned3 = Arc::clone(&mut self.arc);
            thread::spawn(move || {
                println!("Click!");

                // thread::sleep(delay);
                let mut app = cloned.lock().unwrap();
                // destruct
                let InnerApp { url, log } = &mut app.deref_mut();

                let url = format!("{}", &url);

                drop(app);

                println!("{}", url);

                process(&url, cloned, ctx);

            });
        }
    }
}
