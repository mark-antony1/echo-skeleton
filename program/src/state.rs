use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct AuthorizedBufferHeader {
    // TODO
    buffer_seed: u64,
    buffer_size: usize,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct VendingMachineBufferHeader {
    // TODO
    price: u64,
    buffer_size: usize,
}
