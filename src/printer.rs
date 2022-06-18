
pub trait Print {
    fn print(&self, printer: &mut Printer);
}

pub struct Printer {
    code: String,
}
impl Printer {
    pub fn new() -> Self {
        Self {
            code: String::new()
        }
    }
    fn esc(&mut self) {
        self.code.push_str(&format!("{}", 27 as char));
    }
    pub fn csi(&mut self) {
        self.code.push_str(&format!("{}[", 27 as char));
    }
    pub fn clear(&mut self) {
        self.csi();
        self.code.push_str("2J");
    }
    pub fn clear_line(&mut self) {
        self.csi();
        self.code.push_str("2K");
    }
    pub fn clear_end(&mut self) {
        self.csi();
        self.code.push_str("0J");
    }
    pub fn reset(&mut self) {
        self.csi();
        self.code.push_str("0m");
    }
    pub fn repos(&mut self, row: u8, col: u8) {
        self.csi();
        self.code.push_str(&format!("{};{}H", row, col));
    }
    pub fn repos_prev_line(&mut self) {
        self.csi();
        self.code.push_str("2F");
    }
    pub fn repos_next_line(&mut self) {
        self.csi();
        self.code.push_str("2E");
    }
    pub fn alt_on(&mut self) {
        self.csi();
        self.code.push_str("?1049h");
    }
    pub fn alt_off(&mut self) {
        self.csi();
        self.code.push_str("?1049l");
    }
    pub fn color_fg(&mut self, color: u8) {
        self.csi();
        self.code.push_str(&format!("38;5;{}m", color));  // set foreground red
    }
    pub fn color_bg(&mut self, color: u8) {
        self.csi();
        self.code.push_str(&format!("48;5;{}m", color));  // set background red
    }
    pub fn color_fg_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.csi();
        self.code.push_str(&format!("38;2;{};{};{}m", r, g, b));  // set foreground red
    }
    pub fn color_bg_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.csi();
        self.code.push_str(&format!("48;2;{};{};{}m", r, g, b));  // set background red
    }
    pub fn write(&mut self, text: &str) {
        self.code.push_str(&format!("{}", text));
    }
    pub fn newline(&mut self) {
        self.code.push_str("\n");
    }
    fn erase(&mut self) {
        self.code.clear();
    }
    pub fn output(&mut self) {
        print!("{}", self.code);
        // println!("{:?}", self.code);
        self.erase();
    }
}
