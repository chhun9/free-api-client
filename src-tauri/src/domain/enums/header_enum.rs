use super::*;

#[derive(Serialize, Deserialize, Debug, Clone, EnumIter)]
pub enum HttpHeader {
    Accept,
    Accept_Charset,
    Accept_Encoding,
    Accept_Language,
    Accept_Ranges,
    Age,
    Allow,
    Authorization,
    Cache_Control,
    Connection,
    Content_Disposition,
    Content_Encoding,
    Content_Language,
    Content_Length,
    Content_Location,
    Content_Range,
    Content_Type,
    Cookie,
    Date,
    ETag,
    Expect,
    Expires,
    Forwarded,
    From,
    Host,
    If_Match,
    If_Modified_Since,
    If_None_Match,
    If_Range,
    If_Unmodified_Since,
    Keep_Alive,
    Last_Modified,
    Link,
    Location,
    Max_Forwards,
    Origin,
    Pragma,
    Proxy_Authenticate,
    Proxy_Authorization,
    Range,
    Referer,
    Retry_After,
    Sec_Fetch_Dest,
    Sec_Fetch_Mode,
    Sec_Fetch_Site,
    Sec_Fetch_User,
    Server,
    Set_Cookie,
    Strict_Transport_Security,
    TE,
    Trailer,
    Transfer_Encoding,
    Upgrade,
    User_Agent,
    Vary,
    Via,
    Warning,
    WWW_Authenticate,
    X_Content_Type_Options,
    X_DNS_Prefetch_Control,
    X_Frame_Options,
    X_Powered_By,
    X_XSS_Protection,
}

impl HttpHeader {
    pub fn as_str(&self) -> &str {
        match self {
            HttpHeader::Accept => "Accept",
            HttpHeader::Accept_Charset => "Accept-Charset",
            HttpHeader::Accept_Encoding => "Accept-Encoding",
            HttpHeader::Accept_Language => "Accept-Language",
            HttpHeader::Accept_Ranges => "Accept-Ranges",
            HttpHeader::Age => "Age",
            HttpHeader::Allow => "Allow",
            HttpHeader::Authorization => "Authorization",
            HttpHeader::Cache_Control => "Cache-Control",
            HttpHeader::Connection => "Connection",
            HttpHeader::Content_Disposition => "Content-Disposition",
            HttpHeader::Content_Encoding => "Content-Encoding",
            HttpHeader::Content_Language => "Content-Language",
            HttpHeader::Content_Length => "Content-Length",
            HttpHeader::Content_Location => "Content-Location",
            HttpHeader::Content_Range => "Content-Range",
            HttpHeader::Content_Type => "Content-Type",
            HttpHeader::Cookie => "Cookie",
            HttpHeader::Date => "Date",
            HttpHeader::ETag => "ETag",
            HttpHeader::Expect => "Expect",
            HttpHeader::Expires => "Expires",
            HttpHeader::Forwarded => "Forwarded",
            HttpHeader::From => "From",
            HttpHeader::Host => "Host",
            HttpHeader::If_Match => "If-Match",
            HttpHeader::If_Modified_Since => "If-Modified-Since",
            HttpHeader::If_None_Match => "If-None-Match",
            HttpHeader::If_Range => "If-Range",
            HttpHeader::If_Unmodified_Since => "If-Unmodified-Since",
            HttpHeader::Keep_Alive => "Keep-Alive",
            HttpHeader::Last_Modified => "Last-Modified",
            HttpHeader::Link => "Link",
            HttpHeader::Location => "Location",
            HttpHeader::Max_Forwards => "Max-Forwards",
            HttpHeader::Origin => "Origin",
            HttpHeader::Pragma => "Pragma",
            HttpHeader::Proxy_Authenticate => "Proxy-Authenticate",
            HttpHeader::Proxy_Authorization => "Proxy-Authorization",
            HttpHeader::Range => "Range",
            HttpHeader::Referer => "Referer",
            HttpHeader::Retry_After => "Retry-After",
            HttpHeader::Sec_Fetch_Dest => "Sec-Fetch-Dest",
            HttpHeader::Sec_Fetch_Mode => "Sec-Fetch-Mode",
            HttpHeader::Sec_Fetch_Site => "Sec-Fetch-Site",
            HttpHeader::Sec_Fetch_User => "Sec-Fetch-User",
            HttpHeader::Server => "Server",
            HttpHeader::Set_Cookie => "Set-Cookie",
            HttpHeader::Strict_Transport_Security => "Strict-Transport-Security",
            HttpHeader::TE => "TE",
            HttpHeader::Trailer => "Trailer",
            HttpHeader::Transfer_Encoding => "Transfer-Encoding",
            HttpHeader::Upgrade => "Upgrade",
            HttpHeader::User_Agent => "User-Agent",
            HttpHeader::Vary => "Vary",
            HttpHeader::Via => "Via",
            HttpHeader::Warning => "Warning",
            HttpHeader::WWW_Authenticate => "WWW-Authenticate",
            HttpHeader::X_Content_Type_Options => "X-Content-Type-Options",
            HttpHeader::X_DNS_Prefetch_Control => "X-DNS-Prefetch-Control",
            HttpHeader::X_Frame_Options => "X-Frame-Options",
            HttpHeader::X_Powered_By => "X-Powered-By",
            HttpHeader::X_XSS_Protection => "X-XSS-Protection",
        }
    }
    pub fn from_str(header: &str) -> HttpHeader {
        for header_value in HttpHeader::iter() {
            if header_value.as_str() == header {
                return header_value;
            }
        }
        HttpHeader::Accept
    }
    pub fn get_all_headers() -> Vec<String> {
        HttpHeader::iter()
            .map(|m| m.as_str().to_string())
            .collect()
    }
}
