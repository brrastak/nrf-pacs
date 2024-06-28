#[doc = "Register `HFXOSRC` reader"]
pub struct R(crate::R<HFXOSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXOSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXOSRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXOSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFXOSRC` writer"]
pub struct W(crate::W<HFXOSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFXOSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HFXOSRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFXOSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFXOSRC` reader - HFXO clock source selection"]
pub type HFXOSRC_R = crate::BitReader<HFXOSRC_A>;
#[doc = "HFXO clock source selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXOSRC_A {
    #[doc = "1: 32 MHz crystal oscillator"]
    XTAL = 1,
    #[doc = "0: 32 MHz temperature compensated crystal oscillator (TCXO)"]
    TCXO = 0,
}
impl From<HFXOSRC_A> for bool {
    #[inline(always)]
    fn from(variant: HFXOSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl HFXOSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXOSRC_A {
        match self.bits {
            true => HFXOSRC_A::XTAL,
            false => HFXOSRC_A::TCXO,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == HFXOSRC_A::XTAL
    }
    #[doc = "Checks if the value of the field is `TCXO`"]
    #[inline(always)]
    pub fn is_tcxo(&self) -> bool {
        *self == HFXOSRC_A::TCXO
    }
}
#[doc = "Field `HFXOSRC` writer - HFXO clock source selection"]
pub type HFXOSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFXOSRC_SPEC, HFXOSRC_A, O>;
impl<'a, const O: u8> HFXOSRC_W<'a, O> {
    #[doc = "32 MHz crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(HFXOSRC_A::XTAL)
    }
    #[doc = "32 MHz temperature compensated crystal oscillator (TCXO)"]
    #[inline(always)]
    pub fn tcxo(self) -> &'a mut W {
        self.variant(HFXOSRC_A::TCXO)
    }
}
impl R {
    #[doc = "Bit 0 - HFXO clock source selection"]
    #[inline(always)]
    pub fn hfxosrc(&self) -> HFXOSRC_R {
        HFXOSRC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFXO clock source selection"]
    #[inline(always)]
    pub fn hfxosrc(&mut self) -> HFXOSRC_W<0> {
        HFXOSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFXO clock source selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxosrc](index.html) module"]
pub struct HFXOSRC_SPEC;
impl crate::RegisterSpec for HFXOSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxosrc::R](R) reader structure"]
impl crate::Readable for HFXOSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfxosrc::W](W) writer structure"]
impl crate::Writable for HFXOSRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFXOSRC to value 0xffff_ffff"]
impl crate::Resettable for HFXOSRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
