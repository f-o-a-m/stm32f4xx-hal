use crate::gpio::gpiob::{PB5, PB6};
use crate::gpio::gpioc::{PC0, PC2, PC3};
use crate::gpio::gpioh::{PH2, PH3, PH5, PH6, PH7};
use crate::stm32::FMC;

mod buses;
pub use buses::*;

use crate::gpio::gpioe::{PE0, PE1};
use crate::gpio::gpiof::PF11;
use crate::gpio::gpiog::{PG15, PG4, PG5, PG8};
use crate::gpio::gpioi::{PI4, PI5};
pub use crate::stm32::fmc::{
    sdcmr::MODE_AW,
    sdcr::{CAS_A, MWID_A, NB_A, NC_A, NR_A, RPIPE_A, SDCLK_A},
    SDCMR, SDCR, SDRTR, SDSR, SDTR,
};

type FMCAF = crate::gpio::Alternate<crate::gpio::AF12>;

const PIN_SPEED: crate::gpio::Speed = crate::gpio::Speed::High;

macro_rules! fmc_gpio {
    ($x:ident) => {
        $x.into_alternate_af12()
            .set_speed(crate::gpio::Speed::High)
            .internal_pull_up(false)
    };
    ($gpioX:ident.$pXY:ident) => {
        $gpioX
            .$pXY
            .into_alternate_af12()
            .set_speed(crate::gpio::Speed::High)
            .internal_pull_up(false)
    };
}

type NBL0 = PE0<FMCAF>;
type NBL1 = PE1<FMCAF>;
type NBL2 = PI4<FMCAF>;
type NBL3 = PI5<FMCAF>;

pub trait ByteEnable {
    fn prepare_pins(self) -> Self;
}

impl ByteEnable for () {
    #[inline(always)]
    fn prepare_pins(self) -> Self {
        ()
    }
}

impl ByteEnable for (NBL0, NBL1) {
    #[inline(always)]
    fn prepare_pins(self) -> Self {
        let (_0, _1) = self;
        (fmc_gpio!(_0), fmc_gpio!(_1))
    }
}

impl ByteEnable for (NBL0, NBL1, NBL2, NBL3) {
    #[inline(always)]
    fn prepare_pins(self) -> Self {
        let (_0, _1, _2, _3) = self;
        (fmc_gpio!(_0), fmc_gpio!(_1), fmc_gpio!(_2), fmc_gpio!(_3))
    }
}

type BA0 = PG4<FMCAF>;
type BA1 = PG5<FMCAF>;

pub trait BankAccess {
    fn prepare_pins(self) -> Self;
    fn number_of_banks() -> NB_A;
}

impl BankAccess for BA0 {
    #[inline(always)]
    fn prepare_pins(self) -> Self {
        self.set_speed(PIN_SPEED).internal_pull_up(false)
    }

    #[inline(always)]
    fn number_of_banks() -> NB_A {
        NB_A::NB2
    }
}

impl BankAccess for (BA0, BA1) {
    #[inline(always)]
    fn prepare_pins(self) -> Self {
        (
            self.0.set_speed(PIN_SPEED).internal_pull_up(false),
            self.1.set_speed(PIN_SPEED).internal_pull_up(false),
        )
    }

    #[inline(always)]
    fn number_of_banks() -> NB_A {
        NB_A::NB4
    }
}

/// (internal) which SDRAM bank to use
/// determined based on which combination of
/// SDCKE and SDNE pins are used (i.e., 0+0 uses Bank1, 1+1 uses Bank2)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SDRAMInternalBankSelection {
    Bank1,
    Bank2,
}

macro_rules! fmc_pinswappable {
    ($SD_PIN_NAME:ident, [ $($GPIO_PIN_TYPE:ty),+ ]) => {
        pub trait $SD_PIN_NAME {
            fn prepare_pins(self) -> Self;
        }
        $(
            impl $SD_PIN_NAME for $GPIO_PIN_TYPE {
                fn prepare_pins(self) -> Self {
                    self.set_speed(crate::gpio::Speed::High).internal_pull_up(false)
                }
            }
        )+
    }
}

