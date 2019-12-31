//! HAL for Flexible memory controller (FMC)

use crate::stm32;
use crate::stm32::FMC;

use crate::stm32::rcc::d1ccipr;
use stm32h7xx_hal::rcc::CoreClocks;
use stm32h7xx_hal::time::Hertz;

use stm32h7xx_hal::gpio::gpioa::PA7;
use stm32h7xx_hal::gpio::gpiob::{PB5, PB6, PB7};
use stm32h7xx_hal::gpio::gpioc::{PC0, PC2, PC3, PC4, PC5, PC6, PC7, PC8};
use stm32h7xx_hal::gpio::gpiod::{
    PD0, PD1, PD10, PD11, PD12, PD13, PD14, PD15, PD3, PD4, PD5, PD6, PD7, PD8,
    PD9,
};
use stm32h7xx_hal::gpio::gpioe::{
    PE0, PE1, PE10, PE11, PE12, PE13, PE14, PE15, PE2, PE3, PE4, PE5, PE6, PE7,
    PE8, PE9,
};
use stm32h7xx_hal::gpio::gpiof::{
    PF0, PF1, PF11, PF12, PF13, PF14, PF15, PF2, PF3, PF4, PF5,
};
use stm32h7xx_hal::gpio::gpiog::{
    PG0, PG1, PG10, PG12, PG13, PG14, PG15, PG2, PG3, PG4, PG5, PG6, PG7, PG8,
    PG9,
};
use stm32h7xx_hal::gpio::gpioh::{
    PH10, PH11, PH12, PH13, PH14, PH15, PH2, PH3, PH5, PH6, PH7, PH8, PH9,
};
use stm32h7xx_hal::gpio::gpioi::{
    PI0, PI1, PI10, PI2, PI3, PI4, PI5, PI6, PI7, PI9,
};
use stm32h7xx_hal::gpio::{Alternate, AF12, AF9};

/// FMC Bank Base Addresses. See RM0433 Figure 95.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(unused)]
pub enum FmcBank {
    Bank1,
    Bank2,
    Bank3,
    Bank4,
    Bank5,
    Bank6,
}
impl FmcBank {
    /// Return a pointer to this FMC bank
    pub fn ptr(self) -> *mut u32 {
        use FmcBank::*;
        (match self {
            Bank1 => 0x6000_0000u32,
            Bank2 => 0x7000_0000u32,
            Bank3 => 0x8000_0000u32,
            Bank4 => 0x9000_0000u32, // Not used
            Bank5 => 0xC000_0000u32,
            Bank6 => 0xD000_0000u32,
        }) as *mut u32
    }
}

/// FMC controller
#[allow(missing_debug_implementations)]
pub struct Fmc {
    /// Flexible memory controller (FMC)
    pub(crate) fmc: FMC,
}

/// Set of pins for an SDRAM
pub trait PinsSdram<FMC> {
    const EXTERNAL_BANK: u8;
    const NUMBER_INTERNAL_BANKS: u8;
    const ADDRESS_LINES: u8;
}

/// Set of pins for SDRAM on Bank 1 of FMC controller
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PinsSdramBank1<T>(pub T);
/// Set of pins for SDRAM on Bank 2 of FMC controller
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PinsSdramBank2<T>(pub T);

