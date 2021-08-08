// Refer to: https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences?redirectedfrom=MSDN
// Syntax: \u{0001b}[38;2;R;G;BmTEXT\u{0001b}[0m
// With Background Color: \u{0001b}[48;2;R;G;B;38;2;R;G;BmTEXT\u{0001b}[0m
// 001b  : Unicode Character 'ESCAPE' (U+0001B)
// Or use \x1b

struct Color(u8, u8, u8);
struct StringColorBuilder {
    text: String,
    color: Color,
    back: Color,
}
impl StringColorBuilder {
    fn new(text: String) -> Self {
        Self {
            text,
            color: Color(255, 255, 255),
            back: Color(0, 0, 0),
        }
    }
    fn set_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    fn set_b_color(mut self, color: Color) -> Self {
        self.back = color;
        self
    }
    fn build(self) -> StringColor {
        StringColor {
            text: self.text,
            color: self.color,
            back: self.back,
        }
    }
}
struct StringColor {
    text: String,
    color: Color,
    back: Color,
}
impl std::fmt::Display for StringColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\u{0001b}[48;2;{};{};{};38;2;{};{};{}m{}\u{0001b}[0m",
            self.back.0,
            self.back.1,
            self.back.0,
            self.color.0,
            self.color.1,
            self.color.2,
            self.text
        )
    }
}

fn main() {
    let print_me = StringColorBuilder::new("ABC Its easy as 123".to_string())
        .set_color(Color(255, 255, 0))
        .set_b_color(Color(0, 255, 0))
        .build();
    println!("{}", print_me);
    println!("{}", print_me);
}
