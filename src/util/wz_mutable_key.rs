use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockEncrypt, KeyInit};
use aes::{Aes256, Block};
use std::f32;
use std::vec;

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

    pub fn ensure_key_size(&mut self, size: usize) {
        if self.key.is_some() && self.key.as_ref().unwrap().len() >= size {
            return;
        }

        let new_size = ((size as f32 / WzMutableKey::BATCH_SIZE as f32).ceil() as usize)
            * WzMutableKey::BATCH_SIZE;
        log::trace!("new key size {}", new_size);
        let mut new_key: Vec<u8> = vec![];

        let start_index = 0;

        let key = GenericArray::from_slice(&self.aes_user_key);
        // Initialize cipher
        let cipher = Aes256::new(key);

        let mut i = start_index;
        while i < new_size {
            let mut block = Block::default();

            if i == 0 {
                for j in 0..block.len() {
                    block[j] = self.iv[j % 4];
                }
            } else {
                for j in 0..block.len() {
                    block[j] = new_key[i - 16 + j];
                }
            }

            // Encrypt block in-place
            cipher.encrypt_block(&mut block);
            new_key.append(&mut block.to_vec());

            i += 16;
        }

        self.key = Some(new_key)
    }
}