macro_rules! impl_16bit_sdram {
    ($($pins:tt: [$eBankN:expr, $ckeN:tt, $neN:tt,
        $nInternalB:expr $(, $pba1:ident, $ba1:tt)*]),+) => {
        $(
            #[rustfmt::skip]
            /// 16-bit SDRAM
            impl<FMC, PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10,
            PA11, PBA0, $($pba1,)* PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7, PD8,
            PD9, PD10, PD11, PD12, PD13, PD14, PD15, PNBL0, PNBL1, PSDCKEn,
            PSDCLK, PSDNCAS, PSDNEn, PSDNRAS, PSDNWE>
                PinsSdram<FMC>
                for $pins<(PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10,
                     PA11, PBA0, $($pba1,)* PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7,
                     PD8, PD9, PD10, PD11, PD12, PD13, PD14, PD15, PNBL0, PNBL1,
                     PSDCKEn, PSDCLK, PSDNCAS, PSDNEn, PSDNRAS, PSDNWE)>
            where PA0: A0<FMC>, PA1: A1<FMC>, PA2: A2<FMC>, PA3: A3<FMC>, PA4:
            A4<FMC>, PA5: A5<FMC>, PA6: A6<FMC>, PA7: A7<FMC>, PA8: A8<FMC>, PA9:
            A9<FMC>, PA10: A10<FMC>, PA11: A11<FMC>, PBA0: BA0<FMC>,
            $($pba1:$ba1<FMC>,)*
            PD0: D0<FMC>, PD1: D1<FMC>, PD2: D2<FMC>, PD3: D3<FMC>, PD4:
            D4<FMC>, PD5: D5<FMC>, PD6: D6<FMC>, PD7: D7<FMC>, PD8: D8<FMC>, PD9:
            D9<FMC>, PD10: D10<FMC>, PD11: D11<FMC>, PD12: D12<FMC>, PD13:
            D13<FMC>, PD14: D14<FMC>, PD15: D15<FMC>, PNBL0: NBL0<FMC>, PNBL1:
            NBL1<FMC>, PSDCKEn: $ckeN<FMC>, PSDCLK: SDCLK<FMC>, PSDNCAS:
            SDNCAS<FMC>, PSDNEn: $neN<FMC>, PSDNRAS: SDNRAS<FMC>, PSDNWE: SDNWE<FMC> {
                const ADDRESS_LINES: u8 = 12;
                const EXTERNAL_BANK: u8 = $eBankN;
                const NUMBER_INTERNAL_BANKS: u8 = $nInternalB;
            }
        )+
    }
}

macro_rules! impl_32bit_sdram {
    ($($pins:tt: [$eBankN:expr, $ckeN:tt, $neN:tt,
        $nInternalB:expr $(, $pba1:ident, $ba1:tt)*]),+) => {
        $(
            #[rustfmt::skip]
            /// 32-bit SDRAM
            impl<FMC, PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10,
            PA11, PBA0, $($pba1,)* PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7, PD8,
            PD9, PD10, PD11, PD12, PD13, PD14, PD15, PD16, PD17, PD18, PD19,
            PD20, PD21, PD22, PD23, PD24, PD25, PD26, PD27, PD28, PD29, PD30,
            PD31, PNBL0, PNBL1, PNBL2, PNBL3, PSDCKEn, PSDCLK, PSDNCAS,
            PSDNEn, PSDNRAS, PSDNWE>
                PinsSdram<FMC>
                for $pins<(PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10,
                     PA11, PBA0, $($pba1,)* PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7,
                     PD8, PD9, PD10, PD11, PD12, PD13, PD14, PD15, PD16, PD17,
                     PD18, PD19, PD20, PD21, PD22, PD23, PD24, PD25, PD26, PD27,
                     PD28, PD29, PD30, PD31, PNBL0, PNBL1, PNBL2, PNBL3, PSDCKEn,
                     PSDCLK, PSDNCAS, PSDNEn, PSDNRAS, PSDNWE)>
            where PA0: A0<FMC>, PA1: A1<FMC>, PA2: A2<FMC>, PA3: A3<FMC>, PA4:
            A4<FMC>, PA5: A5<FMC>, PA6: A6<FMC>, PA7: A7<FMC>, PA8: A8<FMC>, PA9:
            A9<FMC>, PA10: A10<FMC>, PA11: A11<FMC>, PBA0: BA0<FMC>,
            $($pba1:$ba1<FMC>,)*
            PD0: D0<FMC>, PD1: D1<FMC>, PD2: D2<FMC>, PD3: D3<FMC>, PD4:
            D4<FMC>, PD5: D5<FMC>, PD6: D6<FMC>, PD7: D7<FMC>, PD8: D8<FMC>, PD9:
            D9<FMC>, PD10: D10<FMC>, PD11: D11<FMC>, PD12: D12<FMC>, PD13:
            D13<FMC>, PD14: D14<FMC>, PD15: D15<FMC>, PD16: D16<FMC>, PD17:
            D17<FMC>, PD18: D18<FMC>, PD19: D19<FMC>, PD20: D20<FMC>, PD21:
            D21<FMC>, PD22: D22<FMC>, PD23: D23<FMC>, PD24: D24<FMC>, PD25:
            D25<FMC>, PD26: D26<FMC>, PD27: D27<FMC>, PD28: D28<FMC>, PD29:
            D29<FMC>, PD30: D30<FMC>, PD31: D31<FMC>, PNBL0: NBL0<FMC>, PNBL1:
            NBL1<FMC>, PNBL2: NBL2<FMC>, PNBL3: NBL3<FMC>, PSDCKEn: $ckeN<FMC>,
                  PSDCLK: SDCLK<FMC>, PSDNCAS: SDNCAS<FMC>, PSDNEn: $neN<FMC>, PSDNRAS:
            SDNRAS<FMC>, PSDNWE: SDNWE<FMC> {
                const ADDRESS_LINES: u8 = 12;
                const EXTERNAL_BANK: u8 = $eBankN;
                const NUMBER_INTERNAL_BANKS: u8 = $nInternalB;
            }
        )+
    }
}

