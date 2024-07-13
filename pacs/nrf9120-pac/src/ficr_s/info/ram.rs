#[doc = "Register `RAM` reader"]
pub struct R(crate::R<RAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RAM` reader - RAM variant"]
pub type RAM_R = crate::FieldReader<u32, RAM_A>;
#[doc = "RAM variant\n\nValue on reset: 256"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum RAM_A {
    #[doc = "256: 256 kByte RAM"]
    K256 = 256,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<RAM_A> for u32 {
    #[inline(always)]
    fn from(variant: RAM_A) -> Self {
        variant as _
    }
}
impl RAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAM_A> {
        match self.bits {
            256 => Some(RAM_A::K256),
            4294967295 => Some(RAM_A::UNSPECIFIED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `K256`"]
    #[inline(always)]
    pub fn is_k256(&self) -> bool {
        *self == RAM_A::K256
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == RAM_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - RAM variant"]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new(self.bits)
    }
}
#[doc = "RAM variant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram](index.html) module"]
pub struct RAM_SPEC;
impl crate::RegisterSpec for RAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram::R](R) reader structure"]
impl crate::Readable for RAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAM to value 0x0100"]
impl crate::Resettable for RAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
