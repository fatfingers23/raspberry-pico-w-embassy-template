use core::fmt::write as fmt_write;
use core::str;
use defmt::*;
use embassy_net::{tcp::TcpSocket, Stack};
use embassy_time::Duration;
use embedded_io_async::Write;
use heapless::Vec;
use httparse::Header;

use crate::io::BufWriter;

pub struct HttpServer {
    port: u16,
    stack: Stack<'static>,
}

impl HttpServer {
    pub fn new(port: u16, stack: Stack<'static>) -> Self {
        Self { port, stack }
    }

    pub async fn serve<H>(&mut self, mut handler: H)
    where
        H: WebRequestHandler,
    {
        let mut rx_buffer = [0; 8_192];
        let mut tx_buffer = [0; 8_192];
        let mut buf = [0; 8_192];
        let ip = self.stack.config_v4().unwrap().address;
        info!("Listening on ip: {}", ip);
        loop {
            let mut socket = TcpSocket::new(self.stack, &mut rx_buffer, &mut tx_buffer);
            socket.set_timeout(Some(Duration::from_secs(10)));

            if let Err(e) = socket.accept(self.port).await {
                warn!("accept error: {:?}", e);
                continue;
            }

            info!("Received connection from {:?}", socket.remote_endpoint());

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(0) => {
                        warn!("read EOF");
                        break;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        warn!("read error: {:?}", e);
                        break;
                    }
                };

                let mut headers = [httparse::EMPTY_HEADER; 20];

                let request = self.request_parser(&mut buf[..n], &mut headers);
                match request {
                    Some(request) => {
                        let mut request_response_buffer = [0u8; 8_192]; // Size the buffer appropriately
                        let response = handler
                            .handle_request(request, &mut request_response_buffer)
                            .await;

                        let mut response_buffer = [0u8; 8_192];
                        let mut writer: BufWriter<'_> = BufWriter::new(&mut response_buffer);

                        if response.is_err() {
                            warn!("Something went wrong with the request");
                            socket.close();
                            break;
                        }
                        match response.unwrap().write_response(&mut writer) {
                            Ok(()) => {}
                            Err(_) => {
                                warn!("Error writing response");
                                let mut bad_response_buffer = [0u8; 300];
                                let bad_response = Response::new_html(
                                    StatusCode::InternalServerError,
                                    "Error writing response",
                                );
                                let mut writer: BufWriter<'_> =
                                    BufWriter::new(&mut bad_response_buffer);
                                match bad_response.write_response(&mut writer) {
                                    Ok(()) => {}
                                    Err(_) => {
                                        warn!("Error writing any response");
                                        break;
                                    }
                                };
                                //Already a hail mary, so just ignore the error
                                let _ = socket.write_all(&bad_response_buffer).await;
                            }
                        };

                        let response_len: usize = writer.len();

                        match socket.write_all(&response_buffer[..response_len]).await {
                            Ok(()) => {}
                            Err(e) => {
                                warn!("write error: {:?}", e);
                                break;
                            }
                        };
                    }
                    None => {
                        warn!("Was not a proper web request");
                    }
                }

                //Have to close the socket so the web browser knows its done
                socket.close();
            }
        }
    }
    pub fn request_parser<'headers, 'buf>(
        &mut self,
        request_buffer: &'buf [u8],
        headers: &'headers mut [Header<'buf>],
    ) -> Option<WebRequest<'headers, 'buf>> {
        let mut request: httparse::Request<'headers, 'buf> = httparse::Request::new(headers);
        let attempt_to_parse = request.parse(request_buffer);
        if let Err(_) = attempt_to_parse {
            info!("Failed to parse request");
            return None;
        }
        let res = attempt_to_parse.unwrap();
        if res.is_partial() {
            info!("Was not a proper web request");
            return None;
        }

        // Split the request buffer into headers and body
        let mut headers_end = 0;
        for window in request_buffer.windows(4) {
            if window == b"\r\n\r\n" {
                headers_end = window.as_ptr() as usize - request_buffer.as_ptr() as usize + 4;
                break;
            }
        }

        let body = &request_buffer[headers_end..];

        Some(WebRequest {
            method: Method::new(request.method.unwrap()),
            path: request.path,
            body: match core::str::from_utf8(body) {
                Ok(body) => body,
                Err(_) => "",
            },
            headers: request.headers,
        })
    }
}

