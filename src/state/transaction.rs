use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TransactionHeader {
    pub multisig: Pubkey,
    pub program_id: Pubkey,
    pub num_accounts: [u8; 8],
    pub num_signers: [u8; 8],
    pub data_len: [u8; 8],
    pub executed: u8,
    pub seed: [u8; 8],
    pub bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TransactionAccount {
    pub pubkey: Pubkey,
    pub is_signer: u8,
    pub is_writable: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TransactionSigner {
    pub pubkey: Pubkey,
    pub signed: u8,
}

pub struct Transaction<'a> {
    pub header: TransactionHeader,
    pub accounts: &'a [TransactionAccount],
    pub signers: &'a [u8],
    pub data: &'a [u8],
}

impl<'a> Transaction<'a> {
    pub fn read(
        account: &AccountInfo,
    ) -> Result<
        (
            &TransactionHeader,
            &[TransactionAccount],
            &[TransactionSigner],
            &[u8],
        ),
        ProgramError,
    > {
        let data = unsafe { account.borrow_data_unchecked() };
        Transaction::parse(data)
    }

    pub fn write(
        account: &AccountInfo,
        header: &TransactionHeader,
        accounts: &[TransactionAccount],
        signers: &[TransactionSigner],
        tx_data: &[u8],
    ) -> Result<(), ProgramError> {
        let data = unsafe { account.borrow_mut_data_unchecked() };
        let header_bytes = bytemuck::bytes_of(header);
        let accounts_bytes = bytemuck::cast_slice::<TransactionAccount, u8>(accounts);
        let signers_bytes = bytemuck::cast_slice::<TransactionSigner, u8>(signers);

        let accounts_offset = header_bytes.len();
        let signers_offset = accounts_offset + accounts_bytes.len();
        let data_offset = signers_offset + signers_bytes.len();
        let tx_data_len = tx_data.len();

        data[..accounts_offset].copy_from_slice(header_bytes);
        data[accounts_offset..signers_offset].copy_from_slice(accounts_bytes);
        data[signers_offset..data_offset].copy_from_slice(signers_bytes);
        data[data_offset..data_offset + tx_data_len].copy_from_slice(tx_data);

        Ok(())
    }

    pub fn parse(
        data: &[u8],
    ) -> Result<
        (
            &TransactionHeader,
            &[TransactionAccount],
            &[TransactionSigner],
            &[u8],
        ),
        ProgramError,
    > {
        let header_size = core::mem::size_of::<TransactionHeader>();
        let header = bytemuck::try_from_bytes::<TransactionHeader>(&data[..header_size])
            .map_err(|_| ProgramError::InvalidAccountData)?;

        let num_accounts = u64::from_le_bytes(header.num_accounts);
        let accounts_size = num_accounts as usize * core::mem::size_of::<TransactionAccount>();
        let accounts_data = &data[header_size..header_size + accounts_size];
        let accounts = bytemuck::cast_slice::<u8, TransactionAccount>(accounts_data);

        let num_signers = u64::from_le_bytes(header.num_signers);
        let signers_size = num_signers as usize * core::mem::size_of::<TransactionSigner>();
        let signers_offset = header_size + accounts_size;
        let signers_data = &data[signers_offset..signers_offset + signers_size];
        let signers = bytemuck::cast_slice::<u8, TransactionSigner>(signers_data);

        let data_len = u64::from_le_bytes(header.data_len);
        let data_offset = header_size + accounts_size + signers_size;
        let tx_data = &data[data_offset..data_offset + data_len as usize];

        Ok((header, accounts, signers, tx_data))
    }

    pub fn size(num_accounts: u64, num_signers: u64, data_len: u64) -> usize {
        let header_size = core::mem::size_of::<TransactionHeader>();
        let accounts_size = num_accounts as usize * core::mem::size_of::<TransactionAccount>();
        let signers_size = num_signers as usize * core::mem::size_of::<TransactionSigner>();

        header_size + accounts_size + signers_size + data_len as usize
    }
}
