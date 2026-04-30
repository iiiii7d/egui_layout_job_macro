use duplicate::duplicate_item;
use egui::{Align, Color32, FontFamily, FontId, Stroke, TextFormat, epaint::text::VariationCoords};
use egui_layout_job_macro::text_format;

#[test]
fn font_id_expr() {
    let font_id = FontId::new(10.0, FontFamily::Proportional);
    assert_eq!(
        text_format!(font_id[font_id.clone()]),
        TextFormat {
            font_id,
            ..TextFormat::default()
        }
    );
}
#[duplicate_item(
    test_name v1 v2;
    [font_id_prop] [prop] [FontFamily::Proportional];
    [font_id_mono] [mono] [FontFamily::Monospace];
    [font_id_custom] ["abc"] [FontFamily::Name("abc".into())];
)]
#[test]
fn test_name() {
    assert_eq!(
        text_format!(font_id[10.0, v1]),
        TextFormat {
            font_id: FontId::new(10.0, v2),
            ..TextFormat::default()
        }
    );
}
#[test]
fn font_id_sugar_size() {
    assert_eq!(
        text_format!(size[1.0]),
        TextFormat {
            font_id: FontId::new(1.0, FontFamily::Proportional),
            ..TextFormat::default()
        }
    );
}
#[test]
fn font_id_sugar_family() {
    assert_eq!(
        text_format!(family[mono]),
        TextFormat {
            font_id: FontId::new(14.0, FontFamily::Monospace),
            ..TextFormat::default()
        }
    );
}
#[test]
fn font_id_sugar_mono() {
    assert_eq!(
        text_format!(mono),
        TextFormat {
            font_id: FontId::new(14.0, FontFamily::Monospace),
            ..TextFormat::default()
        }
    );
}
#[test]
fn font_id_sugar_size_family() {
    assert_eq!(text_format!(mono, size[20]), text_format!(font_id[20, mono]));
}
#[test]
fn font_id_sugar_family_size() {
    assert_eq!(text_format!(size[20], mono), text_format!(font_id[20, mono]));
}

#[duplicate_item(
    test_name attr val;
    [extra_letter_spacing_decimal] [extra_letter_spacing] [1.0];
    [extra_letter_spacing_integer] [extra_letter_spacing] [1];
    [extra_letter_spacing_expr] [extra_letter_spacing] [(1.0)];
    [expand_bg_decimal] [expand_bg] [1.0];
    [expand_bg_integer] [expand_bg] [1];
    [expand_bg_expr] [expand_bg] [(1.0)];
)]
#[test]
fn test_name() {
    assert_eq!(
        text_format!(attr[val]),
        TextFormat {
            attr: 1.0,
            ..TextFormat::default()
        }
    );
}

#[duplicate_item(
    test_name val;
    [line_height_decimal] [1.0];
    [line_height_integer] [1];
    [line_height_expr] [Some(1.0)];
)]
#[test]
fn test_name() {
    assert_eq!(
        text_format!(line_height[val]),
        TextFormat {
            line_height: Some(1.0),
            ..TextFormat::default()
        }
    );
}

#[duplicate_item(
    test_name attr1 attr2 val;
    [color_expr] [color] [color] [Color32::RED];
    [color_hex] [color] [color] ["ff0000"];
    [color_rgb] [color] [color] [255, 0, 0];
    [color_rgba] [color] [color] [255, 0, 0, 255];
    [colour] [colour] [color] [Color32::RED];
    [col] [col] [color] [Color32::RED];
    [background_expr] [background] [background] [Color32::RED];
    [background_hex] [background] [background] ["ff0000"];
    [background_rgb] [background] [background] [255, 0, 0];
    [background_rgba] [background] [background] [255, 0, 0, 255];
    [bg] [bg] [background] [Color32::RED];
)]
#[test]
fn test_name() {
    assert_eq!(
        text_format!(attr1[val]),
        TextFormat {
            attr2: Color32::RED,
            ..TextFormat::default()
        }
    );
}

