// use openssl::rsa::Rsa;
// use functions as fx;
// use functions_crypto::*;
use pem::{parse};
// use sha2::{Sha256, Sha512, Digest};
use json;

pub fn main() {

    let pem_data = parse("-----BEGIN TRANSACTION SIGNATURE-----
ewogICAgInRva2VuIjogIjUuMzI0IiwKICAgICJzZW5kZXJfcHVibGljX2FkZHJl
c3MiOiAiYUZnRkhOV2UyMXk0RDhKV0VTRThjUGlGMkZOZEJvRmciLAogICAgInJl
Y2lwaWVudF9wdWJsaWNfYWRkcmVzcyI6ICJjdkxNeDVnME5uVms4dHJtcmZpUHdx
c3VRWkFyYk14OCIsCiAgICAiY3JlYXRlZF9hdCI6IDE2MjA1ODI4NjYsCiAgICAi
c2VuZGVyX3ByaXZhdGVfa2V5X2hhc2giOiAiRUZFNjg1NTE2ODE4RkQyRkZEOEIz
N0M2QjVENTYzMDU0NTI1MTAyQzRCODQyODY0OTY1NEU4NEE0MzNFMTQ3NyIsCiAg
ICAiaGFzaCI6ICI0MEVDOTEyOUQyNDNFMTUwRjJBQkQ4MUY4ODRENjZGNTQ1NEMw
MTE5ODU5QjM5NTlEREZBMUU2MzRDRTNGQUJCIgp9
-----END TRANSACTION SIGNATURE-----
").unwrap();

    let stringify_pretty = String::from_utf8(pem_data.contents).unwrap();

    let data = json::parse(&stringify_pretty);

    println!("{:?}",data);
}


