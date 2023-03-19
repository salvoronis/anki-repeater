use shaku::{Interface, Component};

pub trait Logger: Interface{
    fn log(&self, content: &str);
}

#[derive(Component)]
#[shaku(interface = Logger)]
pub struct LoggerImpl;

impl Logger for LoggerImpl {
    fn log(&self, content: &str) {
        println!("{}", content);
    }
}