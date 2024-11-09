use std::fs;

use druid::{
    commands::SAVE_FILE_AS,
    platform_menus,
    widget::{Align, Button, Flex, Label, TextBox},
    AppDelegate, Command, Data, DelegateCtx, Env, FileDialogOptions, FileSpec, Handled, Lens,
    Target, Widget, WidgetExt, WindowDesc,
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

#[derive(Clone, Data, Lens)]
pub struct UiState {
    pub font_size: f64,
    pub input_text: String,
}

pub fn ui_builder() -> impl Widget<UiState> {
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

    // Input TextBox
    let input = TextBox::new()
        .with_placeholder("Type here")
        .lens(UiState::input_text)
        .padding(5.0);

    // Font Size Label
    let font_size_label =
        Label::new(|data: &UiState, _env: &_| format!("Font Size: {:.0}", data.font_size));

    // Font Size Adjustment Buttons
    let increase_font = Button::new("+").on_click(|_ctx, data: &mut UiState, _env| {
        data.font_size += 1.0;
    });

    let decrease_font = Button::new("-").on_click(|_ctx, data: &mut UiState, _env| {
        data.font_size = (data.font_size - 1.0).max(1.0); // Minimum font size is 1
    });

    // Save and Open Buttons
    let save = Button::new("Save").on_click(move |ctx, data: &mut UiState, _| {
        ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()));
    });

    let open = Button::new("Open").on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()));
    });

    // Layout
    let mut col = Flex::column();
    col.add_child(input);
    col.add_spacer(8.0);
    col.add_child(font_size_label);
    col.add_spacer(4.0);

    // Font Size Buttons Row
    let mut font_size_buttons = Flex::row();
    font_size_buttons.add_child(increase_font);
    font_size_buttons.add_spacer(8.0);
    font_size_buttons.add_child(decrease_font);

    col.add_child(font_size_buttons);
    col.add_spacer(8.0);
    col.add_child(save);
    col.add_child(open);

    Align::centered(col)
}

pub struct Delegate;

impl AppDelegate<UiState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut UiState,
        _env: &Env,
    ) -> Handled {
        match cmd {
            cmd if cmd.is(druid::commands::SHOW_SAVE_PANEL) => {
                if let Some(file_info) = cmd.get(SAVE_FILE_AS) {
                    if let Err(err) = fs::write(file_info.path(), &data.input_text) {
                        println!("Error: {}", err);
                    }
                }
                Handled::Yes
            }
            cmd if cmd.is(druid::commands::OPEN_FILE) => {
                if let Some(file_info) = cmd.get(druid::commands::OPEN_FILE) {
                    match fs::read_to_string(file_info.path()) {
                        Ok(contents) => {
                            let first_line = contents.lines().next().unwrap_or("");
                            data.input_text = first_line.to_owned();
                        }
                        Err(_) => {
                            println!("Error: Could not read file");
                        }
                    }
                }
                Handled::Yes
            }
            _ => Handled::No,
        }
    }
}
