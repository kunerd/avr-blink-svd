# ! [ doc = "Peripheral access API for ATTINY2313 microcontrollers (generated using svd2rust v0.12.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.12.0/svd2rust/#peripheral-api" ] # ! [ allow ( private_no_mangle_statics ) ] # ! [ deny ( missing_docs ) ] # ! [ deny ( warnings ) ] # ! [ allow ( non_camel_case_types ) ] # ! [ feature ( const_fn ) ] # ! [ no_std ]
// extern crate bare_metal ;
extern crate vcell;
use core::ops::Deref;
use core::marker::PhantomData;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[doc = "I/O Port"]
pub struct PORTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTB {}
impl PORTB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portb::RegisterBlock {
        0x36 as *const _
    }
}
impl Deref for PORTB {
    type Target = portb::RegisterBlock;
    fn deref(&self) -> &portb::RegisterBlock {
        unsafe { &*PORTB::ptr() }
    }
}
#[doc = "I/O Port"]
pub mod portb {
    use vcell::VolatileCell;
    #[doc = r" Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "0x00 - Port B Input Pins"] pub pinb: PINB,
        #[doc = "0x01 - Port B Data Direction Register"] pub ddrb: DDRB,
        #[doc = "0x02 - Port B Data Register"] pub portb: PORTB,
    }
    #[doc = "Port B Data Register"]
    pub struct PORTB {
        register: VolatileCell<u8>,
    }
    #[doc = "Port B Data Register"]
    pub mod portb {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::PORTB {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                self.bits
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
        }
    }
    #[doc = "Port B Data Direction Register"]
    pub struct DDRB {
        register: VolatileCell<u8>,
    }
    #[doc = "Port B Data Direction Register"]
    pub mod ddrb {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::DDRB {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                self.bits
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
        }
    }
    #[doc = "Port B Input Pins"]
    pub struct PINB {
        register: VolatileCell<u8>,
    }
    #[doc = "Port B Input Pins"]
    pub mod pinb {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::PINB {
            #[doc = r" Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                self.bits
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline]
            pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
        }
    }
}
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PORTB"] pub PORTB: PORTB,
}
impl Peripherals {
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PORTB: PORTB {
                _marker: PhantomData,
            },
        }
    }
}
