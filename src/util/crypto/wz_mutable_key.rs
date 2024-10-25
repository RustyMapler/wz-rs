use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockEncrypt, KeyInit};
use aes::{Aes256, Block};
use std::f32;

#[derive(Clone)]
pub struct WzMutableKey {
    pub iv: [u8; 4],
    pub aes_user_key: [u8; 32],
    pub key: Option<Vec<u8>>,
}

impl WzMutableKey {
    pub const BATCH_SIZE: usize = 4096;

    pub fn at(&mut self, index: usize) -> u8 {
        if self.key.is_none() || self.key.as_ref().unwrap().len() <= index {
            self.ensure_key_size(index + 1);
        }

        self.key.as_ref().unwrap()[index]
    }

    fn ensure_key_size(&mut self, size: usize) {
        // If the key is already the correct size, do nothing
        if let Some(ref key) = self.key {
            if key.len() >= size {
                return;
            }
        }

        // Calculate the new size
        let batch_count = (size as f32 / WzMutableKey::BATCH_SIZE as f32).ceil() as usize;
        let new_size = batch_count * WzMutableKey::BATCH_SIZE;
        log::trace!("Calculated new key size: {}", new_size);

        let mut new_key: Vec<u8> = Vec::with_capacity(new_size);
        let key = GenericArray::from_slice(&self.aes_user_key);
        let cipher = Aes256::new(key);

        for i in (0..new_size).step_by(16) {
            let mut block = Block::default();

            if i == 0 {
                for j in 0..block.len() {
                    block[j] = self.iv[j % 4];
                }
            } else {
                block.copy_from_slice(&new_key[i - 16..i]);
            }

            // Encrypt block in-place
            cipher.encrypt_block(&mut block);
            new_key.extend_from_slice(&block);
        }

        self.key = Some(new_key);
    }
}
