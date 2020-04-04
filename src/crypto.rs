use rand::{self, Rng};
use aead::{Aead, NewAead, Error, generic_array::GenericArray};
use aes_gcm_siv::Aes256GcmSiv;

pub struct Crypto {}
pub type Key = [u8; 32];
pub type IV = [u8; 12];

impl Crypto {

    pub fn encrypt(iv: IV, key: Key, data: &Vec<u8>) -> Result<Vec<u8>, Error> {

        let key_ga = GenericArray::from(key);
        let iv_ga = GenericArray::from(iv);

        let cipher = Aes256GcmSiv::new(key_ga);
        cipher.encrypt(&iv_ga, &**data)

    }

    pub fn decrypt(iv: IV, key: Key, data: &Vec<u8>) -> Result<Vec<u8>, Error> {

        let key_ga = GenericArray::from(key);
        let iv_ga = GenericArray::from(iv);

        let cipher = Aes256GcmSiv::new(key_ga);
        cipher.decrypt(&iv_ga, &**data)

    }

    pub fn gen_iv() -> IV {

        rand::thread_rng().gen::<[u8; 12]>()

    }

}

