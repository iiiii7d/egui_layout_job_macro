use egui::text::LayoutJob;
use egui::TextFormat;
use egui_layout_job_macro::{layout_job, text_format};

#[test]
fn one_attribute() {
    assert_eq!(text_format!(expand_bg[1.0]), TextFormat {
        expand_bg: 1.0,
        ..TextFormat::default()
    },);
}

#[test]
fn two_attributes() {
    assert_eq!(text_format!(expand_bg[1.0], i), TextFormat {
        expand_bg: 1.0,
        italics: true,
        ..TextFormat::default()
    },);
}