#[allow(dead_code)]
pub struct WebRequest<'headers, 'buf> {
    pub method: Option<Method>,
    pub path: Option<&'buf str>,
    pub body: &'buf str,
    pub headers: &'headers mut [Header<'buf>],
}

#[derive(Debug)]
pub enum WebRequestHandlerError {}

pub trait WebRequestHandler {
    async fn handle_request<'a>(
        &mut self,
        request: WebRequest,
        response_buffer: &'a mut [u8],
    ) -> Result<Response<'a>, WebRequestHandlerError>;
}

pub enum Method {
    Delete,
    Get,
    Head,
    Post,
    Put,
    Connect,
    Options,
    Trace,
    Copy,
    Lock,
    MkCol,
    Move,
    Propfind,
    Proppatch,
    Search,
    Unlock,
    Bind,
    Rebind,
    Unbind,
    Acl,
    Report,
    MkActivity,
    Checkout,
    Merge,
    MSearch,
    Notify,
    Subscribe,
    Unsubscribe,
    Patch,
    Purge,
    MkCalendar,
    Link,
    Unlink,
}

impl Method {
    pub fn new(method: &str) -> Option<Self> {
        if method.eq_ignore_ascii_case("Delete") {
            Some(Self::Delete)
        } else if method.eq_ignore_ascii_case("Get") {
            Some(Self::Get)
        } else if method.eq_ignore_ascii_case("Head") {
            Some(Self::Head)
        } else if method.eq_ignore_ascii_case("Post") {
            Some(Self::Post)
        } else if method.eq_ignore_ascii_case("Put") {
            Some(Self::Put)
        } else if method.eq_ignore_ascii_case("Connect") {
            Some(Self::Connect)
        } else if method.eq_ignore_ascii_case("Options") {
            Some(Self::Options)
        } else if method.eq_ignore_ascii_case("Trace") {
            Some(Self::Trace)
        } else if method.eq_ignore_ascii_case("Copy") {
            Some(Self::Copy)
        } else if method.eq_ignore_ascii_case("Lock") {
            Some(Self::Lock)
        } else if method.eq_ignore_ascii_case("MkCol") {
            Some(Self::MkCol)
        } else if method.eq_ignore_ascii_case("Move") {
            Some(Self::Move)
        } else if method.eq_ignore_ascii_case("Propfind") {
            Some(Self::Propfind)
        } else if method.eq_ignore_ascii_case("Proppatch") {
            Some(Self::Proppatch)
        } else if method.eq_ignore_ascii_case("Search") {
            Some(Self::Search)
        } else if method.eq_ignore_ascii_case("Unlock") {
            Some(Self::Unlock)
        } else if method.eq_ignore_ascii_case("Bind") {
            Some(Self::Bind)
        } else if method.eq_ignore_ascii_case("Rebind") {
            Some(Self::Rebind)
        } else if method.eq_ignore_ascii_case("Unbind") {
            Some(Self::Unbind)
        } else if method.eq_ignore_ascii_case("Acl") {
            Some(Self::Acl)
        } else if method.eq_ignore_ascii_case("Report") {
            Some(Self::Report)
        } else if method.eq_ignore_ascii_case("MkActivity") {
            Some(Self::MkActivity)
        } else if method.eq_ignore_ascii_case("Checkout") {
            Some(Self::Checkout)
        } else if method.eq_ignore_ascii_case("Merge") {
            Some(Self::Merge)
        } else if method.eq_ignore_ascii_case("MSearch") {
            Some(Self::MSearch)
        } else if method.eq_ignore_ascii_case("Notify") {
            Some(Self::Notify)
        } else if method.eq_ignore_ascii_case("Subscribe") {
            Some(Self::Subscribe)
        } else if method.eq_ignore_ascii_case("Unsubscribe") {
            Some(Self::Unsubscribe)
        } else if method.eq_ignore_ascii_case("Patch") {
            Some(Self::Patch)
        } else if method.eq_ignore_ascii_case("Purge") {
            Some(Self::Purge)
        } else if method.eq_ignore_ascii_case("MkCalendar") {
            Some(Self::MkCalendar)
        } else if method.eq_ignore_ascii_case("Link") {
            Some(Self::Link)
        } else if method.eq_ignore_ascii_case("Unlink") {
            Some(Self::Unlink)
        } else {
            None
        }
    }

