#[doc = "Register `PRESCALER` reader"]
pub struct R(crate::R<PRESCALER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESCALER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESCALER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESCALER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESCALER` writer"]
pub struct W(crate::W<PRESCALER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESCALER_SPEC>;
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
impl From<crate::W<PRESCALER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESCALER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALER` reader - RTC PRESCALER value."]
pub struct PRESCALER_R(crate::FieldReader<u16, u16>);
impl PRESCALER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PRESCALER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESCALER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESCALER` writer - RTC PRESCALER value."]
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - RTC PRESCALER value."]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RTC PRESCALER value."]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "12-bit prescaler for COUNTER frequency (32768/(PRESCALER+1)). Must be written when RTC is STOPed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prescaler](index.html) module"]
pub struct PRESCALER_SPEC;
impl crate::RegisterSpec for PRESCALER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prescaler::R](R) reader structure"]
impl crate::Readable for PRESCALER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prescaler::W](W) writer structure"]
impl crate::Writable for PRESCALER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESCALER to value 0"]
impl crate::Resettable for PRESCALER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
