use druid::{
    widget::{Button, Flex, TextBox},
    Widget, WidgetExt, WindowDesc,
};

use crate::state::AppState;

pub fn build_ui() -> impl Widget<AppState> {
    let textbox = TextBox::multiline()
        .with_placeholder("Type something here")
        let font_size_button = Button::new(text)
        .lens(AppState::content);
    let save_button =
        Button::new("Save").on_click(|ctx, data: &mut AppState, _| show_save_dialog(ctx, data));
    let open_button =
        Button::new("Open").on_click(|ctx, data: &mut AppState, _| show_open_dialog(ctx, data));
    let font_size_button = Button::new(text)
    Flex::column().with_child(textbox.expand())
}
