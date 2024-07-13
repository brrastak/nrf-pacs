#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start RTC counter"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop RTC counter"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Clear RTC counter"]
    pub tasks_clear: TASKS_CLEAR,
    #[doc = "0x0c - Set counter to 0xFFFFF0"]
    pub tasks_trigovrflw: TASKS_TRIGOVRFLW,
    _reserved4: [u8; 0x70],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    #[doc = "0x88 - Subscribe configuration for task CLEAR"]
    pub subscribe_clear: SUBSCRIBE_CLEAR,
    #[doc = "0x8c - Subscribe configuration for task TRIGOVRFLW"]
    pub subscribe_trigovrflw: SUBSCRIBE_TRIGOVRFLW,
    _reserved8: [u8; 0x70],
    #[doc = "0x100 - Event on counter increment"]
    pub events_tick: EVENTS_TICK,
    #[doc = "0x104 - Event on counter overflow"]
    pub events_ovrflw: EVENTS_OVRFLW,
    _reserved10: [u8; 0x38],
    #[doc = "0x140..0x150 - Description collection: Compare event on CC\\[n\\]
match"]
    pub events_compare: [EVENTS_COMPARE; 4],
    _reserved11: [u8; 0x30],
    #[doc = "0x180 - Publish configuration for event TICK"]
    pub publish_tick: PUBLISH_TICK,
    #[doc = "0x184 - Publish configuration for event OVRFLW"]
    pub publish_ovrflw: PUBLISH_OVRFLW,
    _reserved13: [u8; 0x38],
    #[doc = "0x1c0..0x1d0 - Description collection: Publish configuration for event COMPARE\\[n\\]"]
    pub publish_compare: [PUBLISH_COMPARE; 4],
    _reserved14: [u8; 0x0134],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved16: [u8; 0x34],
    #[doc = "0x340 - Enable or disable event routing"]
    pub evten: EVTEN,
    #[doc = "0x344 - Enable event routing"]
    pub evtenset: EVTENSET,
    #[doc = "0x348 - Disable event routing"]
    pub evtenclr: EVTENCLR,
    _reserved19: [u8; 0x01b8],
    #[doc = "0x504 - Current counter value"]
    pub counter: COUNTER,
    #[doc = "0x508 - 12-bit prescaler for counter frequency (32768/(PRESCALER+1)). Must be written when RTC is stopped."]
    pub prescaler: PRESCALER,
    _reserved21: [u8; 0x34],
    #[doc = "0x540..0x550 - Description collection: Compare register n"]
    pub cc: [CC; 4],
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start RTC counter"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop RTC counter"]
pub mod tasks_stop;
#[doc = "TASKS_CLEAR (w) register accessor: an alias for `Reg<TASKS_CLEAR_SPEC>`"]
pub type TASKS_CLEAR = crate::Reg<tasks_clear::TASKS_CLEAR_SPEC>;
#[doc = "Clear RTC counter"]
pub mod tasks_clear;
#[doc = "TASKS_TRIGOVRFLW (w) register accessor: an alias for `Reg<TASKS_TRIGOVRFLW_SPEC>`"]
pub type TASKS_TRIGOVRFLW = crate::Reg<tasks_trigovrflw::TASKS_TRIGOVRFLW_SPEC>;
#[doc = "Set counter to 0xFFFFF0"]
pub mod tasks_trigovrflw;
#[doc = "SUBSCRIBE_START (rw) register accessor: an alias for `Reg<SUBSCRIBE_START_SPEC>`"]
pub type SUBSCRIBE_START = crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "SUBSCRIBE_CLEAR (rw) register accessor: an alias for `Reg<SUBSCRIBE_CLEAR_SPEC>`"]
pub type SUBSCRIBE_CLEAR = crate::Reg<subscribe_clear::SUBSCRIBE_CLEAR_SPEC>;
#[doc = "Subscribe configuration for task CLEAR"]
pub mod subscribe_clear;
#[doc = "SUBSCRIBE_TRIGOVRFLW (rw) register accessor: an alias for `Reg<SUBSCRIBE_TRIGOVRFLW_SPEC>`"]
pub type SUBSCRIBE_TRIGOVRFLW = crate::Reg<subscribe_trigovrflw::SUBSCRIBE_TRIGOVRFLW_SPEC>;
#[doc = "Subscribe configuration for task TRIGOVRFLW"]
pub mod subscribe_trigovrflw;
#[doc = "EVENTS_TICK (rw) register accessor: an alias for `Reg<EVENTS_TICK_SPEC>`"]
pub type EVENTS_TICK = crate::Reg<events_tick::EVENTS_TICK_SPEC>;
#[doc = "Event on counter increment"]
pub mod events_tick;
#[doc = "EVENTS_OVRFLW (rw) register accessor: an alias for `Reg<EVENTS_OVRFLW_SPEC>`"]
pub type EVENTS_OVRFLW = crate::Reg<events_ovrflw::EVENTS_OVRFLW_SPEC>;
#[doc = "Event on counter overflow"]
pub mod events_ovrflw;
#[doc = "EVENTS_COMPARE (rw) register accessor: an alias for `Reg<EVENTS_COMPARE_SPEC>`"]
pub type EVENTS_COMPARE = crate::Reg<events_compare::EVENTS_COMPARE_SPEC>;
#[doc = "Description collection: Compare event on CC\\[n\\]
match"]
pub mod events_compare;
#[doc = "PUBLISH_TICK (rw) register accessor: an alias for `Reg<PUBLISH_TICK_SPEC>`"]
pub type PUBLISH_TICK = crate::Reg<publish_tick::PUBLISH_TICK_SPEC>;
#[doc = "Publish configuration for event TICK"]
pub mod publish_tick;
#[doc = "PUBLISH_OVRFLW (rw) register accessor: an alias for `Reg<PUBLISH_OVRFLW_SPEC>`"]
pub type PUBLISH_OVRFLW = crate::Reg<publish_ovrflw::PUBLISH_OVRFLW_SPEC>;
#[doc = "Publish configuration for event OVRFLW"]
pub mod publish_ovrflw;
#[doc = "PUBLISH_COMPARE (rw) register accessor: an alias for `Reg<PUBLISH_COMPARE_SPEC>`"]
pub type PUBLISH_COMPARE = crate::Reg<publish_compare::PUBLISH_COMPARE_SPEC>;
#[doc = "Description collection: Publish configuration for event COMPARE\\[n\\]"]
pub mod publish_compare;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "EVTEN (rw) register accessor: an alias for `Reg<EVTEN_SPEC>`"]
pub type EVTEN = crate::Reg<evten::EVTEN_SPEC>;
#[doc = "Enable or disable event routing"]
pub mod evten;
#[doc = "EVTENSET (rw) register accessor: an alias for `Reg<EVTENSET_SPEC>`"]
pub type EVTENSET = crate::Reg<evtenset::EVTENSET_SPEC>;
#[doc = "Enable event routing"]
pub mod evtenset;
#[doc = "EVTENCLR (rw) register accessor: an alias for `Reg<EVTENCLR_SPEC>`"]
pub type EVTENCLR = crate::Reg<evtenclr::EVTENCLR_SPEC>;
#[doc = "Disable event routing"]
pub mod evtenclr;
#[doc = "COUNTER (r) register accessor: an alias for `Reg<COUNTER_SPEC>`"]
pub type COUNTER = crate::Reg<counter::COUNTER_SPEC>;
#[doc = "Current counter value"]
pub mod counter;
#[doc = "PRESCALER (rw) register accessor: an alias for `Reg<PRESCALER_SPEC>`"]
pub type PRESCALER = crate::Reg<prescaler::PRESCALER_SPEC>;
#[doc = "12-bit prescaler for counter frequency (32768/(PRESCALER+1)). Must be written when RTC is stopped."]
pub mod prescaler;
#[doc = "CC (rw) register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Description collection: Compare register n"]
pub mod cc;
