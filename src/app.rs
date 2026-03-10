use core::marker::PhantomData;

use esp_hal::delay::Delay;
use ratatui::{
    prelude::Backend,
    style::Stylize,
    widgets::{Block, Paragraph, Wrap},
    Terminal,
};

#[derive(Clone, Copy, Debug)]
pub struct App<B: Backend> {
    counter: u32,
    _marker: PhantomData<B>,
}

impl<B: Backend> App<B> {
    pub fn new() -> Self {
        App {
            counter: 0,
            _marker: PhantomData,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<B>, delay: Delay) {
        loop {
            self.counter += 1;
            delay.delay_millis(100);
            terminal
                .draw(|frame| {
                    let text = format!("Ratatui on embedded devices!\n{}", self.counter);
                    let p1 = Paragraph::new(text.black()).wrap(Wrap { trim: true });
                    let bordered_block = Block::bordered().title("Mousefood");
                    frame.render_widget(p1.block(bordered_block), frame.area());
                })
                .expect("to render frame");
        }
    }
}
