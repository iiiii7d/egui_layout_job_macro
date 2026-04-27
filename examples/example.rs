#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use egui_layout_job_macro::{layout_job, text_format};

fn main() {
    eframe::run_ui_native("egui_layout_job_macro", eframe::NativeOptions::default(), |ui, _frame| {
        // simple
        ui.label(layout_job!("Simple LayoutJob"));

        // multiple segments (interpolation)
        let num_segments: usize = 3;
        ui.label(layout_job!("This LayoutJob has " num_segments " segments"));

        // font_id
        ui.label(layout_job!(@font_id[20, mono]("Font size 20, font family monospace")));
        // shortcuts for font_id
        ui.label(layout_job!(@size[20]("Font size 20 only")));
        ui.label(layout_job!(@family[mono]("Monospace only")));
        ui.label(layout_job!(@mono("Another method for monospace text")));

        // extra_letter_spacing
        ui.label(layout_job!(@extra_letter_spacing[3.0]("extra letter spacing 3")));

        // expand_bg
        ui.label(layout_job!(@expand_bg[3.0](@bg_red("expand background padding 3"))));

        // line_height
        ui.label(layout_job!(@line_height[10.0]("line\nheight 10")));

        // colour/background (foreground and background)
        ui.label(layout_job!(@color[red]("red colour text ") @bg[yellow]("yellow colour background")));
        ui.label(layout_job!(@red("Another method ") @bg_yellow("to have coloured text and backgrounds")));
        ui.label(layout_job!(@col["ffa500"]("You can use hex colours ") @bg[0, 255, 0]("or RGB(A) to set the colours too")));

        // coords (no clue what this does lol)
        ui.label(layout_job!(@coords[wght=500,b"wdth"=75]("weight=500, width=75")));

        // italics
        ui.label(layout_job!(@italics("Ital") @i("ics")));

        // underline / strikethrough
        ui.label(layout_job!(@underline("Underline ") @u[2]("with custom width ") @u[2, red]("and colour")));
        ui.label(layout_job!(@strikethrough("Strikethrough ") @s[2]("with custom width ") @s[2, red]("and colour")));

        // valign
        ui.label(layout_job!(@valign[min]("Valign ") @max(" (shortcut)")));

        // leading space (must be before a segment)
        ui.label(layout_job!("This has" ~10 "a lot of leading space"));

        // pre-defined TextFormat
        let tf = text_format!(red, bg_white, underline, italics, size[20]);
        ui.label(layout_job!(@[tf]("This is probably very important")));

        // a custom LayoutJob (pass in a tuple of (text, leading space, text format))
        ui.label(layout_job!(#("Custom LayoutJob", 1.0, egui::TextFormat::default())));

        // you can mix and nest all of the above
        ui.label(layout_job!("The " @i("quick") " " @bg_brown(@white("brown")) " " @u("fox") " " @extra_letter_spacing[10.0]("jumps") " " @mono("over") " the " @s("lazy") " " @size[10]("dog")));
    }).unwrap();
}
