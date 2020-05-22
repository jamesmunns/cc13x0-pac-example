#[doc = "Reader of register FPAC2"]
pub type R = crate::R<u32, super::FPAC2>;
#[doc = "Writer for register FPAC2"]
pub type W = crate::W<u32, super::FPAC2>;
#[doc = "Register FPAC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FPAC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PAGP`"]
pub type PAGP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PAGP`"]
pub struct PAGP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PAGP"]
    #[inline(always)]
    pub fn pagp(&self) -> PAGP_R {
        PAGP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PAGP"]
    #[inline(always)]
    pub fn pagp(&mut self) -> PAGP_W {
        PAGP_W { w: self }
    }
}
