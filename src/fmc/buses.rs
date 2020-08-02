use super::{FMCAF, PIN_SPEED};
use crate::gpio::{gpiod::*, gpioe::*, gpiof::*, gpiog::*, gpioh::*, gpioi::*};
pub use crate::stm32::fmc::sdcr::MWID_A;

type A0 = PF0<FMCAF>;
type A1 = PF1<FMCAF>;
type A2 = PF2<FMCAF>;
type A3 = PF3<FMCAF>;
type A4 = PF4<FMCAF>;
type A5 = PF5<FMCAF>;
type A6 = PF12<FMCAF>;
type A7 = PF13<FMCAF>;
type A8 = PF14<FMCAF>;
type A9 = PF15<FMCAF>;
type A10 = PG0<FMCAF>;
type A11 = PG1<FMCAF>;
type A12 = PG2<FMCAF>;

type D0 = PD14<FMCAF>;
type D1 = PD15<FMCAF>;
type D2 = PD0<FMCAF>;
type D3 = PD1<FMCAF>;
type D4 = PE7<FMCAF>;
type D5 = PE8<FMCAF>;
type D6 = PE9<FMCAF>;
type D7 = PE10<FMCAF>;
type D8 = PE11<FMCAF>;
type D9 = PE12<FMCAF>;
type D10 = PE13<FMCAF>;
type D11 = PE14<FMCAF>;
type D12 = PE15<FMCAF>;
type D13 = PD8<FMCAF>;
type D14 = PD9<FMCAF>;
type D15 = PD10<FMCAF>;
type D16 = PH8<FMCAF>;
type D17 = PH9<FMCAF>;
type D18 = PH10<FMCAF>;
type D19 = PH11<FMCAF>;
type D20 = PH12<FMCAF>;
type D21 = PH13<FMCAF>;
type D22 = PH14<FMCAF>;
type D23 = PH15<FMCAF>;
type D24 = PI0<FMCAF>;
type D25 = PI1<FMCAF>;
type D26 = PI2<FMCAF>;
type D27 = PI3<FMCAF>;
type D28 = PI6<FMCAF>;
type D29 = PI0<FMCAF>;
type D30 = PI9<FMCAF>;
type D31 = PI10<FMCAF>;

// todo: macro these

pub enum SDRAMAddressBus {
    AddressBus11 {
        a0: A0,
        a1: A1,
        a2: A2,
        a3: A3,
        a4: A4,
        a5: A5,
        a6: A6,
        a7: A7,
        a8: A8,
        a9: A9,
        a10: A10,
    },
    AddressBus12 {
        a0: A0,
        a1: A1,
        a2: A2,
        a3: A3,
        a4: A4,
        a5: A5,
        a6: A6,
        a7: A7,
        a8: A8,
        a9: A9,
        a10: A10,
        a11: A11,
    },
    AddressBus13 {
        a0: A0,
        a1: A1,
        a2: A2,
        a3: A3,
        a4: A4,
        a5: A5,
        a6: A6,
        a7: A7,
        a8: A8,
        a9: A9,
        a10: A10,
        a11: A11,
        a12: A12,
    },
}

