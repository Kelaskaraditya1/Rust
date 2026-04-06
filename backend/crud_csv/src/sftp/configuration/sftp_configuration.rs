
use std::net::TcpStream;

use anyhow::Ok;
use ssh2::{Session, Sftp};
use anyhow::Result;
use crate::{ keys};

pub struct SftpClient{
    pub sftp:Sftp
}

impl SftpClient{

    pub async fn connect() -> Result<Self> {

        let tcp_address = keys::keys::TCP_ADDRESS.to_string();
        let tcp_connection = TcpStream::connect(tcp_address)?;

        let mut session = Session::new()?;
        session.set_tcp_stream(tcp_connection);
        session.handshake()?;

        let username = keys::keys::SFTP_USERNAME.to_string();
        let password = keys::keys::SFTP_PASSWORD.to_string();

        session.userauth_password(&username, &password)?;

        Ok(Self { sftp: session.sftp()? })
        
    }

}