fmc_pinswappable!(PinSDNWE,  [PC0<FMCAF>, PH5<FMCAF>]);
fmc_pinswappable!(PinSDCKE0, [PH2<FMCAF>, PC3<FMCAF>]);
fmc_pinswappable!(PinSDCKE1, [PH7<FMCAF>, PB5<FMCAF>]);
fmc_pinswappable!(PinSDNE0,  [PH3<FMCAF>, PC2<FMCAF>]);
fmc_pinswappable!(PinSDNE1,  [PH6<FMCAF>, PB6<FMCAF>]);

type SDCLK = PG8<FMCAF>;
type SDNRAS = PF11<FMCAF>;
type SDNCAS = PG15<FMCAF>;

pub trait SDRAMClockAndChipEnable {
    fn sdram_bank_selection() -> SDRAMInternalBankSelection;
    fn prepare_pins(self) -> Self;
}

pub struct UseSDRAMBank1<SDCKE0: PinSDCKE0, SDNE0: PinSDNE0>(pub SDCKE0, pub SDNE0);
pub struct UseSDRAMBank2<SDCKE1: PinSDCKE1, SDNE1: PinSDNE1>(pub SDCKE1, pub SDNE1);

impl<SDCKE0: PinSDCKE0, SDNE0: PinSDNE0> SDRAMClockAndChipEnable for UseSDRAMBank1<SDCKE0, SDNE0> {
    #[inline(always)]
    fn sdram_bank_selection() -> SDRAMInternalBankSelection {
        SDRAMInternalBankSelection::Bank1
    }

    #[inline(always)]
    fn prepare_pins(self) -> Self {
        UseSDRAMBank1(self.0.prepare_pins(), self.1.prepare_pins())
    }
}

impl<SDCKE1: PinSDCKE1, SDNE1: PinSDNE1> SDRAMClockAndChipEnable for UseSDRAMBank2<SDCKE1, SDNE1> {
    #[inline(always)]
    fn sdram_bank_selection() -> SDRAMInternalBankSelection {
        SDRAMInternalBankSelection::Bank2
    }

    #[inline(always)]
    fn prepare_pins(self) -> Self {
        UseSDRAMBank2(self.0.prepare_pins(), self.1.prepare_pins())
    }
}

pub struct SDRAMConfig {
    pub address_bus: SDRAMAddressBus,
    pub data_bus: SDRAMDataBus,
    pub number_of_column_bits: NC_A,
    pub number_of_row_bits: NR_A,

    pub sdclk_period: SDCLK_A,
    pub rpipe_delay: RPIPE_A,
    pub write_protection: bool,
    pub read_burst: bool,
    pub timing: SDRAMTiming,
}

/// Timing configurations for SDRAM. All units in SDRAM clocks (confirm?)
/// todo: use a nibble type for these if possible...
pub struct SDRAMTiming {
    pub cas_latency: CAS_A,
    /// LOAD MODE REGISTER command to ACTIVE command delay (sometimes called t_mrd).
    /// "Load mode register to active delay" in CubeMX
    pub t_mrd: u8,
    /// Exit SELF_REFRESH-to-ACTIVE command delay
    /// "Exit self-refresh delay" in CubeMX
    pub t_xsr: u8,
    /// Self refresh time (as named in CubeMX, tRAS in some datasheets)
    pub t_ras: u8,
    /// ACTIVE-to-ACTIVE command period (sometimes called tRC)
    /// "SDRAM common row cycle delay" in CubeMX
    pub t_rc: u8,
    /// WRITE recovery time (as named in CubeMX, tWR in datasheets)
    pub t_wr: u8,
    /// PRECHARGE command period (tRP in datasheets)
    /// "SDRAM common row precharge delay" in CubeMX
    pub t_rp: u8,
    /// ACTIVE-to-READ or Write delay (tRCD in datasheets)
    /// "Row to column delay" in CubeMX
    pub t_rcd: u8,
}

