use std::thread;
use chrono::{Utc,Datelike};
//use std::io::Write;
use std::fs::OpenOptions;
pub struct Log{
    log_name: String,//log文件名
    m_split_lines: u32,//日志最大行数
    m_log_buf_size: usize,//日志缓冲区大小
    m_count: u64,//日志行数记录
    m_today: u32,//因为按天分类,记录当前时间是哪一天
    m_buf: String,//存储写入日志分配好空间的字符串
    //blockqueue,阻塞队列待定,I am WAITING..........
    m_is_async: bool,
    //locker xx 锁需要吗?????????????????????
    m_close_log: u32,//关闭日志
    m_fp: std::fs::File,//日志文件描述符
}
impl Log{
    //new直接就init
    pub fn new(file_name:&str,close_log:u32,log_buf_size:usize,split_lines:u32,max_queue_size:u32)->Log{
        let mut _m_is_async=false;
        if max_queue_size>=1{
            _m_is_async = true;
            //初始化阻塞队列m_log_queue = new block===========
            //handle要不要定义成成员，好在主函数里join，异步写函数======
            let _handle = thread::spawn(|| {

            });
        }
        //不需要这么原始m_log_buf_size=log_buf_size;
        let _m_buf=String::with_capacity(log_buf_size);
        let now = Utc::now();
        let log_full_name=String::from(format!("{}_{}_{}_{}",now.year().to_string(),now.month().to_string(),now.day().to_string(),file_name));
        let _m_fp=OpenOptions::new().append(true).create(true).open(log_full_name).expect("创建打开日志文件失败！");
        Log{
            log_name:file_name.to_string(),
            m_split_lines:split_lines,
            m_log_buf_size:log_buf_size,
            //行数记录为0
            m_count:0,
            //天有啥用,为啥单独摘出来
            m_today:now.day(),
            m_buf:_m_buf,
            m_is_async:_m_is_async,
            m_close_log:close_log,
            m_fp:_m_fp,
        }
    }
}
