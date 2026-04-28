use egui::{Color32, TextFormat, text::LayoutJob};
use egui_layout_job_macro::{layout_job, text_format};

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
    });
}

#[test]
fn two_literals() {
    assert_eq!(layout_job!("foo""bar"), {
        let mut l = LayoutJob::default();
        l.append("foo", 0.0, TextFormat::default());
        l.append("bar", 0.0, TextFormat::default());
        l
    });
}

#[test]
fn three_literals() {
    assert_eq!(layout_job!("foo"123"bar"), {
        let mut l = LayoutJob::default();
        l.append("foo", 0.0, TextFormat::default());
        l.append("123", 0.0, TextFormat::default());
        l.append("bar", 0.0, TextFormat::default());
        l
    });
}

#[test]
fn one_ident() {
    let variable = 123_456;
    assert_eq!(layout_job!(variable), {
        let mut l = LayoutJob::default();
        l.append(&variable.to_string(), 0.0, TextFormat::default());
        l
    });
}

#[test]
fn variety() {
    let variable = 123_456;
    assert_eq!(
        layout_job!('a' variable format!("{:.2}", std::f32::consts::PI)),
        {
            let mut l = LayoutJob::default();
            l.append("a", 0.0, TextFormat::default());
            l.append(&variable.to_string(), 0.0, TextFormat::default());
            l.append("3.14", 0.0, TextFormat::default());
            l
        }
    );
}

#[test]
fn from_existing_layout_job() {
    let l2 = LayoutJob {
        justify: true,
        ..LayoutJob::default()
    };
    assert_eq!(layout_job!(use l2.clone(): 'a'), {
        let mut l = l2;
        l.append("a", 0.0, TextFormat::default());
        l
    });
}

#[test]
fn format_nested() {
    assert_eq!(layout_job!(@red(@bg_blue("a"))), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                color: Color32::RED,
                background: Color32::BLUE,
                ..TextFormat::default()
            },
        );
        l
    });
}

#[test]
fn format_and_non_format() {
    assert_eq!(layout_job!("a" @color[red]("b") "c"), {
        let mut l = LayoutJob::default();
        l.append("a", 0.0, TextFormat::default());
        l.append(
            "b",
            0.0,
            TextFormat {
                color: Color32::RED,
                ..TextFormat::default()
            },
        );
        l.append("c", 0.0, TextFormat::default());
        l
    });
}

#[test]
fn format_existing_text_format() {
    let tf = text_format!(red);
    assert_eq!(layout_job!(@[tf]("a")), {
        let mut l = LayoutJob::default();
        l.append("a", 0.0, tf);
        l
    });
}

#[test]
fn raw() {
    assert_eq!(layout_job!(#("a", 0.0, TextFormat::default())), {
        let mut l = LayoutJob::default();
        l.append("a", 0.0, TextFormat::default());
        l
    });
}

#[test]
fn leading_space() {
    assert_eq!(layout_job!(~2 "a"), {
        let mut l = LayoutJob::default();
        l.append("a", 2.0, TextFormat::default());
        l
    });
}
