mod verify;
use eframe::egui::{self, Color32, RichText};
use verify::verify;
// Handles UI
pub fn main() {
    // change result text colours here
    let verified_color = Color32::GREEN;
    let unverified_color = Color32::WHITE;

    // change verbosity here
    let verbose = true;
    //prepare a simple UI
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let mut uen = "".to_owned();
    let mut message = "".to_string();
    let mut verified = false;
    // Our application state:
    let _ = eframe::run_simple_native("UEN Verification", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Please insert UEN Number");
            ui.label(" ");
            ui.horizontal(|ui| {
                let name_label = ui.label("UEN: ");
                ui.text_edit_singleline(&mut uen).labelled_by(name_label.id);
            });
            ui.label(" ");

            if ui.button("Enter").clicked() {
                //send data to verify function
                (message, verified) = verify(uen.clone());
            }
            let result_text: String;
            let result_colour: Color32;

            if verified {
                result_colour = verified_color;
                if verbose {
                    result_text = message.to_owned();
                } else {
                    result_text = "verified".to_string();
                }
            } else {
                result_colour = unverified_color;
                if verbose {
                    result_text = message.to_owned();
                } else {
                    result_text = "invalid UEN".to_string();
                }
            }

            ui.label(RichText::new(result_text).color(result_colour));
        });
    });
}
