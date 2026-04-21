use std::os::unix::raw::dev_t;

use egui::{
    Align, Color32, FontFamily, Stroke, TextFormat, epaint::text::VariationCoords, text::LayoutJob,
};
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
    let variable = 123456;
    assert_eq!(layout_job!(variable), {
        let mut l = LayoutJob::default();
        l.append(&variable.to_string(), 0.0, TextFormat::default());
        l
    });
}

#[test]
fn variety() {
    let variable = 123456;
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
    assert_eq!(layout_job!(in l2.clone(): 'a'), {
        let mut l = l2;
        l.append("a", 0.0, TextFormat::default());
        l
    });
}

#[test]
fn format_font_id() {
    assert_eq!(layout_job!(@font_id[10, mono]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                font_id: egui::FontId::new(10.0, FontFamily::Monospace),
                ..TextFormat::default()
            },
        );
        l
    });
    assert_eq!(layout_job!(@font_id[10.0, prop]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                font_id: egui::FontId::new(10.0, FontFamily::Proportional),
                ..TextFormat::default()
            },
        );
        l
    });
    let size = 5.0;
    assert_eq!(layout_job!(@font_id[size, "foobar"]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                font_id: egui::FontId::new(size, FontFamily::Name("foobar".into())),
                ..TextFormat::default()
            },
        );
        l
    });
}

#[test]
fn format_extra_letter_spacing() {
    assert_eq!(layout_job!(@extra_letter_spacing[1.0]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                extra_letter_spacing: 1.0,
                ..TextFormat::default()
            },
        );
        l
    });
    assert_eq!(layout_job!(@extra_letter_spacing[2]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                extra_letter_spacing: 2.0,
                ..TextFormat::default()
            },
        );
        l
    });
}

#[test]
fn format_expand_bg() {
    assert_eq!(layout_job!(@expand_bg[1.0]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                expand_bg: 1.0,
                ..TextFormat::default()
            },
        );
        l
    });
}

#[test]
fn format_line_height() {
    assert_eq!(layout_job!(@line_height[1.0]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                line_height: Some(1.0),
                ..TextFormat::default()
            },
        );
        l
    });
    assert_eq!(layout_job!(@line_height[2]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                line_height: Some(2.0),
                ..TextFormat::default()
            },
        );
        l
    });
    assert_eq!(layout_job!(@line_height[Some(3.0)]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                line_height: Some(3.0),
                ..TextFormat::default()
            },
        );
        l
    });
}

#[test]
fn format_color() {
    assert_eq!(layout_job!(@color[1, 2, 3]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                color: Color32::from_rgb(1, 2, 3),
                ..TextFormat::default()
            },
        );
        l
    });
    assert_eq!(layout_job!(@color[1, 2, 3, 4]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                color: Color32::from_rgba_unmultiplied(1, 2, 3, 4),
                ..TextFormat::default()
            },
        );
        l
    });
    assert_eq!(layout_job!(@color[red]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                color: Color32::RED,
                ..TextFormat::default()
            },
        );
        l
    });
    let color = Color32::RED;
    assert_eq!(layout_job!(@color[color]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                color,
                ..TextFormat::default()
            },
        );
        l
    });
}
#[test]
fn format_background() {
    assert_eq!(layout_job!(@background[1, 2, 3]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                background: Color32::from_rgb(1, 2, 3),
                ..TextFormat::default()
            },
        );
        l
    });
}

#[test]
fn format_coords() {
    assert_eq!(layout_job!(@coords[wght=500.0,b"wdth"=75]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                coords: VariationCoords::new([(b"wght", 500.0), (b"wdth", 75.0)]),
                ..TextFormat::default()
            },
        );
        l
    });
}

#[test]
fn format_italics() {
    assert_eq!(layout_job!(@italics("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                italics: true,
                ..TextFormat::default()
            },
        );
        l
    });
    assert_eq!(layout_job!(@italics[true]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                italics: true,
                ..TextFormat::default()
            },
        );
        l
    });
    assert_eq!(layout_job!(@italics[false]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                italics: false,
                ..TextFormat::default()
            },
        );
        l
    });
}

#[test]
fn format_underline() {
    assert_eq!(layout_job!(@underline[1.0f32, 1, 2, 3]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                underline: Stroke::new(1.0f32, Color32::from_rgb(1, 2, 3)),
                ..TextFormat::default()
            },
        );
        l
    });
    assert_eq!(layout_job!(@underline[1, red]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                underline: Stroke::new(1.0f32, Color32::RED),
                ..TextFormat::default()
            },
        );
        l
    });
}

#[test]
fn format_strikethrough() {
    assert_eq!(layout_job!(@strikethrough[1.0f32, 1, 2, 3]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                strikethrough: Stroke::new(1.0f32, Color32::from_rgb(1, 2, 3)),
                ..TextFormat::default()
            },
        );
        l
    });
}

#[test]
fn format_valign() {
    assert_eq!(layout_job!(@valign[min]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                valign: Align::Min,
                ..TextFormat::default()
            },
        );
        l
    });
    let valign = Align::Center;
    assert_eq!(layout_job!(@valign[centre]("a")), {
        let mut l = LayoutJob::default();
        l.append(
            "a",
            0.0,
            TextFormat {
                valign: Align::Center,
                ..TextFormat::default()
            },
        );
        l
    });
}

#[test]
fn format_nested() {
    assert_eq!(layout_job!(@color[red](@background[blue]("a"))), {
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