impl_16bit_sdram! {
    // 16-bit SDRAM with 12 address lines, BA0 only
    PinsSdramBank1: [1, SDCKE0, SDNE0, 2],
    PinsSdramBank2: [2, SDCKE1, SDNE1, 2],
    // 16-bit SDRAM with 12 address lines, BA0 and BA1
    PinsSdramBank1: [1, SDCKE0, SDNE0, 4, PBA1, BA1],
    PinsSdramBank2: [2, SDCKE1, SDNE1, 4, PBA1, BA1]
}

impl_32bit_sdram! {
    // 32-bit SDRAM with 12 address lines, BA0 only
    PinsSdramBank1: [1, SDCKE0, SDNE0, 2],
    PinsSdramBank2: [2, SDCKE1, SDNE1, 2],
    // 32-bit SDRAM with 12 address lines, BA0 and BA1
    PinsSdramBank1: [1, SDCKE0, SDNE0, 4, PBA1, BA1],
    PinsSdramBank2: [2, SDCKE1, SDNE1, 4, PBA1, BA1]
}
macro_rules! pins {
    (FMC: $($pin:ident: [$($inst:ty),*])+) => {
        $(
            $(
                impl $pin<FMC> for $inst {}
            )*
        )+
    }
}

pub trait A0<FMC> {}
pub trait A1<FMC> {}
pub trait A10<FMC> {}
pub trait A11<FMC> {}
pub trait A12<FMC> {}
pub trait A13<FMC> {}
pub trait A14<FMC> {}
pub trait A15<FMC> {}
pub trait A16<FMC> {}
pub trait A17<FMC> {}
pub trait A18<FMC> {}
pub trait A19<FMC> {}
pub trait A2<FMC> {}
pub trait A20<FMC> {}
pub trait A21<FMC> {}
pub trait A22<FMC> {}
pub trait A23<FMC> {}
pub trait A24<FMC> {}
pub trait A25<FMC> {}
pub trait A3<FMC> {}
pub trait A4<FMC> {}
pub trait A5<FMC> {}
pub trait A6<FMC> {}
pub trait A7<FMC> {}
pub trait A8<FMC> {}
pub trait A9<FMC> {}
pub trait BA0<FMC> {}
pub trait BA1<FMC> {}
pub trait CLK<FMC> {}
pub trait D0<FMC> {}
pub trait D1<FMC> {}
pub trait D10<FMC> {}
pub trait D11<FMC> {}
pub trait D12<FMC> {}
pub trait D13<FMC> {}
pub trait D14<FMC> {}
pub trait D15<FMC> {}
pub trait D16<FMC> {}
pub trait D17<FMC> {}
pub trait D18<FMC> {}
pub trait D19<FMC> {}
pub trait D2<FMC> {}
pub trait D20<FMC> {}
pub trait D21<FMC> {}
pub trait D22<FMC> {}
pub trait D23<FMC> {}
pub trait D24<FMC> {}
pub trait D25<FMC> {}
pub trait D26<FMC> {}
pub trait D27<FMC> {}
pub trait D28<FMC> {}
pub trait D29<FMC> {}
pub trait D3<FMC> {}
pub trait D30<FMC> {}
pub trait D31<FMC> {}
pub trait D4<FMC> {}
pub trait D5<FMC> {}
pub trait D6<FMC> {}
pub trait D7<FMC> {}
pub trait D8<FMC> {}
pub trait D9<FMC> {}
pub trait DA0<FMC> {}
pub trait DA1<FMC> {}
pub trait DA10<FMC> {}
pub trait DA11<FMC> {}
pub trait DA12<FMC> {}
pub trait DA13<FMC> {}
pub trait DA14<FMC> {}
pub trait DA15<FMC> {}
pub trait DA2<FMC> {}
pub trait DA3<FMC> {}
pub trait DA4<FMC> {}
pub trait DA5<FMC> {}
pub trait DA6<FMC> {}
pub trait DA7<FMC> {}
pub trait DA8<FMC> {}
pub trait DA9<FMC> {}
pub trait INT<FMC> {}
pub trait NBL0<FMC> {}
pub trait NBL1<FMC> {}
pub trait NBL2<FMC> {}
pub trait NBL3<FMC> {}
pub trait NCE<FMC> {}
pub trait NE1<FMC> {}
pub trait NE2<FMC> {}
pub trait NE3<FMC> {}
pub trait NE4<FMC> {}
pub trait NL<FMC> {}
pub trait NOE<FMC> {}
pub trait NWAIT<FMC> {}
pub trait NWE<FMC> {}
pub trait SDCKE0<FMC> {}
pub trait SDCKE1<FMC> {}
pub trait SDCLK<FMC> {}
pub trait SDNCAS<FMC> {}
pub trait SDNE0<FMC> {}
pub trait SDNE1<FMC> {}
pub trait SDNRAS<FMC> {}
pub trait SDNWE<FMC> {}