    fn _as_str(&self) -> &'static str {
        match self {
            Self::Delete => "DELETE",
            Self::Get => "GET",
            Self::Head => "HEAD",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Connect => "CONNECT",
            Self::Options => "OPTIONS",
            Self::Trace => "TRACE",
            Self::Copy => "COPY",
            Self::Lock => "LOCK",
            Self::MkCol => "MKCOL",
            Self::Move => "MOVE",
            Self::Propfind => "PROPFIND",
            Self::Proppatch => "PROPPATCH",
            Self::Search => "SEARCH",
            Self::Unlock => "UNLOCK",
            Self::Bind => "BIND",
            Self::Rebind => "REBIND",
            Self::Unbind => "UNBIND",
            Self::Acl => "ACL",
            Self::Report => "REPORT",
            Self::MkActivity => "MKACTIVITY",
            Self::Checkout => "CHECKOUT",
            Self::Merge => "MERGE",
            Self::MSearch => "MSEARCH",
            Self::Notify => "NOTIFY",
            Self::Subscribe => "SUBSCRIBE",
            Self::Unsubscribe => "UNSUBSCRIBE",
            Self::Patch => "PATCH",
            Self::Purge => "PURGE",
            Self::MkCalendar => "MKCALENDAR",
            Self::Link => "LINK",
            Self::Unlink => "UNLINK",
        }
    }
}

#[allow(dead_code)]
pub enum StatusCode {
    Ok,
    Created,
    Accepted,
    NoContent,
    MovedPermanently,
    MovedTemporarily,
    NotModified,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
}

impl StatusCode {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Ok => "200 OK",
            Self::Created => "201 Created",
            Self::Accepted => "202 Accepted",
            Self::NoContent => "204 No Content",
            Self::MovedPermanently => "301 Moved Permanently",
            Self::MovedTemporarily => "302 Moved Temporarily",
            Self::NotModified => "304 Not Modified",
            Self::BadRequest => "400 Bad Request",
            Self::Unauthorized => "401 Unauthorized",
            Self::Forbidden => "403 Forbidden",
            Self::NotFound => "404 Not Found",
            Self::InternalServerError => "500 Internal Server Error",
            Self::NotImplemented => "501 Not Implemented",
            Self::BadGateway => "502 Bad Gateway",
            Self::ServiceUnavailable => "503 Service Unavailable",
        }
    }

    fn _as_u16(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::Created => 201,
            Self::Accepted => 202,
            Self::NoContent => 204,
            Self::MovedPermanently => 301,
            Self::MovedTemporarily => 302,
            Self::NotModified => 304,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::InternalServerError => 500,
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::ServiceUnavailable => 503,
        }
    }
}

type ResponseHeader = (&'static str, &'static str);

pub struct Response<'a> {
    status_code: StatusCode,
    body: &'a str,
    headers: Vec<ResponseHeader, 5>,
}

#[allow(dead_code)]
impl<'a> Response<'a> {
    pub fn new(status_code: StatusCode, body: &'static str) -> Self {
        Self {
            status_code: status_code,
            body,
            headers: Vec::new(),
        }
    }

    pub fn new_html(status_code: StatusCode, body: &'a str) -> Self {
        let headers: Vec<ResponseHeader, 5> =
            Vec::from_slice(&[("Content-type", "text/html")]).unwrap();

        Self {
            status_code: status_code,
            body,
            headers,
        }
    }

    pub fn json_response(status_code: StatusCode, body: &'a str) -> Self {
        let headers: Vec<ResponseHeader, 5> =
            Vec::from_slice(&[("Content-type", "application/json")]).unwrap();

        Self {
            status_code: status_code,
            body,
            headers,
        }
    }

    pub fn write_response<W>(&self, writer: &mut W) -> Result<(), core::fmt::Error>
    where
        W: core::fmt::Write,
    {
        let _ = fmt_write(
            writer,
            format_args!("HTTP/1.1 {} \r\n", self.status_code.as_str(),),
        );

        for (key, value) in self.headers.iter() {
            let _ = fmt_write(writer, format_args!("{}: {}\r\n", key, value));
        }
        let _ = fmt_write(writer, format_args!("\r\n"));
        writer.write_str(self.body)?;

        Ok(())
    }
}
