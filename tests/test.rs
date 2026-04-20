use egui::text::LayoutJob;
use egui::TextFormat;
use egui_layout_job_macro::layout_job;

#[test]
fn empty() {
    assert_eq!(layout_job!(), LayoutJob::default());
}

#[test]
fn one_literal() {
    assert_eq!(layout_job!("foobar"), {
        let mut l = LayoutJob::default();
        l.append("foobar", 0.0, TextFormat::default());
        l
    })
}

#[test]
fn two_literals() {
    assert_eq!(layout_job!("foo""bar"), {
        let mut l = LayoutJob::default();
        l.append("foo", 0.0, TextFormat::default());
        l.append("bar", 0.0, TextFormat::default());
        l
    })
}

#[test]
fn three_literals() {
    assert_eq!(layout_job!("foo"123"bar"), {
        let mut l = LayoutJob::default();
        l.append("foo", 0.0, TextFormat::default());
        l.append("123", 0.0, TextFormat::default());
        l.append("bar", 0.0, TextFormat::default());
        l
    })
}

#[test]
fn one_ident() {
    let variable = 123456;
    assert_eq!(layout_job!(variable), {
        let mut l = LayoutJob::default();
        l.append(&variable.to_string(), 0.0, TextFormat::default());
        l
    })
}

#[test]
fn variety() {
    let variable = 123456;
    assert_eq!(layout_job!('a' variable format!("{:.2}", std::f32::consts::PI)), {
        let mut l = LayoutJob::default();
        l.append("a", 0.0, TextFormat::default());
        l.append(&variable.to_string(), 0.0, TextFormat::default());
        l.append("3.14", 0.0, TextFormat::default());
        l
    })
}