#[test]
fn color_sugar_red() {
    assert_eq!(
        text_format!(red),
        TextFormat {
            color: Color32::RED,
            ..TextFormat::default()
        }
    );
}
#[test]
fn background_sugar_bg_red() {
    assert_eq!(
        text_format!(bg_red),
        TextFormat {
            background: Color32::RED,
            ..TextFormat::default()
        }
    );
}

#[test]
fn coords() {
    let tag = b"abcd";
    let value = 123.0f32;
    assert_eq!(
        text_format!(coords[wght=500.0,b"wdth"=75,(tag)=value]),
        TextFormat {
            coords: VariationCoords::new([(b"wght", 500.0), (b"wdth", 75.0), (tag, value)]),
            ..TextFormat::default()
        }
    );
}

#[duplicate_item(
    test_name attr val;
    [italics_0args] [i] [];
    [italics_1arg] [i] [true];
    [i] [i] [];
)]
#[test]
fn test_name() {
    assert_eq!(
        text_format!(attr[val]),
        TextFormat {
            italics: true,
            ..TextFormat::default()
        }
    );
}

#[duplicate_item(
    test_name attr1 attr2 val;
    [underline_0args] [underline] [underline] [];
    [underline_1arg_decimal] [underline] [underline] [1.0];
    [underline_1arg_integer] [underline] [underline] [1];
    [underline_1arg_expr] [underline] [underline] [(1.0f32)];
    [underline_color_expr] [underline] [underline] [1, Color32::GRAY];
    [underline_color_hex] [underline] [underline] [1, "a0a0a0"];
    [underline_color_rgb] [underline] [underline] [1, 160, 160, 160];
    [underline_color_rgba] [underline] [underline] [1, 160, 160, 160, 255];
    [u] [u] [underline] [];
    [strikethrough_0args] [strikethrough] [strikethrough] [];
    [strikethrough_1arg_decimal] [strikethrough] [strikethrough] [1.0];
    [strikethrough_1arg_integer] [strikethrough] [strikethrough] [1];
    [strikethrough_1arg_expr] [strikethrough] [strikethrough] [(1.0f32)];
    [strikethrough_color_expr] [strikethrough] [strikethrough] [1, Color32::GRAY];
    [strikethrough_color_hex] [strikethrough] [strikethrough] [1, "a0a0a0"];
    [strikethrough_color_rgb] [strikethrough] [strikethrough] [1, 160, 160, 160];
    [strikethrough_color_rgba] [strikethrough] [strikethrough] [1, 160, 160, 160, 255];
    [s] [s] [strikethrough] [];
)]
#[test]
fn test_name() {
    assert_eq!(
        text_format!(attr1[val]),
        TextFormat {
            attr2: Stroke::new(1.0f32, Color32::GRAY),
            ..TextFormat::default()
        }
    );
}

#[duplicate_item(
    test_name attr;
    [underline_auto_colour] [underline];
    [strikethrough_auto_colour] [strikethrough];
)]
#[test]
fn test_name() {
    assert_eq!(
        text_format!(red, attr),
        TextFormat {
            color: Color32::RED,
            attr: Stroke::new(1.0f32, Color32::RED),
            ..TextFormat::default()
        }
    );
}

#[duplicate_item(
    test_name val1 val2;
    [valign_min] [min] [Align::Min];
    [valign_centre] [centre] [Align::Center];
    [valign_center] [center] [Align::Center];
    [valign_max] [max] [Align::Max];
    [valign_expr] [Align::Min] [Align::Min];
)]
#[test]
fn test_name() {
    assert_eq!(
        text_format!(valign[val1]),
        TextFormat {
            valign: val2,
            ..TextFormat::default()
        }
    );
}

#[test]
fn valign_sugar_min() {
    assert_eq!(
        text_format!(min),
        TextFormat {
            valign: Align::Min,
            ..TextFormat::default()
        }
    );
}

#[test]
fn two_attributes() {
    assert_eq!(
        text_format!(expand_bg[1.0], i),
        TextFormat {
            expand_bg: 1.0,
            italics: true,
            ..TextFormat::default()
        },
    );
}