impl SDRAMAddressBus {
    #[inline(always)]
    pub fn prepare_pins(self) -> Self {
        match self {
            SDRAMAddressBus::AddressBus11 {
                a0,
                a1,
                a2,
                a3,
                a4,
                a5,
                a6,
                a7,
                a8,
                a9,
                a10,
            } => SDRAMAddressBus::AddressBus11 {
                a0: a0.set_speed(PIN_SPEED).internal_pull_up(false),
                a1: a1.set_speed(PIN_SPEED).internal_pull_up(false),
                a2: a2.set_speed(PIN_SPEED).internal_pull_up(false),
                a3: a3.set_speed(PIN_SPEED).internal_pull_up(false),
                a4: a4.set_speed(PIN_SPEED).internal_pull_up(false),
                a5: a5.set_speed(PIN_SPEED).internal_pull_up(false),
                a6: a6.set_speed(PIN_SPEED).internal_pull_up(false),
                a7: a7.set_speed(PIN_SPEED).internal_pull_up(false),
                a8: a8.set_speed(PIN_SPEED).internal_pull_up(false),
                a9: a9.set_speed(PIN_SPEED).internal_pull_up(false),
                a10: a10.set_speed(PIN_SPEED).internal_pull_up(false),
            },
            SDRAMAddressBus::AddressBus12 {
                a0,
                a1,
                a2,
                a3,
                a4,
                a5,
                a6,
                a7,
                a8,
                a9,
                a10,
                a11,
            } => SDRAMAddressBus::AddressBus12 {
                a0: a0.set_speed(PIN_SPEED).internal_pull_up(false),
                a1: a1.set_speed(PIN_SPEED).internal_pull_up(false),
                a2: a2.set_speed(PIN_SPEED).internal_pull_up(false),
                a3: a3.set_speed(PIN_SPEED).internal_pull_up(false),
                a4: a4.set_speed(PIN_SPEED).internal_pull_up(false),
                a5: a5.set_speed(PIN_SPEED).internal_pull_up(false),
                a6: a6.set_speed(PIN_SPEED).internal_pull_up(false),
                a7: a7.set_speed(PIN_SPEED).internal_pull_up(false),
                a8: a8.set_speed(PIN_SPEED).internal_pull_up(false),
                a9: a9.set_speed(PIN_SPEED).internal_pull_up(false),
                a10: a10.set_speed(PIN_SPEED).internal_pull_up(false),
                a11: a11.set_speed(PIN_SPEED).internal_pull_up(false),
            },
            SDRAMAddressBus::AddressBus13 {
                a0,
                a1,
                a2,
                a3,
                a4,
                a5,
                a6,
                a7,
                a8,
                a9,
                a10,
                a11,
                a12,
            } => SDRAMAddressBus::AddressBus13 {
                a0: a0.set_speed(PIN_SPEED).internal_pull_up(false),
                a1: a1.set_speed(PIN_SPEED).internal_pull_up(false),
                a2: a2.set_speed(PIN_SPEED).internal_pull_up(false),
                a3: a3.set_speed(PIN_SPEED).internal_pull_up(false),
                a4: a4.set_speed(PIN_SPEED).internal_pull_up(false),
                a5: a5.set_speed(PIN_SPEED).internal_pull_up(false),
                a6: a6.set_speed(PIN_SPEED).internal_pull_up(false),
                a7: a7.set_speed(PIN_SPEED).internal_pull_up(false),
                a8: a8.set_speed(PIN_SPEED).internal_pull_up(false),
                a9: a9.set_speed(PIN_SPEED).internal_pull_up(false),
                a10: a10.set_speed(PIN_SPEED).internal_pull_up(false),
                a11: a11.set_speed(PIN_SPEED).internal_pull_up(false),
                a12: a12.set_speed(PIN_SPEED).internal_pull_up(false),
            },
        }
    }
}

pub enum SDRAMDataBus {
    DataBus8 {
        d0: D0,
        d1: D1,
        d2: D2,
        d3: D3,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
    },
    DataBus16 {
        d0: D0,
        d1: D1,
        d2: D2,
        d3: D3,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
        d8: D8,
        d9: D9,
        d10: D10,
        d11: D11,
        d12: D12,
        d13: D13,
        d14: D14,
        d15: D15,
    },
    DataBus32 {
        d0: D0,
        d1: D1,
        d2: D2,
        d3: D3,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
        d8: D8,
        d9: D9,
        d10: D10,
        d11: D11,
        d12: D12,
        d13: D13,
        d14: D14,
        d15: D15,
        d16: D16,
        d17: D17,
        d18: D18,
        d19: D19,
        d20: D20,
        d21: D21,
        d22: D22,
        d23: D23,
        d24: D24,
        d25: D25,
        d26: D26,
        d27: D27,
        d28: D28,
        d29: D29,
        d30: D30,
        d31: D31,
    },
}

