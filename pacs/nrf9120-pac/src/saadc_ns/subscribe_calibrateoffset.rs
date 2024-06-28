#[doc = "Register `SUBSCRIBE_CALIBRATEOFFSET` reader"]
pub type R = crate::R<SubscribeCalibrateoffsetSpec>;
#[doc = "Register `SUBSCRIBE_CALIBRATEOFFSET` writer"]
pub type W = crate::W<SubscribeCalibrateoffsetSpec>;
#[doc = "Field `CHIDX` reader - DPPI channel that task CALIBRATEOFFSET will subscribe to"]
pub type ChidxR = crate::FieldReader;
#[doc = "Field `CHIDX` writer - DPPI channel that task CALIBRATEOFFSET will subscribe to"]
pub type ChidxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disable subscription"]
    Disabled = 0,
    #[doc = "1: Enable subscription"]
    Enabled = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - "]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Disabled,
            true => En::Enabled,
        }
    }
    #[doc = "Disable subscription"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == En::Disabled
    }
    #[doc = "Enable subscription"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == En::Enabled
    }
}
#[doc = "Field `EN` writer - "]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable subscription"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disabled)
    }
    #[doc = "Enable subscription"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:7 - DPPI channel that task CALIBRATEOFFSET will subscribe to"]
    #[inline(always)]
    pub fn chidx(&self) -> ChidxR {
        ChidxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DPPI channel that task CALIBRATEOFFSET will subscribe to"]
    #[inline(always)]
    #[must_use]
    pub fn chidx(&mut self) -> ChidxW<SubscribeCalibrateoffsetSpec> {
        ChidxW::new(self, 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<SubscribeCalibrateoffsetSpec> {
        EnW::new(self, 31)
    }
}
#[doc = "Subscribe configuration for task CALIBRATEOFFSET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_calibrateoffset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_calibrateoffset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubscribeCalibrateoffsetSpec;
impl crate::RegisterSpec for SubscribeCalibrateoffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`subscribe_calibrateoffset::R`](R) reader structure"]
impl crate::Readable for SubscribeCalibrateoffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`subscribe_calibrateoffset::W`](W) writer structure"]
impl crate::Writable for SubscribeCalibrateoffsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUBSCRIBE_CALIBRATEOFFSET to value 0"]
impl crate::Resettable for SubscribeCalibrateoffsetSpec {
    const RESET_VALUE: u32 = 0;
}
