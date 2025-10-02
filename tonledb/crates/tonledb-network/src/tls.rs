use std::{fs::File, io::BufReader, sync::Arc};
use rustls::{Certificate, PrivateKey, RootCertStore, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

pub fn tls_config(cert_path: &str, key_path: &str, ca_path: Option<&str>, require_client_auth: bool) -> anyhow::Result<Arc<ServerConfig>> {
    let mut cert_reader = BufReader::new(File::open(cert_path)?);
    let mut key_reader  = BufReader::new(File::open(key_path)?);

    let cert_chain = certs(&mut cert_reader).map_err(|_| anyhow::anyhow!("Failed to read certificates"))?
        .into_iter()
        .map(Certificate)
        .collect::<Vec<_>>();
        
    let keys = pkcs8_private_keys(&mut key_reader).map_err(|_| anyhow::anyhow!("Failed to read private keys"))?;
    anyhow::ensure!(!keys.is_empty(), "no private keys found");
    let key = PrivateKey(keys.into_iter().next().unwrap());

    let cfg = if require_client_auth {
        let mut roots = RootCertStore::empty();
        if let Some(ca) = ca_path {
            let mut r = BufReader::new(File::open(ca)?);
            for c in certs(&mut r).map_err(|_| anyhow::anyhow!("Failed to read CA certificates"))? { 
                roots.add(&Certificate(c)).ok(); 
            }
        }
        ServerConfig::builder()
            .with_safe_defaults()
            .with_client_cert_verifier(Arc::new(rustls::server::AllowAnyAuthenticatedClient::new(roots)))
            .with_single_cert(cert_chain, key)?
    } else {
        ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key)?
    };

    Ok(Arc::new(cfg))
}