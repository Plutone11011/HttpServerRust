use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::Display;

#[derive(Debug)]
struct HttpRequest {
    method: Method,
    route: Resource,
    version: Version,
    headers: HttpHeader,
    request_body: String
}

#[derive(Debug)]
struct HttpHeader{
    headers: HashMap<String, String>
}

impl HttpHeader {
    pub fn new(request: &str) -> Option<HttpHeader> {
        let mut http_header = HttpHeader {
            headers: HashMap::new()
        };

        let (_, header_str) = request.split_once("\r\n")?;
        
        for line in header_str.split_terminator("\r\n"){
            if line.is_empty(){
                // break because it isn't a valid
                // http request if the line doesn't end with \r\n
                break;
            }
            let (header, value) = line.split_once(":")?;
            http_header.headers.insert(header.trim().to_string(), value.trim().to_string());
        }

        Some(http_header)
    }
}

#[derive(Debug)]
enum Method {
    Get,
    Post,
    Uninitialized
}

impl Method {
    pub fn new(request: &str) -> Method{
        let request_split = request.split_once("\r\n");
        if let Some((method_line, _rest)) = request_split {
            let method = method_line.split_once("\r\n");
            if let Some((method, _rest)) = method {
                return match method {
                    "GET" => Method::Get,
                    "POST" => Method::Post,
                    _ => Method::Uninitialized
                }
            };
        };
        Method::Uninitialized
    }

    pub fn identify(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized
        }
    }
}

#[derive(Debug)]
enum Version {
    V1_1,
    V2_0
}
#[derive(Debug)]
struct VersionError {
    msg: String
}

impl Display for VersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Version {

    pub fn new(request: &str) -> Result<Self, VersionError> {
        Version::from_str(request)
    } 
}
// use trait to parse new type
impl FromStr for Version {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let request_split = s.split_once("\r\n");
        if let Some((method_line, _rest)) = request_split {
            let splits = method_line.split_ascii_whitespace();
            for split in splits {
                if split == "HTTP/1.1"{
                    return Ok(Version::V1_1)
                }
                else if split == "HTTP/2.0" {
                    return Ok(Version::V2_0)
                }
            }
        };

        let invalid = format!("Unknown protocol version in {}", s);
        let version_error = VersionError {
            msg: invalid
        };
        Err(version_error)
    }
}

#[derive(Debug)]
struct Resource {
    path: String
}

impl Resource {
    pub fn new(request: &str) -> Option<Resource> {
        if let Some((request_method, _)) = request.split_once("\r\n") {
            let (method, rest) = request_method.split_once(' ')?;
            return match Method::identify(method) {
                Method::Get | Method::Post => {
                    let (resource, _protocol_version) =  rest.split_once(' ')?;
                    let resource = resource.trim();
                    let resource = resource.trim_start_matches("/");
                    return Some(Resource {
                        path:resource.to_string()
                    })
                },
                Method::Uninitialized => None
            };
        };
        None
    }
}