#![no_std]
#![no_main]

use panic_halt as _;

use core::ptr::{read_volatile, write_volatile};
use cortex_m::asm::nop;
use cortex_m_rt::entry;

const RESETS_RESET: *mut u32 = 0x4000_c000 as *mut u32;
const RESETS_RESET_DONE: *mut u32 = 0x4000_c008 as *mut u32;
const IO_BANK0_GPIO15_CTRL: *mut u32 = 0x4001_407c as *mut u32;
const SIO_GPIO_OE_SET: *mut u32 = 0xd000_0024 as *mut u32;
const SIO_GPIO_OUT_XOR: *mut u32 = 0xd000_001c as *mut u32;

#[unsafe(link_section = ".boot_loader")]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    unsafe {
        let val = read_volatile(RESETS_RESET);
        write_volatile(RESETS_RESET, val & !(1 << 5));
        while (read_volatile(RESETS_RESET_DONE) & (1 << 5)) == 0 {}

        write_volatile(IO_BANK0_GPIO15_CTRL, 5);

        write_volatile(SIO_GPIO_OE_SET, 1 << 15);

        loop {
            write_volatile(SIO_GPIO_OUT_XOR, 1 << 15);
            for _ in 0..50_000 {
                nop();
            }
        }
    }
}
