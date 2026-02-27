use bytemuck::{Pod, Zeroable};
use pinocchio::Address;

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct Fundraiser {
    pub maker: Address,
    pub mint_to_raise: Address,
    pub amount_to_raise: [u8; 8],
    pub current_amount: [u8; 8],
    pub time_started: [i8; 8],
    pub duration: [u8; 1],
    pub bump: [u8; 1],
}

impl Fundraiser {
    pub const LEN: usize = core::mem::size_of::<Fundraiser>();

    pub fn to_bytes(&self) -> &[u8; Self::LEN] {
        bytemuck::bytes_of(self).try_into().unwrap()
        // bytemuck::cast_ref(self)
    }

    pub fn min_sendable(&self) -> u64 {
        10_000_000 // 10 usdc
    }

    pub fn max_sendable(&self) -> u64 {
        10_000_000_000 // 10k usdc
    }
}
