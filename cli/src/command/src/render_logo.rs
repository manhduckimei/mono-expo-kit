use crate::themes::THEME;

pub fn render_logo() {
    let theme = THEME.lock().unwrap();
    println!("{}", theme.logo_color());
    println!("\nLet's build something awesome!\n");
}
