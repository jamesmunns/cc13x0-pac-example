#[doc = "Reader of register UDMACH16SSEL"]
pub type R = crate::R<u32, super::UDMACH16SSEL>;
#[doc = "Reader of field `EV`"]
pub type EV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - EV"]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0x7f) as u8)
    }
}
