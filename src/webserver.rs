mod log;
use std::time::Duration;
use std::io::Write;
use std::thread;
use std::sync::{Arc,Mutex};
pub struct WebServer{
    m_port:u32,
    m_close_log:u32,
    m_log_write:u32,
}

impl WebServer{
    pub fn log_write(&mut self){
        if 0==self.m_close_log{
            if 1 == self.m_log_write{
                //此刻初始化一个log变量，用共享变量初始化,传递不同的参数即可
                let mut logObj=log::Log::new("ServerLog",self.m_close_log,2000,800);
                logObj.write_log(2,"HELLO_1");
                logObj.write_log(2,"HELLO_2");
                logObj.write_log(2,"HELLO_3");
                logObj.write_log(2,"HELLO_4");
                logObj.write_log(2,"HELLO_5");
                logObj.write_log(2,"HELLO_6");
                logObj.write_log(2,"HELLO_7");
                logObj.write_log(2,"HELLO_8");
                logObj.write_log(2,"HELLO_9");
                logObj.write_log(2,"HELLO_10");
                logObj.write_log(2,"HELLO_11");
                logObj.write_log(2,"HELLO_12");
                logObj.write_log(2,"HELLO_13");
                logObj.write_log(2,"HELLO_14");
                logObj.write_log(2,"HELLO_15");
                logObj.write_log(2,"HELLO_16");
                logObj.create_handle();
                logObj.m_handler.unwrap().join().unwrap();
            }
            else{
                //同上
                let logObj=log::Log::new("ServerLog",self.m_close_log,2000,0);
            }
        }
    }
    pub fn sql_pool(&self){

    }
    pub fn thread_pool(&self){

    }
    pub fn trig_mode(&self){

    }
    pub fn event_listen(&self){

    }
    pub fn event_loop(&self){

    }
    pub fn new(_port:u32,_user:&String,_pass_word:&String,_database_name:&String,_log_write:u32,_opt_linger:&u32,_trigmode:&u32,_sql_num:&u32,_thread_num:&u32,_close_log:u32,_actor_model:&u32)->WebServer{
        WebServer{
            m_port:_port,
            m_close_log:_close_log,
            m_log_write:_log_write,
        } 
    }
}