impl SDRAMConfig {
    fn configure_sdcr<CCE: SDRAMClockAndChipEnable, BA: BankAccess>(&self, fmc: FMC) -> FMC {
        // going off the HAL code, seems like if you use Bank1, you just set everything in SDCR1
        // but if you use Bank2 then sdclk, rburst, and rpipe go in SDCR1 and everything else into SDCR2
        // todo: STM32 LL api clears all of these (i.e., sets to 0 regardless of what the startup default is),
        // todo: including parts of SDCR2 that we don't touch here. This should probably be verified...

        // apply remaining values to the appropriate SDCR
        match CCE::sdram_bank_selection() {
            SDRAMInternalBankSelection::Bank1 => {
                fmc.sdcr1.write_with_zero(|w| {
                    w.sdclk()
                        .variant(self.sdclk_period)
                        .rburst()
                        .bit(self.read_burst)
                        .rpipe()
                        .variant(self.rpipe_delay)
                        .nc()
                        .variant(self.number_of_column_bits)
                        .nr()
                        .variant(self.number_of_row_bits)
                        .mwid()
                        .variant(self.data_bus.mwid())
                        .nb()
                        .variant(BA::number_of_banks())
                        .cas()
                        .variant(self.timing.cas_latency)
                        .wp()
                        .bit(self.write_protection)
                });
            }
            SDRAMInternalBankSelection::Bank2 => {
                fmc.sdcr1.modify(|_, w| {
                    w.sdclk()
                        .variant(self.sdclk_period)
                        .rburst()
                        .bit(self.read_burst)
                        .rpipe()
                        .variant(self.rpipe_delay)
                });
                fmc.sdcr2.write_with_zero(|w| {
                    w.nc()
                        .variant(self.number_of_column_bits)
                        .nr()
                        .variant(self.number_of_row_bits)
                        .mwid()
                        .variant(self.data_bus.mwid())
                        .nb()
                        .variant(BA::number_of_banks())
                        .cas()
                        .variant(self.timing.cas_latency)
                        .wp()
                        .bit(self.write_protection)
                });
            }
        };

        fmc
    }

    fn configure_sdtr<CCE: SDRAMClockAndChipEnable>(&self, fmc: FMC) -> FMC {
        // similar to SDCR above, TRC and TRP always get set in Bank 1 even if you're using bank 2
        // also we do a bunch of "-1" to the values specified in timing, this is to maintain
        // parity with HAL (i.e., you specify actual desired timings, HAL makes sure the FMC uses them)
        match CCE::sdram_bank_selection() {
            SDRAMInternalBankSelection::Bank1 => {
                fmc.sdtr1.write_with_zero(|w| {
                    w.trc()
                        .bits(self.timing.t_rc - 1)
                        .trp()
                        .bits(self.timing.t_rp - 1)
                        .tmrd()
                        .bits(self.timing.t_mrd - 1)
                        .txsr()
                        .bits(self.timing.t_xsr - 1)
                        .tras()
                        .bits(self.timing.t_ras - 1)
                        .twr()
                        .bits(self.timing.t_wr - 1)
                        .trcd()
                        .bits(self.timing.t_rcd - 1)
                });
            }
            SDRAMInternalBankSelection::Bank2 => {
                fmc.sdtr1.modify(|_, w| {
                    w.trc()
                        .bits(self.timing.t_rc - 1)
                        .trp()
                        .bits(self.timing.t_rp - 1)
                });
                fmc.sdtr2.write_with_zero(|w| {
                    w.tmrd()
                        .bits(self.timing.t_mrd - 1)
                        .txsr()
                        .bits(self.timing.t_xsr - 1)
                        .tras()
                        .bits(self.timing.t_ras - 1)
                        .twr()
                        .bits(self.timing.t_wr - 1)
                        .trcd()
                        .bits(self.timing.t_rcd - 1)
                });
            }
        };

        fmc
    }

