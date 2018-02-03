#![feature(asm, lang_items, unwind_attributes)]
#![no_std]
#![no_main]

extern crate attiny2313;

#[no_mangle]
pub extern "C" fn main() {
    // let peripherals = unsafe { attiny2313::Peripherals::steal() };
    let peripherals = unsafe { attiny2313::Peripherals::steal() };

    peripherals
        .PORTB
        .ddrb
        .write(|w| unsafe { w.bits(0xFF) });

    loop {
        // Set all pins on PORTB to high.
        peripherals
            .PORTB
            .portb
            .write(|w| unsafe { w.bits(0xff) });

        small_delay();

        // Set all pins on PORTB to low.
        peripherals
            .PORTB
            .portb
            .write(|w| unsafe { w.bits(0x00) });

        small_delay();
    }
}

/// A small busy loop.
fn small_delay() {
    for _ in 0..400000 {
        unsafe { asm!("" :::: "volatile") }
    }
}

// These do not need to be in a module, but we group them here for clarity.
pub mod std {
    #[lang = "eh_personality"]
    #[no_mangle]
    pub unsafe extern "C" fn rust_eh_personality(
        _state: (),
        _exception_object: *mut (),
        _context: *mut (),
    ) -> () {
    }

    #[lang = "panic_fmt"]
    #[unwind]
    pub extern "C" fn rust_begin_panic(_msg: (), _file: &'static str, _line: u32) -> ! {
        loop {}
    }
}
