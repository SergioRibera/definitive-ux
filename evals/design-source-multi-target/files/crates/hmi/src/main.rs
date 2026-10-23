slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = Terminal::new()?;
    ui.run()
}
