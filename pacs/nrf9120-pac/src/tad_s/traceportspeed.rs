#[doc = "Register `TRACEPORTSPEED` reader"]
pub struct R(crate::R<TRACEPORTSPEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACEPORTSPEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRACEPORTSPEED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRACEPORTSPEED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACEPORTSPEED` writer"]
pub struct W(crate::W<TRACEPORTSPEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACEPORTSPEED_SPEC>;
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
impl From<crate::W<TRACEPORTSPEED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRACEPORTSPEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRACEPORTSPEED` reader - Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
pub type TRACEPORTSPEED_R = crate::FieldReader<u8, TRACEPORTSPEED_A>;
#[doc = "Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRACEPORTSPEED_A {
    #[doc = "0: Trace Port clock is: 32MHz"]
    _32MHZ = 0,
    #[doc = "1: Trace Port clock is: 16MHz"]
    _16MHZ = 1,
    #[doc = "2: Trace Port clock is: 8MHz"]
    _8MHZ = 2,
    #[doc = "3: Trace Port clock is: 4MHz"]
    _4MHZ = 3,
}
impl From<TRACEPORTSPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: TRACEPORTSPEED_A) -> Self {
        variant as _
    }
}
impl TRACEPORTSPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACEPORTSPEED_A {
        match self.bits {
            0 => TRACEPORTSPEED_A::_32MHZ,
            1 => TRACEPORTSPEED_A::_16MHZ,
            2 => TRACEPORTSPEED_A::_8MHZ,
            3 => TRACEPORTSPEED_A::_4MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32MHZ`"]
    #[inline(always)]
    pub fn is_32mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_32MHZ
    }
    #[doc = "Checks if the value of the field is `_16MHZ`"]
    #[inline(always)]
    pub fn is_16mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_16MHZ
    }
    #[doc = "Checks if the value of the field is `_8MHZ`"]
    #[inline(always)]
    pub fn is_8mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_8MHZ
    }
    #[doc = "Checks if the value of the field is `_4MHZ`"]
    #[inline(always)]
    pub fn is_4mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_4MHZ
    }
}
#[doc = "Field `TRACEPORTSPEED` writer - Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
pub type TRACEPORTSPEED_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRACEPORTSPEED_SPEC, u8, TRACEPORTSPEED_A, 2, O>;
impl<'a, const O: u8> TRACEPORTSPEED_W<'a, O> {
    #[doc = "Trace Port clock is: 32MHz"]
    #[inline(always)]
    pub fn _32mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_32MHZ)
    }
    #[doc = "Trace Port clock is: 16MHz"]
    #[inline(always)]
    pub fn _16mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_16MHZ)
    }
    #[doc = "Trace Port clock is: 8MHz"]
    #[inline(always)]
    pub fn _8mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_8MHZ)
    }
    #[doc = "Trace Port clock is: 4MHz"]
    #[inline(always)]
    pub fn _4mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_4MHZ)
    }
}
impl R {
    #[doc = "Bits 0:1 - Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
    #[inline(always)]
    pub fn traceportspeed(&self) -> TRACEPORTSPEED_R {
        TRACEPORTSPEED_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
    #[inline(always)]
    pub fn traceportspeed(&mut self) -> TRACEPORTSPEED_W<0> {
        TRACEPORTSPEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clocking options for the Trace Port debug interface Reset behavior is the same as debug components\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [traceportspeed](index.html) module"]
pub struct TRACEPORTSPEED_SPEC;
impl crate::RegisterSpec for TRACEPORTSPEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [traceportspeed::R](R) reader structure"]
impl crate::Readable for TRACEPORTSPEED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [traceportspeed::W](W) writer structure"]
impl crate::Writable for TRACEPORTSPEED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRACEPORTSPEED to value 0"]
impl crate::Resettable for TRACEPORTSPEED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
