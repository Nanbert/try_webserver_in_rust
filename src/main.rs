use std::collections::HashMap;
//配置项
const DEFAULT_PORT:u32 = 80;
const MAX_CONTENT_LENGTH:u32 = 250000000;
const MAX_CPU:u32 = 30;

let mut zRoot = String::from("");//网站根目录
let mut zTmpNam = String::from("");//临时文件名
let mut zProtocol = String::from("");//浏览器所用协议
let mut zMethod = String::from("");//必须GET方法
let mut zScript = String::from("");//获取的对象
let mut zRealScript = String::from("");//同上
let mut zHome = String::from("");//内容的主目录
let mut zQueryString = String::from("");//询问字符串
let mut zFile = String::from("");//获取对象的文件名
let mut zDir = String::from("");//zFile所在的目录
let mut zPathInfo = String::from("");//Part of the pathname past the file
let mut zAgent = String::from("");//What type if browser is making this query
let mut zServerName = String::from("");//http:之后的名字
let mut zServerPort = String::from("");//端口号
let mut zCookie = String::from("");//请求中的Cookie
let mut zHttpHost = String::from("");//浏览器的名字
let mut zRealPort = String::from("");//真正的TCP端口
let mut zRemoteAddr = String::from("");//请求的IP
let mut zReferer = String::from("");//Name of the page that refered to us
let mut zAccept = String::from("");//可接受的格式
let mut zAcceptEncoding = String::from("");//gzip or default
let mut zContentLength = String::from("");//Content length reported in the header
let mut zContentType = String::from("");//Content type reported in the header
let mut zQuerySuffix = String::from("");//URL?后的部分
let mut zAuthType = String::from("");//Authorization type(basic or digest)
let mut zAuthArg = String::from("");//Authorization values
let mut zRemoteUser = String::from("");//REMOTE_USER set by authorization module
let mut zlfNoneMatch = String::from("");//The If-None-Match header value
let mut zlfModifiedSince = String::from("");//The IF-Modified-Since header value
let mut nIn = 0; //输入字节数
let mut nOut = 0;//输出字节数
let mut zReplyStatus = String::from("");//返回的状态码
let mut statusSent = false;//True after status line is sent
let mut zLogFile = String::from("");//日志文件
let mut debugFlag = false;//True if being debugged
let mut beginTime = ?//进程开始的时间
let mut closeConnection = false;//True to send Connection:close in reply
let mut nRequest = 0;//处理的请求数
let mut omitLog = false;//如果为真，不写日志
let mut useHttps = false;//是否使用Https
let mut zHttp = String::from("http");//http or https
let mut useTimeout = true;//True to use times
let mut standalone = false;//Run as a standalone server(no inetd)
let mut ipv6Only = false;//Use Ipv6 only
let mut ipv4Only = false;//Use IPv4 only
let mut priorSelf = ?;//Previously report SELF time
let mut priorChild = ?;//Previously report CHILD time
let mut mxAge = 120; //Cache-control max-age
let mut default_path = String::from("/bin:/usr/bin");//默认PATH变量值
let mut zScgi = String::from("");//SCGI环境变量
let mut rangeStart = false;//Start of a Range:request
let mut rangeEnd = false;//End of a Range:request
let mut maxCpu = MAX_CPU;//每个进程的最大cpu时间

let mut cgienv = HashMap::new();
cgienv.insert("CONTENT_LENGTH",zContentLength);
cgienv.insert("AUTH_TYPE",zAuthType);
cgienv.insert("AUTH_CONTENT",zAuthArg);
cgienv.insert("CONTENT_TYPE",zContentType);
cgienv.insert("DOCUMENT_ROOT",zHome);
cgienv.insert("HTTP_ACCEPT",zAccept);
cgienv.insert("HTTP_ACCEPT_ENCODING",zAcceptEncoding);
cgienv.insert("HTTP_COOKIE",zCookie);
cgienv.insert("HTTP_HOST",zHttpHost);
cgienv.insert("HTTP_IF_MODIFIED_SINCE",zlfModifiedSince);
cgienv.insert("HTTP_IF_NONE_MATCH",zlfNoneMatch);
cgienv.insert("HTTP_REFERER",zReferer);
cgienv.insert("HTTP_USER_AGENT",zAgent);
cgienv.insert("PATH",default_path);
cgienv.insert("PATH_INFO",zPathInfo);
cgienv.insert("QUERY_STRING",zQueryString);
cgienv.insert("REMOTE_ADDR",zRemoteAddr);
cgienv.insert("REQUEST_METHOD",zMethod);
cgienv.insert("REQUEST_URI",zScript);
cgienv.insert("REQUEST_USER",zRemoteUser);
cgienv.insert("SCGI",zScgi);
cgienv.insert("SCRIPT_DIRECTORY",zDir);
cgienv.insert("SCRIPT_FILENAME",zFile);
cgienv.insert("SCRIPT_NAME",zRealScript);
cgienv.insert("SERVER_NAME",zServerName);
cgienv.insert("SERVER_PORT",zServerPort);
cgienv.insert("SERVER_PROTOCOL",zProtocol);


fn main() {
    println!("default_port is {}",DEFAULT_PORT);
    println!("default_port is {}",MAX_CONTENT_LENGTH);
    println!("default_port is {}",MAX_CPU);
}