    pub fn configure<
        CCE: SDRAMClockAndChipEnable,
        SDNWE: PinSDNWE,
        TBA: BankAccess,
        TNBL: ByteEnable,
    >(
        mut self,
        fmc: FMC,
        bank_access: TBA,
        byte_enable: TNBL,
        sdclk: SDCLK,
        sdnras: SDNRAS,
        sdncas: SDNCAS,
        clock_and_chip_enable: CCE,
        sdnwe: SDNWE,
    ) -> SDRAM {
        let peripherals = unsafe { crate::stm32::Peripherals::steal() };
        let rcc = peripherals.RCC;

        // Enable the FMC peripheral clock, appears to be the only bit in AHB3ENR
        rcc.ahb3enr.modify(|_, w| w.fmcen().enabled());

        self.address_bus = self.address_bus.prepare_pins();
        self.data_bus = self.data_bus.prepare_pins();
        bank_access.prepare_pins();
        byte_enable.prepare_pins();
        fmc_gpio!(sdclk);
        fmc_gpio!(sdncas);
        fmc_gpio!(sdnras);
        clock_and_chip_enable.prepare_pins();
        sdnwe.prepare_pins();

        let fmc = self.configure_sdcr::<CCE, TBA>(fmc);
        let fmc = self.configure_sdtr::<CCE>(fmc);

        SDRAM {
            bank_selection: CCE::sdram_bank_selection(),
            fmc,
        }
    }
}

pub struct SDRAM {
    bank_selection: SDRAMInternalBankSelection,
    fmc: FMC,
}

impl SDRAM {
    pub fn send_command(&mut self, cmd: SDRAMCommand) {
        let (ctb1, ctb2) = match cmd.target_bank {
            SDRAMCommandTargetBank::None => (false, false),
            SDRAMCommandTargetBank::Bank1 => (true, false),
            SDRAMCommandTargetBank::Bank2 => (false, true),
            SDRAMCommandTargetBank::Both => (true, true),
        };
        self.fmc.sdcmr.write_with_zero(|w| {
            w.mode()
                .variant(cmd.command_mode)
                .ctb1()
                .bit(ctb1)
                .ctb2()
                .bit(ctb2)
                .nrfs()
                .bits(cmd.auto_refresh_number)
                .mrd()
                .bits(cmd.mode_register_definition)
        });
        loop {
            if !self.fmc.sdsr.read().busy().is_busy() {
                break;
            }
        }
    }

    pub fn is_write_protect(&self) -> bool {
        let sdcr = match self.bank_selection {
            SDRAMInternalBankSelection::Bank1 => &self.fmc.sdcr1,
            SDRAMInternalBankSelection::Bank2 => &self.fmc.sdcr2,
        };
        sdcr.read().wp().bit()
    }

    pub fn set_write_protect(&mut self, enabled: bool) {
        let sdcr = match self.bank_selection {
            SDRAMInternalBankSelection::Bank1 => &self.fmc.sdcr1,
            SDRAMInternalBankSelection::Bank2 => &self.fmc.sdcr2,
        };
        sdcr.modify(|_, w| w.wp().bit(enabled))
    }

    pub fn set_autorefresh_number(&mut self, nrfs: u8) {
        self.fmc.sdcmr.modify(|_, w| w.nrfs().bits(nrfs))
    }

    pub fn program_refresh_rate(&mut self, refresh_rate: u16) {
        self.fmc.sdrtr.modify(|_, w| w.count().bits(refresh_rate))
    }

    pub fn sliced<'a, T: Sized>(&self, memory_length: usize) -> &'a mut [T] {
        // todo: this is true for f427/f429, dunno how it translates to other devices with FMC
        // todo: confirm that using SDRAM bank 1 means accessing A000_0000-CFFF_FFFF (FMC Block 5)
        // todo: Can't help but feel like it should be 0x80000000 (Block 4)...
        let base_address: u32 = match self.bank_selection {
            SDRAMInternalBankSelection::Bank1 => 0xA000_0000,
            SDRAMInternalBankSelection::Bank2 => 0xD000_0000,
        };

        unsafe {
            core::slice::from_raw_parts_mut(
                base_address as *mut T,
                memory_length / core::mem::size_of::<T>(),
            )
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SDRAMCommandTargetBank {
    None,
    Bank1,
    Bank2,
    Both,
}

pub struct SDRAMCommand {
    pub command_mode: MODE_AW,
    pub target_bank: SDRAMCommandTargetBank, // todo: might be redundant, as we have SDRAM::BankSelection to determine this...
    pub auto_refresh_number: u8,
    pub mode_register_definition: u16,
}
