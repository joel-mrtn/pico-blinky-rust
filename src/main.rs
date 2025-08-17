#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m::asm::nop;
use cortex_m_rt::entry;

const LED_PIN: usize = 15;

#[entry]
fn main() -> ! {
    let p = unsafe { rp2040_pac::Peripherals::steal() };

    p.RESETS.reset().modify(|_, w| w.io_bank0().set_bit());
    p.RESETS.reset().modify(|_, w| w.io_bank0().clear_bit());
    while p.RESETS.reset_done().read().io_bank0().bit_is_clear() {}

    p.IO_BANK0
        .gpio(LED_PIN)
        .gpio_ctrl()
        .modify(|_, w| w.funcsel().sio());

    p.SIO
        .gpio_oe_set()
        .write(|w| unsafe { w.bits(1 << LED_PIN) });

    loop {
        p.SIO
            .gpio_out_xor()
            .write(|w| unsafe { w.bits(1 << LED_PIN) });

        for _ in 0..50_000 {
            nop();
        }
    }
}

#[unsafe(link_section = ".boot_loader")]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
