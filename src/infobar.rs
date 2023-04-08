use crate::os_info;
use console::style;

pub fn show_infobar() {
    let cwd = os_info::current_working_dir();
    let username = os_info::current_username();
    print!(
        "{}->{}{}",
        style(format!("[{}]", username)).green().bold(),
        style(format!("{}", cwd)).cyan().italic(),
        style(" $ ").green().bold()
    );
}
