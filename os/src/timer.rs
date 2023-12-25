const MSEC_PER_SEC: usize = 1000;
const TICKS_PER_SEC: usize = 100;
use crate::sbi::set_timer;
use crate::config::CLOCK_FREQ;
use riscv::register::time;


pub fn get_time() -> usize {
        time::read()
}

pub fn set_next_trigger() {
        set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}



pub fn get_time_ms() -> usize {
        time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}
