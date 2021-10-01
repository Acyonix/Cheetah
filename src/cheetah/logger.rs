use ansi_term::Colour;

pub fn info(content: &str) {
    print!("{}", content);
}

pub fn infoln(content: &str) {
    println!("{}", content);
}

pub fn warn(content: &str) {
    print!("{}", Colour::Yellow.paint(content));
}

pub fn warnln(content: &str) {
    println!("{}", Colour::Yellow.paint(content));
}

pub fn error(content: &str) {
    eprint!("{}", Colour::Red.paint(content));
}

pub fn errorln(content: &str) {
    eprintln!("{}", Colour::Red.paint(content));
}
