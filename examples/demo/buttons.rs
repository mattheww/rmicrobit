use microbit::hal::nrf51;

const BUTTON_A_PIN : usize = 17;
const BUTTON_B_PIN : usize = 26;

const GPIOTE_CHANNEL_BUT_A : usize = 0;
const GPIOTE_CHANNEL_BUT_B : usize = 1;

pub fn initialise_pins(p: &mut nrf51::Peripherals) {
    // This one is used for debouncing
    // Using 31.25kHz clock (32µs ticks)
    // Want about 250ms
    p.TIMER2.prescaler.write(|w| unsafe { w.bits(9) });
    p.TIMER2.cc[0].write(|w| unsafe { w.bits(7_812) });
    p.TIMER2.bitmode.write(|w| w.bitmode()._16bit());
    p.TIMER2.shorts.write(|w| w.compare0_clear().enabled()
                          .compare0_stop().enabled());
    p.TIMER2.intenset.write(|w| w.compare0().set());

    for pin_number in [BUTTON_A_PIN, BUTTON_B_PIN].iter() {
        p.GPIO.pin_cnf[*pin_number].write(
            |w| w.dir().input().input().connect()
        );
    }

    p.GPIOTE.config[GPIOTE_CHANNEL_BUT_A].write(
        |w| unsafe {w.mode().event()
                    .psel().bits(BUTTON_A_PIN as u8)
                    .polarity().hi_to_lo()}
    );
    p.GPIOTE.intenset.write(|w| w.in0().set());

    p.GPIOTE.config[GPIOTE_CHANNEL_BUT_B].write(
        |w| unsafe {w.mode().event()
                    .psel().bits(BUTTON_B_PIN as u8)
                    .polarity().hi_to_lo()}
    );
    p.GPIOTE.intenset.write(|w| w.in1().set());
}

pub fn a_pressed(gpiote: &mut nrf51::GPIOTE,
                 debounce_timer: &mut nrf51::TIMER2) -> bool {
    let event_reg = &gpiote.events_in[GPIOTE_CHANNEL_BUT_A];
    // Button A pressed
    if event_reg.read().bits() != 0 {
        event_reg.write(|w| unsafe {w.bits(0)} );
        // Ignore button A interrupt for 250ms
        gpiote.intenclr.write(|w| w.in0().clear());
        debounce_timer.tasks_start.write(|w| unsafe { w.bits(1) });
        true
    } else {
        false
    }
}

pub fn b_pressed(gpiote: &mut nrf51::GPIOTE,
                 debounce_timer: &mut nrf51::TIMER2) -> bool {
    let event_reg = &gpiote.events_in[GPIOTE_CHANNEL_BUT_B];
    // Button B pressed
    if event_reg.read().bits() != 0 {
        event_reg.write(|w| unsafe {w.bits(0)} );
        // Ignore button B interrupt for 250ms
        gpiote.intenclr.write(|w| w.in1().clear());
        debounce_timer.tasks_start.write(|w| unsafe { w.bits(1) });
        true
    } else {
        false
    }
}

pub fn handle_debounce_timer(gpiote: &mut nrf51::GPIOTE,
                             debounce_timer: &mut nrf51::TIMER2) {
    debounce_timer.events_compare[0].write(|w| unsafe {w.bits(0)} );
    // Ignore any button A events that have happened while we were
    // ignoring the interrupt, and re-enable button A interrupt.
    gpiote.events_in[GPIOTE_CHANNEL_BUT_A].write(|w| unsafe {w.bits(0)});
    gpiote.intenset.write(|w| w.in0().set());
    // Ignore any button B events that have happened while we were
    // ignoring the interrupt, and re-enable button B interrupt.
    gpiote.events_in[GPIOTE_CHANNEL_BUT_B].write(|w| unsafe {w.bits(0)});
    gpiote.intenset.write(|w| w.in1().set());
}

