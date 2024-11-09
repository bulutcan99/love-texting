use druid::{AppLauncher, LocalizedString, WindowDesc};
use ui::{ui_builder, Delegate};

pub mod command;
pub mod state;
pub mod ui;

//TODO: commands olarak kendi komutlarimizi yazacagiz
//daha moduler hale getirecegiz
//ve not defteri formatina benzetecegiz.

//TODO: state olarak dosya islemlerini yapacagiz
//dosya acma, dosya kaydetme, dosya silme, dosya guncelleme

//TODO: ui olarak islemleri ayirip builderda birlestirecegiz
fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title(LocalizedString::new("open-save-demo").with_placeholder("Opening/Saving Demo"));
    let data = "Type here.".to_owned();
    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .log_to_console()
        .launch(data)
        .expect("launch failed");
}