pins! {
    FMC:
        A0: [ PF0<Alternate<AF12>> ]
        A1: [ PF1<Alternate<AF12>> ]
        A2: [ PF2<Alternate<AF12>> ]
        A3: [ PF3<Alternate<AF12>> ]
        A4: [ PF4<Alternate<AF12>> ]
        A5: [ PF5<Alternate<AF12>> ]
        A6: [ PF12<Alternate<AF12>> ]
        A7: [ PF13<Alternate<AF12>> ]
        A8: [ PF14<Alternate<AF12>> ]
        A9: [ PF15<Alternate<AF12>> ]
        A10: [ PG0<Alternate<AF12>> ]
        A11: [ PG1<Alternate<AF12>> ]
        A12: [ PG2<Alternate<AF12>> ]
        A13: [ PG3<Alternate<AF12>> ]
        A14: [ PG4<Alternate<AF12>> ]
        A15: [ PG5<Alternate<AF12>> ]
        A16: [ PD11<Alternate<AF12>> ]
        A17: [ PD12<Alternate<AF12>> ]
        A18: [ PD13<Alternate<AF12>> ]
        A19: [ PE3<Alternate<AF12>> ]
        A20: [ PE4<Alternate<AF12>> ]
        A21: [ PE5<Alternate<AF12>> ]
        A22: [ PE6<Alternate<AF12>> ]
        A23: [ PE2<Alternate<AF12>> ]
        A24: [ PG13<Alternate<AF12>> ]
        A25: [ PG14<Alternate<AF12>> ]

        BA0: [ PG4<Alternate<AF12>> ]
        BA1: [ PG5<Alternate<AF12>> ]

        CLK: [ PD3<Alternate<AF12>> ]

        D0: [ PD14<Alternate<AF12>> ]
        D1: [ PD15<Alternate<AF12>> ]
        D2: [ PD0<Alternate<AF12>> ]
        D3: [ PD1<Alternate<AF12>> ]
        D4: [ PE7<Alternate<AF12>> ]
        D5: [ PE8<Alternate<AF12>> ]
        D6: [ PE9<Alternate<AF12>> ]
        D7: [ PE10<Alternate<AF12>> ]
        D8: [ PE11<Alternate<AF12>> ]
        D9: [ PE12<Alternate<AF12>> ]
        D10: [ PE13<Alternate<AF12>> ]
        D11: [ PE14<Alternate<AF12>> ]
        D12: [ PE15<Alternate<AF12>> ]
        D13: [ PD8<Alternate<AF12>> ]
        D14: [ PD9<Alternate<AF12>> ]
        D15: [ PD10<Alternate<AF12>> ]
        D16: [ PH8<Alternate<AF12>> ]
        D17: [ PH9<Alternate<AF12>> ]
        D18: [ PH10<Alternate<AF12>> ]
        D19: [ PH11<Alternate<AF12>> ]
        D20: [ PH12<Alternate<AF12>> ]
        D21: [ PH13<Alternate<AF12>> ]
        D22: [ PH14<Alternate<AF12>> ]
        D23: [ PH15<Alternate<AF12>> ]
        D24: [ PI0<Alternate<AF12>> ]
        D25: [ PI1<Alternate<AF12>> ]
        D26: [ PI2<Alternate<AF12>> ]
        D27: [ PI3<Alternate<AF12>> ]
        D28: [ PI6<Alternate<AF12>> ]
        D29: [ PI7<Alternate<AF12>> ]
        D30: [ PI9<Alternate<AF12>> ]
        D31: [ PI10<Alternate<AF12>> ]

        DA0: [ PD14<Alternate<AF12>> ]
        DA1: [ PD15<Alternate<AF12>> ]
        DA2: [ PD0<Alternate<AF12>> ]
        DA3: [ PD1<Alternate<AF12>> ]
        DA4: [ PE7<Alternate<AF12>> ]
        DA5: [ PE8<Alternate<AF12>> ]
        DA6: [ PE9<Alternate<AF12>> ]
        DA7: [ PE10<Alternate<AF12>> ]
        DA8: [ PE11<Alternate<AF12>> ]
        DA9: [ PE12<Alternate<AF12>> ]
        DA10: [ PE13<Alternate<AF12>> ]
        DA11: [ PE14<Alternate<AF12>> ]
        DA12: [ PE15<Alternate<AF12>> ]
        DA13: [ PD8<Alternate<AF12>> ]
        DA14: [ PD9<Alternate<AF12>> ]
        DA15: [ PD10<Alternate<AF12>> ]

        INT: [ PG7<Alternate<AF12>> ]

        NBL0: [ PE0<Alternate<AF12>> ]
        NBL1: [ PE1<Alternate<AF12>> ]
        NBL2: [ PI4<Alternate<AF12>> ]
        NBL3: [ PI5<Alternate<AF12>> ]

        // NAND
        NCE: [
            PC8<Alternate<AF9>>,
            PG9<Alternate<AF12>>
        ]
        NE1: [
            PC7<Alternate<AF9>>,
            PD7<Alternate<AF12>>
        ]
        NE2: [
            PC8<Alternate<AF9>>,
            PG9<Alternate<AF12>>
        ]
        NE3: [
            PG6<Alternate<AF12>>,
            PG10<Alternate<AF12>>
        ]
        NE4: [
            PG12<Alternate<AF12>>
        ]
        NL: [ PB7<Alternate<AF12>> ]
        NOE: [ PD4<Alternate<AF12>> ]
        NWAIT: [
            PC6<Alternate<AF9>>,
            PD6<Alternate<AF12>>
        ]
        NWE: [ PD5<Alternate<AF12>> ]

        // SDRAM
        SDCKE0: [
            PC3<Alternate<AF12>>,
            PC5<Alternate<AF12>>,
            PH2<Alternate<AF12>>
        ]
        SDCKE1: [
            PB5<Alternate<AF12>>,
            PH7<Alternate<AF12>>
        ]
        SDCLK: [ PG8<Alternate<AF12>> ]
        SDNCAS: [ PG15<Alternate<AF12>> ]
        SDNE0: [
            PC2<Alternate<AF12>>,
            PC4<Alternate<AF12>>,
            PH3<Alternate<AF12>>
        ]
        SDNE1: [
            PB6<Alternate<AF12>>,
            PH6<Alternate<AF12>>
        ]
        SDNRAS: [ PF11<Alternate<AF12>> ]
        SDNWE: [
            PA7<Alternate<AF12>>,
            PC0<Alternate<AF12>>,
            PH5<Alternate<AF12>>
        ]
}

