use crate::postgres_driver::errors::DecoderError;
use crate::postgres_driver::message::StartupMessage;
use anyhow::*;
use byteorder::{ByteOrder, NetworkEndian};
use bytes::{Buf, BytesMut};
use std::char;
use std::collections::HashMap;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Decoder, Encoder};
// Postgres protocol version.
const VERSION_3: i32 = 0x30000;
pub const VERSION_SSL: i32 = (1234 << 16) + 5679;

// decode_startup_message decode pg startup message.
pub async fn decode_startup_message<T>(mut conn: T) -> Result<StartupMessage, DecoderError>
where
    T: AsyncRead + Unpin,
{
    // read the frame length.
    let len = decode_frame_length(&mut conn).await?;
    let mut buf = BytesMut::new();
    buf.resize(len, b'0');
    conn.read_exact(&mut buf).await?;
    // read version number
    let version_number = buf.get_i32();
    match version_number {
        VERSION_SSL => return Ok(StartupMessage::SslRequest),
        VERSION_3 => {
            let mut params = HashMap::new();
            // read all the params.
            // have to make it safe.
            while *buf.get(0).unwrap() != 0 {
                let key =
                    read_cstr(&mut buf).map_err(|_| anyhow!("error while reading key params"))?;
                let val =
                    read_cstr(&mut buf).map_err(|_| anyhow!("error while reading value params"))?;
                params.insert(key, val);
            }

            return Ok(StartupMessage::Startup {
                params: params,
                version: version_number,
            });
        }
        _ => {
            return Err(DecoderError::UnsupporedVersion);
        }
    };
}

async fn decode_frame_length<T>(mut conn: T) -> Result<usize, anyhow::Error>
where
    T: AsyncRead + Unpin,
{
    let mut buf = [0; 4];
    conn.read_exact(&mut buf).await?;
    let frame_len = NetworkEndian::read_u32(&buf) as usize;
    if frame_len < 4 {
        // client didn't include the length of frame length itself.
        return Err(anyhow!("invalid frame length"));
    }
    Ok(frame_len - 4)
}

fn read_cstr(buf: &mut BytesMut) -> Result<String, Error> {
    if let Some(pos) = buf.iter().position(|d| *d == 0) {
        let str = std::str::from_utf8(&buf[..pos])
            .map_err(|_| anyhow!("error while reading cstr"))?
            .to_string();
        buf.advance(pos + 1);
        return Ok(str);
    }
    Err(anyhow!("string has not termination deliminiter"))
}

pub async fn decode_password_message<T>(mut conn: T) -> Result<StartupMessage, anyhow::Error>
where
    T: AsyncRead + Unpin,
{
    let mut buf = [0; 1];
    conn.read_exact(&mut buf).await?;
    if buf[0] != b'p' {
        return Err(anyhow!("incoming message is not a password message"));
    }
    let len = decode_frame_length(&mut conn).await.map_err(|_| {
        anyhow!("error while decoding frame length while decoding password message")
    })?;
    let mut buf = BytesMut::new();
    buf.resize(len, b'0');
    conn.read_exact(&mut buf).await?;
    // read the passcode.
    let password =
        read_cstr(&mut buf).map_err(|err| anyhow!("error while reading password {:?}", err))?;
    Ok(StartupMessage::PasswordMessage { password: password })
}
