use druid::{
    widget::{Align, Button, Flex, TextBox},
    FileDialogOptions, FileSpec, Widget, WidgetExt, WindowDesc,
};

#[derive(Debug, Clone)]
enum AllowedFileTypes {
    Text,
    Rust,
}
impl AllowedFileTypes {
    const TEXT_DESCRIPTION: &'static str = "Text";
    const RUST_DESCRIPTION: &'static str = "Rust";
    const TEXT_EXTENSION: &'static [&'static str] = &["txt"];
    const RUST_EXTENSION: &'static [&'static str] = &["rs"];

    fn description(&self) -> &'static str {
        match self {
            AllowedFileTypes::Text => Self::TEXT_DESCRIPTION,
            AllowedFileTypes::Rust => Self::RUST_DESCRIPTION,
        }
    }

    fn extensions(&self) -> &'static [&'static str] {
        match self {
            AllowedFileTypes::Text => Self::TEXT_EXTENSION,
            AllowedFileTypes::Rust => Self::RUST_EXTENSION,
        }
    }
}

pub fn ui_builder() -> impl Widget<String> {
    let allowed_type = AllowedFileTypes::Text;
    let description = allowed_type.description();
    let extensions = allowed_type.extensions();
    let txt = FileSpec::new(description, extensions);

    let default_save_name = "default.txt".to_owned();
    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![txt])
        .default_type(txt)
        .default_name(default_save_name)
        .name_label("Target")
        .title("Choose a target file")
        .button_text("Export");

    let default_open_name = "sesame.txt".to_owned();
    let open_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![txt])
        .default_type(txt)
        .default_name(default_open_name)
        .name_label("Source")
        .title("Choose a source file")
        .button_text("Import");

    let input = TextBox::new();
    let save = Button::new("Save").on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()));
    });

    let open = Button::new("Open").on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()));
    });

    let mut col = Flex::column();
    col.add_child(input);
    col.add_spacer(8.0);
    col.add_child(save);
    col.add_child(open);
    Align::centered(col)
}
