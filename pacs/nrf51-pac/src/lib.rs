#![doc = "Peripheral access API for NRF51 microcontrollers (generated using svd2rust v0.25.1 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.25.1/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {
    fn POWER_CLOCK();
    fn RADIO();
    fn UART0();
    fn SPI0_TWI0();
    fn SPI1_TWI1();
    fn GPIOTE();
    fn ADC();
    fn TIMER0();
    fn TIMER1();
    fn TIMER2();
    fn RTC0();
    fn TEMP();
    fn RNG();
    fn ECB();
    fn CCM_AAR();
    fn WDT();
    fn RTC1();
    fn QDEC();
    fn LPCOMP();
    fn SWI0();
    fn SWI1();
    fn SWI2();
    fn SWI3();
    fn SWI4();
    fn SWI5();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 26] = [
    Vector {
        _handler: POWER_CLOCK,
    },
    Vector { _handler: RADIO },
    Vector { _handler: UART0 },
    Vector {
        _handler: SPI0_TWI0,
    },
    Vector {
        _handler: SPI1_TWI1,
    },
    Vector { _reserved: 0 },
    Vector { _handler: GPIOTE },
    Vector { _handler: ADC },
    Vector { _handler: TIMER0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _handler: RTC0 },
    Vector { _handler: TEMP },
    Vector { _handler: RNG },
    Vector { _handler: ECB },
    Vector { _handler: CCM_AAR },
    Vector { _handler: WDT },
    Vector { _handler: RTC1 },
    Vector { _handler: QDEC },
    Vector { _handler: LPCOMP },
    Vector { _handler: SWI0 },
    Vector { _handler: SWI1 },
    Vector { _handler: SWI2 },
    Vector { _handler: SWI3 },
    Vector { _handler: SWI4 },
    Vector { _handler: SWI5 },
];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - POWER_CLOCK"]
    POWER_CLOCK = 0,
    #[doc = "1 - RADIO"]
    RADIO = 1,
    #[doc = "2 - UART0"]
    UART0 = 2,
    #[doc = "3 - SPI0_TWI0"]
    SPI0_TWI0 = 3,
    #[doc = "4 - SPI1_TWI1"]
    SPI1_TWI1 = 4,
    #[doc = "6 - GPIOTE"]
    GPIOTE = 6,
    #[doc = "7 - ADC"]
    ADC = 7,
    #[doc = "8 - TIMER0"]
    TIMER0 = 8,
    #[doc = "9 - TIMER1"]
    TIMER1 = 9,
    #[doc = "10 - TIMER2"]
    TIMER2 = 10,
    #[doc = "11 - RTC0"]
    RTC0 = 11,
    #[doc = "12 - TEMP"]
    TEMP = 12,
    #[doc = "13 - RNG"]
    RNG = 13,
    #[doc = "14 - ECB"]
    ECB = 14,
    #[doc = "15 - CCM_AAR"]
    CCM_AAR = 15,
    #[doc = "16 - WDT"]
    WDT = 16,
    #[doc = "17 - RTC1"]
    RTC1 = 17,
    #[doc = "18 - QDEC"]
    QDEC = 18,
    #[doc = "19 - LPCOMP"]
    LPCOMP = 19,
    #[doc = "20 - SWI0"]
    SWI0 = 20,
    #[doc = "21 - SWI1"]
    SWI1 = 21,
    #[doc = "22 - SWI2"]
    SWI2 = 22,
    #[doc = "23 - SWI3"]
    SWI3 = 23,
    #[doc = "24 - SWI4"]
    SWI4 = 24,
    #[doc = "25 - SWI5"]
    SWI5 = 25,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[doc = "Power Control."]
