mod log;
mod http_conn;
mod thread_pool;
use std::time::Duration;
use std::io::Write;
use std::thread;
use std::sync::{Arc,Mutex};
use mysql::*;
use mysql::prelude::*;
pub struct WebServer{
    m_port:u32,
    m_close_log:u32,
    m_log_write:u32,
    m_user:String,
    m_passWord:String,
    m_dataBaseName:String,
    m_sql_num:usize,
    m_connPool:Option<Pool>,
    logObj:Option<log::Log>,
    users:http_conn::http_conn,
    //线程池相关
    m_pool:Option<thread_pool::thread_pool>,
    m_thread_num:usize,
}

impl WebServer{
    pub fn log_write(&mut self){
        if 0==self.m_close_log{
            if 1 == self.m_log_write{
                //此刻初始化一个log变量，用共享变量初始化,传递不同的参数即可
                self.logObj=Some(log::Log::new("ServerLog",self.m_close_log,2000,800));
                self.logObj.as_mut().unwrap().create_handle();
                self.logObj.as_mut().unwrap().write_log(2,"HELLO_2");
                self.logObj.as_mut().unwrap().write_log(2,"HELLO_3");
                //待建join?????????????
                //self.logObj.as_mut().unwrap().m_handler.unwrap().join().unwrap();
            }
            else{
                //同上
                self.logObj=Some(log::Log::new("ServerLog",self.m_close_log,2000,0));
            }
        }
    }
    pub fn sql_pool(& mut self){
        //初始化数据库链接池
        let opts=OptsBuilder::new()
            .user(Some(&self.m_user))
            .db_name(Some(&self.m_dataBaseName))
            .ip_or_hostname(Some("localhost"))
            .tcp_port(3306)
            .pass(Some("nan"));
        self.m_connPool=Some(Pool::new_manual(self.m_sql_num,self.m_sql_num,opts).unwrap());

        //初始化数据库读取表
        self.users.initmysql_result(self.m_connPool.as_ref().unwrap());

    }
    pub fn thread_pool(&mut self){
        self.m_pool=Some(thread_pool::thread_pool::new(self.m_thread_num));   
    }
    pub fn trig_mode(&self){

    }
    pub fn event_listen(&self){

    }
    pub fn event_loop(&self){

    }
    pub fn new(_port:u32,_user:String,_pass_word:String,_database_name:String,_log_write:u32,_opt_linger:&u32,_trigmode:&u32,_sql_num:usize,_thread_num:usize,_close_log:u32,_actor_model:&u32)->WebServer{
        WebServer{
            m_port:_port,
            m_close_log:_close_log,
            m_log_write:_log_write,
            m_user:_user,
            m_passWord:_pass_word,
            m_connPool:None,
            logObj:None,
            m_sql_num:_sql_num,
            m_dataBaseName:_database_name,
            users:http_conn::http_conn::new(),
            m_thread_num:_thread_num,
            m_pool:None,
        } 
    }
}