impl Fmc {
    /// New FMC instance
    pub fn new(fmc: FMC) -> Self {
        {
            // Unsafe: only access bits specific to FMC
            let rcc = unsafe { &*stm32::RCC::ptr() };

            // Enable clock and reset
            rcc.ahb3enr.modify(|_, w| w.fmcen().enabled());
            rcc.ahb3rstr.modify(|_, w| w.fmcrst().set_bit());
            rcc.ahb3rstr.modify(|_, w| w.fmcrst().clear_bit());
        }

        Fmc { fmc }
    }

    /// Current kernel clock (`fmc_ker_ck`)
    pub fn get_ker_clk(clocks: CoreClocks) -> Option<Hertz> {
        // Unsafe: read only
        let rcc = unsafe { &*stm32::RCC::ptr() };

        match rcc.d1ccipr.read().fmcsel().variant() {
            d1ccipr::FMCSEL_A::RCC_HCLK3 => Some(clocks.hclk()),
            d1ccipr::FMCSEL_A::PLL1_Q => clocks.pll1_q_ck(),
            d1ccipr::FMCSEL_A::PLL2_R => clocks.pll2_r_ck(),
            d1ccipr::FMCSEL_A::PER => clocks.per_ck(),
        }
    }

    /// Enable FMC controller
    pub(crate) fn enable(&mut self) {
        // The FMCEN bit of the FMC_BCR2..4 registers is don’t
        // care. It is only enabled through the FMC_BCR1 register.
        self.fmc.bcr1.modify(|_, w| w.fmcen().set_bit());
    }
}
