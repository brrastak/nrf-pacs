#[doc = "Register `TXSTATUS` reader"]
pub struct R(crate::R<TXSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXSTATUS` reader - Status of data in register TXDATA"]
pub type TXSTATUS_R = crate::BitReader<TXSTATUS_A>;
#[doc = "Status of data in register TXDATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTATUS_A {
    #[doc = "0: No data pending in register TXDATA"]
    NO_DATA_PENDING = 0,
    #[doc = "1: Data pending in register TXDATA"]
    DATA_PENDING = 1,
}
impl From<TXSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: TXSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl TXSTATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSTATUS_A {
        match self.bits {
            false => TXSTATUS_A::NO_DATA_PENDING,
            true => TXSTATUS_A::DATA_PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA_PENDING`"]
    #[inline(always)]
    pub fn is_no_data_pending(&self) -> bool {
        *self == TXSTATUS_A::NO_DATA_PENDING
    }
    #[doc = "Checks if the value of the field is `DATA_PENDING`"]
    #[inline(always)]
    pub fn is_data_pending(&self) -> bool {
        *self == TXSTATUS_A::DATA_PENDING
    }
}
impl R {
    #[doc = "Bit 0 - Status of data in register TXDATA"]
    #[inline(always)]
    pub fn txstatus(&self) -> TXSTATUS_R {
        TXSTATUS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "This register shows a status that indicates if the data sent from the CPU to the debugger has been read.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txstatus](index.html) module"]
pub struct TXSTATUS_SPEC;
impl crate::RegisterSpec for TXSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txstatus::R](R) reader structure"]
impl crate::Readable for TXSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXSTATUS to value 0"]
impl crate::Resettable for TXSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
