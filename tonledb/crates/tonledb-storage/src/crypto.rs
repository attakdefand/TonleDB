use aes_gcm::{Aes256Gcm, aead::{Aead, KeyInit, Payload}, Key, Nonce};
use rand::RngCore;
use tonledb_core::{Result, Space, Storage, DbError};

pub struct CryptoStorage<S: Storage> { inner: S, dek: [u8;32] }

impl<S: Storage> CryptoStorage<S> {
    pub fn new(inner:S, kek_b64:&str)->Result<Self>{
        let kek = base64::decode(kek_b64).map_err(|e| DbError::Invalid(format!("KEK b64: {e}")))?;
        if kek.len()!=32 { return Err(DbError::Invalid("KEK must be 32 bytes".into())); }
        let mut dek=[0u8;32]; rand::thread_rng().fill_bytes(&mut dek);
        Ok(Self{ inner, dek })
    }
    fn seal(&self, pt:&[u8], space:&Space, key:&[u8])->Result<Vec<u8>>{
        let aead=Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&self.dek));
        let mut nonce=[0u8;12]; rand::thread_rng().fill_bytes(&mut nonce);
        let mut aad=Vec::new(); aad.extend_from_slice(space.0.as_bytes()); aad.extend_from_slice(key);
        let mut ct=aead.encrypt(Nonce::from_slice(&nonce), Payload{msg:pt, aad:&aad}).map_err(|e|DbError::Storage(e.to_string()))?;
        let mut out=Vec::with_capacity(12+ct.len()); out.extend_from_slice(&nonce); out.append(&mut ct); Ok(out)
    }
    fn open(&self, blob:&[u8], space:&Space, key:&[u8])->Result<Vec<u8>>{
        if blob.len()<12 { return Err(DbError::Storage("ciphertext too short".into())); }
        let (nonce, ct)=blob.split_at(12);
        let aead=Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&self.dek));
        let mut aad=Vec::new(); aad.extend_from_slice(space.0.as_bytes()); aad.extend_from_slice(key);
        aead.decrypt(Nonce::from_slice(nonce), Payload{msg:ct, aad:&aad}).map_err(|_| DbError::Storage("decrypt failed".into()))
    }
}
impl<S: Storage> Storage for CryptoStorage<S>{
    fn get(&self, space:&Space, key:&[u8])->Result<Option<Vec<u8>>>{
        match self.inner.get(space,key)? { Some(ct)=>Ok(Some(self.open(&ct,space,key)?)), None=>Ok(None) }
    }
    fn put(&self, space:&Space, key:Vec<u8>, val:Vec<u8>)->Result<()>{
        self.inner.put(space, key.clone(), self.seal(&val,space,&key)?) }
    fn del(&self, space:&Space, key:&[u8])->Result<()> { self.inner.del(space,key) }
    fn scan_prefix(&self, space:&Space, prefix:&[u8])->Result<Box<dyn Iterator<Item=(Vec<u8>,Vec<u8>)>+Send>>{
        let it=self.inner.scan_prefix(space,prefix)?;
        let dek=self.dek; let sp=space.clone();
        Ok(Box::new(it.filter_map(move|(k,v)|{
            if v.len()<12 { return None; }
            let aead=Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&dek));
            let (nonce,ct)=v.split_at(12);
            let mut aad=Vec::new(); aad.extend_from_slice(sp.0.as_bytes()); aad.extend_from_slice(&k);
            aead.decrypt(Nonce::from_slice(nonce), Payload{msg:ct, aad:&aad}).ok().map(|pt|(k,pt))
        })))
    }
}