pub struct POWER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for POWER {}
impl POWER {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const power::RegisterBlock = 0x4000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const power::RegisterBlock {
        Self::PTR
    }
}
impl Deref for POWER {
    type Target = power::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for POWER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER").finish()
    }
}
#[doc = "Power Control."]
pub mod power;
#[doc = "Clock control."]
pub struct CLOCK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLOCK {}
impl CLOCK {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const clock::RegisterBlock = 0x4000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const clock::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CLOCK {
    type Target = clock::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCK").finish()
    }
}
#[doc = "Clock control."]
pub mod clock;
#[doc = "The radio."]
pub struct RADIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RADIO {}
impl RADIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const radio::RegisterBlock = 0x4000_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const radio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RADIO {
    type Target = radio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RADIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RADIO").finish()
    }
}
#[doc = "The radio."]
pub mod radio;
#[doc = "Universal Asynchronous Receiver/Transmitter."]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x4000_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0").finish()
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter."]
pub mod uart0;
#[doc = "SPI master 0."]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi0::RegisterBlock = 0x4000_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI0").finish()
    }
}
#[doc = "SPI master 0."]
pub mod spi0;
#[doc = "Two-wire interface master 0."]
pub struct TWI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI0 {}
impl TWI0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const twi0::RegisterBlock = 0x4000_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twi0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TWI0 {
    type Target = twi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TWI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWI0").finish()
    }
}
#[doc = "Two-wire interface master 0."]
pub mod twi0;
#[doc = "SPI master 1."]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi0::RegisterBlock = 0x4000_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI1").finish()
    }
}
#[doc = "SPI master 1."]
pub use spi0 as spi1;
#[doc = "Two-wire interface master 1."]
pub struct TWI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI1 {}
impl TWI1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const twi0::RegisterBlock = 0x4000_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twi0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TWI1 {
    type Target = twi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TWI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWI1").finish()
    }
}
#[doc = "Two-wire interface master 1."]
pub use twi0 as twi1;
#[doc = "SPI slave 1."]
pub struct SPIS1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS1 {}
impl SPIS1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spis1::RegisterBlock = 0x4000_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spis1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPIS1 {
    type Target = spis1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPIS1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPIS1").finish()
    }
}
#[doc = "SPI slave 1."]
pub mod spis1;
#[doc = "GPIO tasks and events."]
pub struct GPIOTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOTE {}
impl GPIOTE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpiote::RegisterBlock = 0x4000_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiote::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOTE {
    type Target = gpiote::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOTE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOTE").finish()
    }
}
#[doc = "GPIO tasks and events."]
pub mod gpiote;
#[doc = "Analog to digital converter."]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adc::RegisterBlock = 0x4000_7000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC").finish()
    }
}
#[doc = "Analog to digital converter."]
pub mod adc;
#[doc = "Timer 0."]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer0::RegisterBlock = 0x4000_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER0").finish()
    }
}
#[doc = "Timer 0."]
pub mod timer0;
#[doc = "Timer 1."]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer0::RegisterBlock = 0x4000_9000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER1 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER1").finish()
    }
}
#[doc = "Timer 1."]
pub use timer0 as timer1;
#[doc = "Timer 2."]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer0::RegisterBlock = 0x4000_a000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER2 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER2").finish()
    }
}
#[doc = "Timer 2."]
pub use timer0 as timer2;
#[doc = "Real time counter 0."]
pub struct RTC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC0 {}
impl RTC0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc0::RegisterBlock = 0x4000_b000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC0 {
    type Target = rtc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC0").finish()
    }
}
#[doc = "Real time counter 0."]
pub mod rtc0;
#[doc = "Temperature Sensor."]
pub struct TEMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TEMP {}
impl TEMP {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const temp::RegisterBlock = 0x4000_c000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const temp::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TEMP {
    type Target = temp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TEMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEMP").finish()
    }
}
#[doc = "Temperature Sensor."]
pub mod temp;
#[doc = "Random Number Generator."]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rng::RegisterBlock = 0x4000_d000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RNG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG").finish()
    }
}
#[doc = "Random Number Generator."]
pub mod rng;
#[doc = "AES ECB Mode Encryption."]
pub struct ECB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECB {}
impl ECB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ecb::RegisterBlock = 0x4000_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ecb::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ECB {
    type Target = ecb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ECB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECB").finish()
    }
}
#[doc = "AES ECB Mode Encryption."]
pub mod ecb;
#[doc = "Accelerated Address Resolver."]
pub struct AAR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AAR {}
impl AAR {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const aar::RegisterBlock = 0x4000_f000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aar::RegisterBlock {
        Self::PTR
    }
}
impl Deref for AAR {
    type Target = aar::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for AAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AAR").finish()
    }
}
#[doc = "Accelerated Address Resolver."]
pub mod aar;
#[doc = "AES CCM Mode Encryption."]
pub struct CCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCM {}
impl CCM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ccm::RegisterBlock = 0x4000_f000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccm::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CCM {
    type Target = ccm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CCM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM").finish()
    }
}
#[doc = "AES CCM Mode Encryption."]
pub mod ccm;
#[doc = "Watchdog Timer."]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wdt::RegisterBlock = 0x4001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WDT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT").finish()
    }
}
#[doc = "Watchdog Timer."]
pub mod wdt;
#[doc = "Real time counter 1."]
pub struct RTC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC1 {}
impl RTC1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc0::RegisterBlock = 0x4001_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC1 {
    type Target = rtc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC1").finish()
    }
}
#[doc = "Real time counter 1."]
pub use rtc0 as rtc1;
#[doc = "Rotary decoder."]
pub struct QDEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QDEC {}
impl QDEC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const qdec::RegisterBlock = 0x4001_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qdec::RegisterBlock {
        Self::PTR
    }
}
impl Deref for QDEC {
    type Target = qdec::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for QDEC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QDEC").finish()
    }
}
#[doc = "Rotary decoder."]
pub mod qdec;
#[doc = "Low power comparator."]
pub struct LPCOMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPCOMP {}
impl LPCOMP {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lpcomp::RegisterBlock = 0x4001_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpcomp::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LPCOMP {
    type Target = lpcomp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LPCOMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCOMP").finish()
    }
}
#[doc = "Low power comparator."]
pub mod lpcomp;
#[doc = "SW Interrupts."]
pub struct SWI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI {}
impl SWI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const swi::RegisterBlock = 0x4001_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const swi::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SWI {
    type Target = swi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SWI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWI").finish()
    }
}
#[doc = "SW Interrupts."]
pub mod swi;
#[doc = "Non Volatile Memory Controller."]
pub struct NVMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMC {}
impl NVMC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const nvmc::RegisterBlock = 0x4001_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nvmc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for NVMC {
    type Target = nvmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for NVMC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVMC").finish()
    }
}
#[doc = "Non Volatile Memory Controller."]
pub mod nvmc;
#[doc = "PPI controller."]
pub struct PPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PPI {}
impl PPI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ppi::RegisterBlock = 0x4001_f000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ppi::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PPI {
    type Target = ppi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PPI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPI").finish()
    }
}
#[doc = "PPI controller."]
pub mod ppi;
#[doc = "Factory Information Configuration."]
pub struct FICR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FICR {}
impl FICR {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ficr::RegisterBlock = 0x1000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ficr::RegisterBlock {
        Self::PTR
    }
}
impl Deref for FICR {
    type Target = ficr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for FICR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FICR").finish()
    }
}
#[doc = "Factory Information Configuration."]
pub mod ficr;
#[doc = "User Information Configuration."]
pub struct UICR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UICR {}
impl UICR {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uicr::RegisterBlock = 0x1000_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uicr::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UICR {
    type Target = uicr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UICR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UICR").finish()
    }
}
#[doc = "User Information Configuration."]
pub mod uicr;
#[doc = "General purpose input and output."]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpio::RegisterBlock = 0x5000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO").finish()
    }
}
#[doc = "General purpose input and output."]
pub mod gpio;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "POWER"]
    pub POWER: POWER,
    #[doc = "CLOCK"]
    pub CLOCK: CLOCK,
    #[doc = "RADIO"]
    pub RADIO: RADIO,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "TWI0"]
    pub TWI0: TWI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "TWI1"]
    pub TWI1: TWI1,
    #[doc = "SPIS1"]
    pub SPIS1: SPIS1,
    #[doc = "GPIOTE"]
    pub GPIOTE: GPIOTE,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "RTC0"]
    pub RTC0: RTC0,
    #[doc = "TEMP"]
    pub TEMP: TEMP,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "ECB"]
    pub ECB: ECB,
    #[doc = "AAR"]
    pub AAR: AAR,
    #[doc = "CCM"]
    pub CCM: CCM,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC1"]
    pub RTC1: RTC1,
    #[doc = "QDEC"]
    pub QDEC: QDEC,
    #[doc = "LPCOMP"]
    pub LPCOMP: LPCOMP,
    #[doc = "SWI"]
    pub SWI: SWI,
    #[doc = "NVMC"]
    pub NVMC: NVMC,
    #[doc = "PPI"]
    pub PPI: PPI,
    #[doc = "FICR"]
    pub FICR: FICR,
    #[doc = "UICR"]
    pub UICR: UICR,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            POWER: POWER {
                _marker: PhantomData,
            },
            CLOCK: CLOCK {
                _marker: PhantomData,
            },
            RADIO: RADIO {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            TWI0: TWI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            TWI1: TWI1 {
                _marker: PhantomData,
            },
            SPIS1: SPIS1 {
                _marker: PhantomData,
            },
            GPIOTE: GPIOTE {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            RTC0: RTC0 {
                _marker: PhantomData,
            },
            TEMP: TEMP {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            ECB: ECB {
                _marker: PhantomData,
            },
            AAR: AAR {
                _marker: PhantomData,
            },
            CCM: CCM {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            RTC1: RTC1 {
                _marker: PhantomData,
            },
            QDEC: QDEC {
                _marker: PhantomData,
            },
            LPCOMP: LPCOMP {
                _marker: PhantomData,
            },
            SWI: SWI {
                _marker: PhantomData,
            },
            NVMC: NVMC {
                _marker: PhantomData,
            },
            PPI: PPI {
                _marker: PhantomData,
            },
            FICR: FICR {
                _marker: PhantomData,
            },
            UICR: UICR {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
        }
    }
}
