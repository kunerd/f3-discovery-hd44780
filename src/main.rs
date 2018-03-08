#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate f3;

extern crate clerk;

mod extern_pin;

use f3::hal::delay;
use f3::hal::prelude::*;
use f3::hal::stm32f30x::{Peripherals};
use clerk::{Delay, CursorBlinking, CursorState, DataPins4Lines, DefaultLines, Display,
            DisplayControlBuilder, DisplayState, FunctionSetBuilder, LineNumber, Pins, SeekFrom};

use extern_pin::ExternPin;

pub struct CustomDelay;

impl Delay for CustomDelay {}

fn main() {
    let p = Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    let mut gpiod = p.GPIOD.split(&mut rcc.ahb);
    let pd0 = gpiod
        .pd0
        .into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);

    let pd1 = gpiod
        .pd1
        .into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);

    let pd2 = gpiod
        .pd2
        .into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);

    let pd3 = gpiod
        .pd3
        .into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);

    let pd4 = gpiod
        .pd4
        .into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);

    let pd5 = gpiod
        .pd5
        .into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);

    let pd6 = gpiod
        .pd6
        .into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);

    // pd0.set_high();

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let delay = delay::Delay::new(cp.SYST, clocks);

    let pins = Pins {
        register_select: ExternPin::new(pd0),
        read: ExternPin::new(pd1),
        enable: ExternPin::new(pd2),
        data: DataPins4Lines {
            data4: ExternPin::new(pd3),
            data5: ExternPin::new(pd4),
            data6: ExternPin::new(pd5),
            data7: ExternPin::new(pd6),
        },
    };

    let mut lcd: Display<_, DefaultLines> = Display::new(pins.into_connection::<CustomDelay, delay::Delay>(delay));

    lcd.init(FunctionSetBuilder::default().set_line_number(LineNumber::Two));

    lcd.set_display_control(
        DisplayControlBuilder::default()
            .set_display(DisplayState::On)
            .set_cursor(CursorState::Off)
            .set_cursor_blinking(CursorBlinking::On),
    );

    lcd.write_message("Hello");

    lcd.seek(SeekFrom::Line {
        line: DefaultLines::Two,
        offset: 3,
    });

    lcd.write_message("F3 Discovery!");

    loop {}
}
