#[doc = "Register `HFCLKSTAT` reader"]
pub struct R(crate::R<HFCLKSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFCLKSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFCLKSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFCLKSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRC` reader - Active clock source"]
pub type SRC_R = crate::BitReader<SRC_A>;
#[doc = "Active clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_A {
    #[doc = "0: HFINT - 64 MHz on-chip oscillator"]
    HFINT = 0,
    #[doc = "1: HFXO - 64 MHz clock derived from external 32 MHz crystal oscillator"]
    HFXO = 1,
}
impl From<SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            false => SRC_A::HFINT,
            true => SRC_A::HFXO,
        }
    }
    #[doc = "Checks if the value of the field is `HFINT`"]
    #[inline(always)]
    pub fn is_hfint(&self) -> bool {
        *self == SRC_A::HFINT
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == SRC_A::HFXO
    }
}
#[doc = "Field `STATE` reader - HFCLK state"]
pub type STATE_R = crate::BitReader<STATE_A>;
#[doc = "HFCLK state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: HFXO has not been started or HFCLKSTOP task has been triggered"]
    NOT_RUNNING = 0,
    #[doc = "1: HFXO has been started (HFCLKSTARTED event has been generated)"]
    RUNNING = 1,
}
impl From<STATE_A> for bool {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as u8 != 0
    }
}
impl STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATE_A {
        match self.bits {
            false => STATE_A::NOT_RUNNING,
            true => STATE_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RUNNING`"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == STATE_A::NOT_RUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == STATE_A::RUNNING
    }
}
impl R {
    #[doc = "Bit 0 - Active clock source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - HFCLK state"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "The register shows if HFXO has been requested by triggering HFCLKSTART task and if it has been started (STATE)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclkstat](index.html) module"]
pub struct HFCLKSTAT_SPEC;
impl crate::RegisterSpec for HFCLKSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfclkstat::R](R) reader structure"]
impl crate::Readable for HFCLKSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HFCLKSTAT to value 0"]
impl crate::Resettable for HFCLKSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
