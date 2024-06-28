#[doc = "Register `TRACEDATA0` reader"]
pub struct R(crate::R<TRACEDATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACEDATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRACEDATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRACEDATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACEDATA0` writer"]
pub struct W(crate::W<TRACEDATA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACEDATA0_SPEC>;
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
impl From<crate::W<TRACEDATA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRACEDATA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN` reader - Pin number"]
pub type PIN_R = crate::FieldReader<u8, PIN_A>;
#[doc = "Pin number\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PIN_A {
    #[doc = "22: TRACEDATA0 pin"]
    TRACEDATA0 = 22,
}
impl From<PIN_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN_A) -> Self {
        variant as _
    }
}
impl PIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PIN_A> {
        match self.bits {
            22 => Some(PIN_A::TRACEDATA0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRACEDATA0`"]
    #[inline(always)]
    pub fn is_tracedata0(&self) -> bool {
        *self == PIN_A::TRACEDATA0
    }
}
#[doc = "Field `PIN` writer - Pin number"]
pub type PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRACEDATA0_SPEC, u8, PIN_A, 5, O>;
impl<'a, const O: u8> PIN_W<'a, O> {
    #[doc = "TRACEDATA0 pin"]
    #[inline(always)]
    pub fn tracedata0(self) -> &'a mut W {
        self.variant(PIN_A::TRACEDATA0)
    }
}
#[doc = "Field `CONNECT` reader - Connection"]
pub type CONNECT_R = crate::BitReader<CONNECT_A>;
#[doc = "Connection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONNECT_A {
    #[doc = "1: Disconnect"]
    DISCONNECTED = 1,
    #[doc = "0: Connect"]
    CONNECTED = 0,
}
impl From<CONNECT_A> for bool {
    #[inline(always)]
    fn from(variant: CONNECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CONNECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONNECT_A {
        match self.bits {
            true => CONNECT_A::DISCONNECTED,
            false => CONNECT_A::CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == CONNECT_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `CONNECTED`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == CONNECT_A::CONNECTED
    }
}
#[doc = "Field `CONNECT` writer - Connection"]
pub type CONNECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRACEDATA0_SPEC, CONNECT_A, O>;
impl<'a, const O: u8> CONNECT_W<'a, O> {
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(CONNECT_A::DISCONNECTED)
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(CONNECT_A::CONNECTED)
    }
}
impl R {
    #[doc = "Bits 0:4 - Pin number"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    pub fn connect(&self) -> CONNECT_R {
        CONNECT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pin number"]
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W<0> {
        PIN_W::new(self)
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    pub fn connect(&mut self) -> CONNECT_W<31> {
        CONNECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin configuration for TRACEDATA\\[0\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tracedata0](index.html) module"]
pub struct TRACEDATA0_SPEC;
impl crate::RegisterSpec for TRACEDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tracedata0::R](R) reader structure"]
impl crate::Readable for TRACEDATA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tracedata0::W](W) writer structure"]
impl crate::Writable for TRACEDATA0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRACEDATA0 to value 0xffff_ffff"]
impl crate::Resettable for TRACEDATA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
