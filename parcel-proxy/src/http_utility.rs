use http::{Request, Response};
use tokio::io::{AsyncRead, AsyncReadExt};

pub trait ToHttp {
    fn to_raw_http(&self, new_host_header: Option<&str>) -> String;
}

impl ToHttp for Response<String> {
    fn to_raw_http(&self, new_host_header: Option<&str>) -> String {
        let mut result = String::with_capacity(128);

        result.push_str(format!("{:?} {}\r\n", self.version(), self.status()).as_str());

        for (name, value) in self.headers() {
            let mut value = String::from_utf8_lossy(value.as_bytes());
            if name.as_str().eq_ignore_ascii_case("host") {
                if let Some(new_host_header) = new_host_header {
                    value = new_host_header.into();
                }
            }

            result.push_str(format!("{}: {}\r\n", name, value).as_str());
        }

        result.push_str("\r\n");

        let body = self.body();

        if !body.is_empty() {
            result.push_str(body);
        }

        result
    }
}

impl ToHttp for Request<String> {
    fn to_raw_http(&self, new_host_header: Option<&str>) -> String {
        let mut result = String::with_capacity(128);

        result.push_str(&format!(
            "{} {} {:?}\r\n",
            self.method(),
            self.uri(),
            self.version()
        ));

        for (name, value) in self.headers() {
            let mut value = String::from_utf8_lossy(value.as_bytes());
            if name.as_str().eq_ignore_ascii_case("host") {
                if let Some(new_host_header) = new_host_header {
                    value = new_host_header.into();
                }
            }

            result.push_str(format!("{}: {}\r\n", name, value).as_str());
        }

        result.push_str("\r\n");

        result.push_str(self.body());

        result
    }
}

pub async fn read_headers<T>(stream: &mut T) -> anyhow::Result<Vec<u8>>
where
    T: AsyncRead + Unpin,
{
    let mut buf = Vec::with_capacity(128);

    loop {
        let u8 = stream.read_u8().await?;

        buf.push(u8);

        if u8 == b'\n' && buf.ends_with(b"\r\n\r\n") {
            break;
        }
    }

    Ok(buf)
}
