use std::thread;
use chrono::{Utc,Datelike,Timelike};
use std::io::Write;
use std::fs::{OpenOptions,File};
use std::sync::{Mutex,Arc};
mod blockqueue;
pub struct Log{
    log_name: String,//log文件名
    m_split_lines: u64,//日志最大行数
    m_count: Arc<Mutex<u64>>,//日志行数记录,LOCK_NEED
    m_today: Arc<Mutex<u32>>,//因为按天分类,记录当前时间是哪一天,LOCK_NEED
    pub m_deque:Arc<Mutex<blockqueue::BlockDeque>>,
    m_is_async: bool,
    //locker xx 锁需要吗?????????????????????
    m_close_log: u32,//关闭日志
    pub m_fp: Arc<Mutex<File>>,//日志文件描述符,LOCK_NEED
    pub m_handler:Option<thread::JoinHandle<()>>,
}
impl Log{
    //new直接就init
    pub fn new(file_name:&str,close_log:u32,split_lines:u64,max_queue_size:usize)->Log{
        let mut _m_is_async=false;
        if max_queue_size>=1{
            _m_is_async = true;
            //初始化阻塞队列m_log_queue = new block===========
            //handle要不要定义成成员，好在主函数里join，异步写函数======
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
            m_deque:Arc::new(Mutex::new(blockqueue::BlockDeque::new(max_queue_size))),
            //天有啥用,为啥单独摘出来
            m_today:Arc::new(Mutex::new(now.day())),
            m_is_async:_m_is_async,
            m_close_log:close_log,
            m_fp:Arc::new(Mutex::new(_m_fp)),
            m_handler: None,
        }
    }
    pub fn create_handle(&mut self){
        //在这里建异步写进程
        let tmpfp=Arc::clone(&self.m_fp);
        let tmpdeque=Arc::clone(&self.m_deque);
        self.m_handler=Some(thread::spawn(move ||{
            let mut item=String::from("");
            //let tmpdeque_pointer=tmpdeque.lock().unwrap();
       //     println!("what the outspawn fuck");
            while tmpdeque.lock().unwrap().pop(&mut item)
            {
        //        println!("thread is looping");
                let mut fp=tmpfp.lock().unwrap();
                (*fp).write(item.as_bytes());
            }
        }));
    }
    pub fn write_log(&mut self,level:u32,content:&str){
        let now=Utc::now();
        let s= match level{
            0=>String::from("[debug]:"),
            1=>String::from("[info]:"),
            2=>String::from("[warn]:"),
            3=>String::from("[erro]:"),
            _=>String::from("[info]:"),
        };

        let mut m_count_pointer=self.m_count.lock().unwrap();
        let mut m_today_pointer=self.m_today.lock().unwrap();
        let mut m_fp_pointer=self.m_fp.lock().unwrap();
        let mut m_deque_pointer=self.m_deque.lock().unwrap();
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
        if self.m_is_async && !m_deque_pointer.full(){
            m_deque_pointer.push_back(&whole_line);
        }
        else{
            (*m_fp_pointer).write(whole_line.as_bytes()).expect("同步写日志错误!");
        }
    }
}
