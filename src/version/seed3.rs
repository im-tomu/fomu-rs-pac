#[doc = "Reader of register SEED3"]
pub type R = crate::R<u32, super::SEED3>;
#[doc = "Reader of field `seed`"]
pub type SEED_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn seed(&self) -> SEED_R {
        SEED_R::new((self.bits & 0xff) as u8)
    }
}
