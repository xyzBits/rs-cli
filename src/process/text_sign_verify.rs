use crate::{process_genpass, TextSignFormat};
use anyhow::Result;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::{collections::HashMap, io::Read};

pub trait TextSigner {
    // signer could sign any input data
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerifier {
    // verifier could verify any input data
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}


pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}


impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        // 读取所有的 bytes，直到 EOF
        reader.read_to_end(&mut buf)?;

        let hash = blake3::keyed_hash(&self.key, &buf);

        Ok(hash.as_bytes().to_vec())
    }
}


impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        // 对原始数据再次进行 keyed hash，验证其结果和签名是否一致
        // 这个其实就是 哈希验证
        let hash = blake3::keyed_hash(&self.key, &buf);

        Ok(hash.as_bytes() == sig)
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        // 使用 ed25519 算法生成的 私钥 对 数据进行签名
        let signature = self.key.sign(&buf);

        Ok(signature.to_bytes().to_vec())
    }
}


impl TextVerifier for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let sig = (&sig[..64]).try_into()?;
        let signature = Signature::from_bytes(&sig);

        // verify 方法是 Verifier trait 定义的方法，所以在使用时，需要将 trait import 进来
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}

impl Blake3 {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();

        let key = (&key[..32]).try_into()?;

        Ok(Self::new(key))
    }

    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {

        // 使用 genpass 生成 32 位长的密码，作为 key
        let key = process_genpass(32, true, true, true, true)?;

        let mut map = HashMap::new();
        map.insert("blake3.txt", key.as_bytes().to_vec());

        Ok(map)
    }
}


impl Ed25519Signer {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        // 只取前 32 位
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;

        Ok(Self::new(key))
    }

    pub fn new(key: &[u8; 32]) -> Self {
        let key = SigningKey::from_bytes(key);
        Self {
            key
        }
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let mut rng = OsRng;
        // 使用随机数生成私钥
        let signing_key = SigningKey::generate(&mut rng);

        // 由私钥导出公钥
        // impl From<&SigningKey> for VerifyingKey {
        // 因为 verifyingKey 实现了 From trait
        // 因此可以支持转换
        let verifying_key: VerifyingKey = (&signing_key).into();

        let mut map = HashMap::new();
        map.insert("ed25519.signing_key", signing_key.to_bytes().to_vec());
        map.insert("ed25519.verifying_key", verifying_key.to_bytes().to_vec());

        Ok(map)
    }
}

impl Ed25519Verifier {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;

        let verifying_key = VerifyingKey::from_bytes(key)?;
        Ok(Self {
            key: verifying_key
        })
    }
}

pub fn process_text_sign(
    reader: &mut dyn Read,
    key: &[u8],
    format: TextSignFormat,
) -> Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?)
    };

    signer.sign(reader)
}

pub fn process_text_verify(
    reader: &mut dyn Read,
    key: &[u8],
    sig: &[u8],
    format: TextSignFormat,
) -> Result<bool> {
    let verifier: Box<dyn TextVerifier> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?)
    };

    verifier.verify(reader, sig)
}

pub fn process_text_key_generate(format: TextSignFormat)
                                 -> Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &[u8] = include_bytes!("../../fixtures/");


    #[test]
    fn test_process_text_sign() -> Result<()> {
        let mut reader = "hello".as_bytes();
        let mut reader1 = "hello".as_bytes();

        let format = TextSignFormat::Blake3;

        // process_text_sign(&mut reader, )

        Ok(())
    }
}










