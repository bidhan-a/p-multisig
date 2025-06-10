use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct MultisigHeader {
    pub seed: [u8; 8],
    pub num_owners: [u8; 8],
    pub threshold: [u8; 8],
    pub nonce: u8,
    pub bump: u8,
}

pub struct Multisig<'a> {
    pub header: MultisigHeader,
    pub owners: &'a [Pubkey],
}

impl<'a> Multisig<'a> {
    pub fn read(account: &AccountInfo) -> Result<(&MultisigHeader, &[Pubkey]), ProgramError> {
        let data = unsafe { account.borrow_data_unchecked() };
        Multisig::parse(data)
    }

    pub fn write(
        account: &AccountInfo,
        header: &MultisigHeader,
        owners: &[Pubkey],
    ) -> Result<(), ProgramError> {
        let data = unsafe { account.borrow_mut_data_unchecked() };
        let header_bytes = bytemuck::bytes_of(header);
        let owners_bytes = bytemuck::cast_slice::<Pubkey, u8>(owners);

        data[..header_bytes.len()].copy_from_slice(header_bytes);
        data[header_bytes.len()..header_bytes.len() + owners_bytes.len()]
            .copy_from_slice(owners_bytes);

        Ok(())
    }

    pub fn parse(data: &[u8]) -> Result<(&MultisigHeader, &[Pubkey]), ProgramError> {
        let header_size = core::mem::size_of::<MultisigHeader>();
        let header = bytemuck::try_from_bytes::<MultisigHeader>(&data[..header_size])
            .map_err(|_| ProgramError::InvalidAccountData)?;
        let num_owners = u64::from_le_bytes(header.num_owners);
        let owners_data = &data[header_size..];
        let owners = bytemuck::cast_slice::<u8, Pubkey>(owners_data);
        Ok((header, &owners[..num_owners as usize]))
    }

    pub fn size(num_owners: u64) -> usize {
        let header_size = core::mem::size_of::<MultisigHeader>();
        let pubkeys_size = num_owners as usize * core::mem::size_of::<Pubkey>();

        header_size + pubkeys_size
    }
}
