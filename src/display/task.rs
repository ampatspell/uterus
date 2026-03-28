use defmt::info;
use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, iso_8859_5::FONT_9X18_BOLD},
    prelude::*,
    primitives::{PrimitiveStyle, StyledDrawable},
    text::Text,
};
use epd_waveshare::{color::Color, prelude::RefreshLut};
use no_std_strings::str16;

use crate::{
    channels::{
        RANDOM,
        Random::{self},
    },
    display::{DisplayPins, display::Display},
};

#[embassy_executor::task]
pub async fn display_task(pins: DisplayPins) {
    info!("start display_task");

    let mut display = Display::new(pins);

    let clear = |display: &mut Display| {
        let style = PrimitiveStyle::with_fill(Color::White);
        display.bounding_box().draw_styled(&style, display);
    };

    display.set_lut(RefreshLut::Full);
    clear(&mut display);

    let style = MonoTextStyle::new(&FONT_9X18_BOLD, Color::Black);
    let label = str16::from("Uterus");

    display.update_and_display();
    display.set_lut(RefreshLut::Quick);

    loop {
        clear(&mut display);
        let x = 10;
        let y = 25;

        Text::new(label.to_str(), Point::new(x, y), style)
            .draw(&mut display)
            .unwrap();

        display.update_and_display();

        let message = RANDOM.receive().await;
        match message {
            Random::Button { .. } => {}
        }
    }
}