impl SDRAMDataBus {
    #[inline(always)]
    pub fn prepare_pins(self) -> Self {
        match self {
            SDRAMDataBus::DataBus8 {
                d0,
                d1,
                d2,
                d3,
                d4,
                d5,
                d6,
                d7,
            } => SDRAMDataBus::DataBus8 {
                d0: d0.set_speed(PIN_SPEED).internal_pull_up(false),
                d1: d1.set_speed(PIN_SPEED).internal_pull_up(false),
                d2: d2.set_speed(PIN_SPEED).internal_pull_up(false),
                d3: d3.set_speed(PIN_SPEED).internal_pull_up(false),
                d4: d4.set_speed(PIN_SPEED).internal_pull_up(false),
                d5: d5.set_speed(PIN_SPEED).internal_pull_up(false),
                d6: d6.set_speed(PIN_SPEED).internal_pull_up(false),
                d7: d7.set_speed(PIN_SPEED).internal_pull_up(false),
            },
            SDRAMDataBus::DataBus16 {
                d0,
                d1,
                d2,
                d3,
                d4,
                d5,
                d6,
                d7,
                d8,
                d9,
                d10,
                d11,
                d12,
                d13,
                d14,
                d15,
            } => SDRAMDataBus::DataBus16 {
                d0: d0.set_speed(PIN_SPEED).internal_pull_up(false),
                d1: d1.set_speed(PIN_SPEED).internal_pull_up(false),
                d2: d2.set_speed(PIN_SPEED).internal_pull_up(false),
                d3: d3.set_speed(PIN_SPEED).internal_pull_up(false),
                d4: d4.set_speed(PIN_SPEED).internal_pull_up(false),
                d5: d5.set_speed(PIN_SPEED).internal_pull_up(false),
                d6: d6.set_speed(PIN_SPEED).internal_pull_up(false),
                d7: d7.set_speed(PIN_SPEED).internal_pull_up(false),
                d8: d8.set_speed(PIN_SPEED).internal_pull_up(false),
                d9: d9.set_speed(PIN_SPEED).internal_pull_up(false),
                d10: d10.set_speed(PIN_SPEED).internal_pull_up(false),
                d11: d11.set_speed(PIN_SPEED).internal_pull_up(false),
                d12: d12.set_speed(PIN_SPEED).internal_pull_up(false),
                d13: d13.set_speed(PIN_SPEED).internal_pull_up(false),
                d14: d14.set_speed(PIN_SPEED).internal_pull_up(false),
                d15: d15.set_speed(PIN_SPEED).internal_pull_up(false),
            },
            SDRAMDataBus::DataBus32 {
                d0,
                d1,
                d2,
                d3,
                d4,
                d5,
                d6,
                d7,
                d8,
                d9,
                d10,
                d11,
                d12,
                d13,
                d14,
                d15,
                d16,
                d17,
                d18,
                d19,
                d20,
                d21,
                d22,
                d23,
                d24,
                d25,
                d26,
                d27,
                d28,
                d29,
                d30,
                d31,
            } => SDRAMDataBus::DataBus32 {
                d0: d0.set_speed(PIN_SPEED).internal_pull_up(false),
                d1: d1.set_speed(PIN_SPEED).internal_pull_up(false),
                d2: d2.set_speed(PIN_SPEED).internal_pull_up(false),
                d3: d3.set_speed(PIN_SPEED).internal_pull_up(false),
                d4: d4.set_speed(PIN_SPEED).internal_pull_up(false),
                d5: d5.set_speed(PIN_SPEED).internal_pull_up(false),
                d6: d6.set_speed(PIN_SPEED).internal_pull_up(false),
                d7: d7.set_speed(PIN_SPEED).internal_pull_up(false),
                d8: d8.set_speed(PIN_SPEED).internal_pull_up(false),
                d9: d9.set_speed(PIN_SPEED).internal_pull_up(false),
                d10: d10.set_speed(PIN_SPEED).internal_pull_up(false),
                d11: d11.set_speed(PIN_SPEED).internal_pull_up(false),
                d12: d12.set_speed(PIN_SPEED).internal_pull_up(false),
                d13: d13.set_speed(PIN_SPEED).internal_pull_up(false),
                d14: d14.set_speed(PIN_SPEED).internal_pull_up(false),
                d15: d15.set_speed(PIN_SPEED).internal_pull_up(false),
                d16: d16.set_speed(PIN_SPEED).internal_pull_up(false),
                d17: d17.set_speed(PIN_SPEED).internal_pull_up(false),
                d18: d18.set_speed(PIN_SPEED).internal_pull_up(false),
                d19: d19.set_speed(PIN_SPEED).internal_pull_up(false),
                d20: d20.set_speed(PIN_SPEED).internal_pull_up(false),
                d21: d21.set_speed(PIN_SPEED).internal_pull_up(false),
                d22: d22.set_speed(PIN_SPEED).internal_pull_up(false),
                d23: d23.set_speed(PIN_SPEED).internal_pull_up(false),
                d24: d24.set_speed(PIN_SPEED).internal_pull_up(false),
                d25: d25.set_speed(PIN_SPEED).internal_pull_up(false),
                d26: d26.set_speed(PIN_SPEED).internal_pull_up(false),
                d27: d27.set_speed(PIN_SPEED).internal_pull_up(false),
                d28: d28.set_speed(PIN_SPEED).internal_pull_up(false),
                d29: d29.set_speed(PIN_SPEED).internal_pull_up(false),
                d30: d30.set_speed(PIN_SPEED).internal_pull_up(false),
                d31: d31.set_speed(PIN_SPEED).internal_pull_up(false),
            },
        }
    }

    #[inline(always)]
    pub fn mwid(&self) -> MWID_A {
        match self {
            SDRAMDataBus::DataBus8 { .. } => MWID_A::BITS8,
            SDRAMDataBus::DataBus16 { .. } => MWID_A::BITS16,
            SDRAMDataBus::DataBus32 { .. } => MWID_A::BITS32,
        }
    }
}
