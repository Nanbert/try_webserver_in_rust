use std::thread;
use chrono::{Utc,Datelike,Timelike};
use std::io::Write;
use std::fs::{OpenOptions,File};
use std::sync::{Mutex,Arc};
pub struct Log{
    log_name: String,//log文件名
    m_split_lines: u64,//日志最大行数
    m_count: Arc<Mutex<u64>>,//日志行数记录,LOCK_NEED
    m_today: Arc<Mutex<u32>>,//因为按天分类,记录当前时间是哪一天,LOCK_NEED
    //blockqueue,阻塞队列待定,I am WAITING..........
    m_is_async: bool,
    //locker xx 锁需要吗?????????????????????
    m_close_log: u32,//关闭日志
    m_fp: Arc<Mutex<File>>,//日志文件描述符,LOCK_NEED
}
impl Log{
    //new直接就init
    pub fn new(file_name:&str,close_log:u32,split_lines:u64,max_queue_size:u32)->Log{
        let mut _m_is_async=false;
        if max_queue_size>=1{
            _m_is_async = true;
            //初始化阻塞队列m_log_queue = new block===========
            //handle要不要定义成成员，好在主函数里join，异步写函数======
            let _handle = thread::spawn(|| {

            });
        }
        let now = Utc::now();
        let log_full_name=String::from(format!("{}_{}_{}_{}",now.year().to_string(),now.month().to_string(),now.day().to_string(),file_name));
        //let _m_fp=OpenOptions::new().append(true).create(true).open(log_full_name).expect("创建打开日志文件失败！");
        let _m_fp=match OpenOptions::new().append(true).create(true).open(log_full_name){
            Err(why)=>panic!("无法初始化日志文件:{}",why.to_string()),
            Ok(file)=>file
        };
        Log{
            log_name:file_name.to_string(),
            m_split_lines:split_lines,
            //行数记录为0
            m_count:Arc::new(Mutex::new(0)),
            //天有啥用,为啥单独摘出来
            m_today:Arc::new(Mutex::new(now.day())),
            m_is_async:_m_is_async,
            m_close_log:close_log,
            m_fp:Arc::new(Mutex::new(_m_fp)),
        }
    }
    pub fn write_log(self,level:u32,content:&str){
        let now=Utc::now();
        let s= match level{
            0=>String::from("[debug]:"),
            1=>String::from("[info]:"),
            2=>String::from("[warn]:"),
            3=>String::from("[erro]:"),
            _=>String::from("[info]:"),
        };
        //写入一个log,对m_conut++,m_split_lines最大行数,关键是不止这一个共享变量？？？？？？？？？？？？？
        //self.m_fp.lock().unwrap();
        let mut m_count_pointer=self.m_count.lock().unwrap();
        let mut m_today_pointer=self.m_today.lock().unwrap();
        let mut m_fp_pointer=self.m_fp.lock().unwrap();
        *m_count_pointer+=1;
        if now.day()!=*m_today_pointer || (*m_count_pointer%self.m_split_lines)==0
        {
            //这个还是flush
            (*m_fp_pointer).sync_all().expect("sync_all错误");
            let mut new_log_filename=String::from(format!("{}_{}_{}_{}",now.year().to_string(),now.month().to_string(),now.day().to_string(),self.log_name));
            if *m_today_pointer != now.day()
            {
                *m_today_pointer=now.day();
                *m_count_pointer=0;
            }
            else{
                new_log_filename+=&(*m_count_pointer/self.m_split_lines).to_string();
            }
            *m_fp_pointer = OpenOptions::new().append(true).create(true).open(new_log_filename).expect("创建新日志失败!");
        } 
        let  whole_line=String::from(format!("{}-{}-{} {}:{}:{}.{} {} {}\n",now.year().to_string(),now.month().to_string(),now.day().to_string(),now.hour().to_string(),now.minute(),now.second(),now.nanosecond(),s,content));
        //?????????????判断这有个阻塞队列
        if !self.m_is_async{
            (*m_fp_pointer).write(whole_line.as_bytes()).expect("同步写日志错误!");
        }
    }
}
