use serde::{Deserialize, Serialize};
use mysql::*;
use indexmap::IndexMap;


pub struct DBParameters<'a>{
    pub conn_pool: &'a Pool,
    pub table_name: &'a str,
    pub columns: IndexMap<&'a str, &'a str>,
    pub update: IndexMap<&'a str, &'a str>,
    pub query_append: Option<&'a str>,
    pub count: Option<i32>
}


pub struct DBParametersWithColumns<'a>{
    pub conn_pool: &'a Pool,
    pub table_name: &'a str,
    pub columns: IndexMap<&'a str, &'a str>
}



pub struct DBParametersWithUpdate<'a>{
    pub conn_pool: &'a Pool,
    pub table_name: &'a str,
    pub columns: IndexMap<&'a str, &'a str>,
    pub update: IndexMap<&'a str, &'a str>
}


#[derive(Serialize, Deserialize)]
pub struct PortalProperties{
    pub ssh: PPSSH,
    pub settings: PPSettings
}

#[derive(Serialize, Deserialize)]
pub struct PPSettings{
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub site_port: i32,
    pub rel_dirname: String,
    pub admissions_rel_dirname: String,
    pub portal_id: String
}



#[derive(Serialize, Deserialize)]
pub struct PPSSH{
    pub host: String,
    pub username: String,
    pub password: String,
    pub passphrase: String
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User{
    pub id: i64,
    pub public_address: String,
    pub private_key: String,
    pub email_address: String,
    pub phone_number: String,
    pub country: String,
    pub token: f32,
    pub bonus: f32,
    pub created_at: i32
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Settings{
    pub host: String,
    pub db_host: String,
    pub db_user: String,
    pub db_name: String,
    pub rel_dirname: String,
    pub site_port: String,
    pub db_password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeSettings{
    pub name: String,
    pub node_id: String,
    pub host: String,
    pub db_host: String,
    pub node_base_url: String,
    pub rel_dirname: String,
    pub node_url: String,
    pub domain: String,
    pub handshake_auth_key: String,
    pub active: bool,
    pub db: NodeDBSettings,
    pub ssh: NodeSSHSettings
}


#[derive(Serialize, Deserialize, Debug)]
pub struct NodeDBSettings{
    pub name: String,
    pub user: String,
    pub password: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct NodeSSHSettings{
    pub username: String,
    pub password: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegistrationInfo{
    pub email: String,
    pub phone_number: String,
    pub country: String
}