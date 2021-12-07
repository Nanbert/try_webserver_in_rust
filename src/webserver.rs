mod log;
mod http_conn;
mod thread_pool;
mod timer;
use std::net::{SocketAddr,IpAddr,Ipv4Addr};
use socket2::{Socket,Domain,Type};
use std::time::Duration;
use mio::{Events,Poll,Interest,Token};
use mio::net::{TcpStream,TcpListener};
use mio::unix::pipe::{self,Receiver,Sender};
use mysql::*;
use core::borrow::Borrow;
use once_cell::sync::OnceCell;
pub static M_EPOLLFD :OnceCell<Poll>=OnceCell::new();

const MAX_FD:u32=65536;
const MAX_EVENT_NUMBER:usize=10000;
const TIMESLOT:u32=5;
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
    m_CONNTrigmode:u32,
    m_LISTENTrigmode:u32,
    m_TRIGMode:u32,
    m_listenfd:TcpListener,
    m_pipefd:(Sender,Receiver),
    utils:timer::Utils,
    m_OPT_LINGER:u32,
    events: Events,
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
    pub fn trig_mode(&mut self){
        //LT+LT
        if 0==self.m_TRIGMode{
            self.m_LISTENTrigmode=0;
            self.m_CONNTrigmode=0;
        }
        //LT+ET
        else if 1==self.m_TRIGMode{
            self.m_LISTENTrigmode=0;
            self.m_CONNTrigmode=1;
        }
        //ET+LT
        else if 2==self.m_TRIGMode{
            self.m_LISTENTrigmode=1;
            self.m_CONNTrigmode=0;
        }
        //ET+ET
        else if 3==self.m_TRIGMode{
            self.m_LISTENTrigmode=1;
            self.m_CONNTrigmode=1;
        }
    }
    pub fn event_listen(&mut self ){
       //let mut listenStream=TcpStream::connect(self.m_listenfd.as_ref().unwrap().local_addr().unwrap()).unwrap();
        //epoll创建内核事件表
        const LISTENTOKEN:Token = Token(0);
        //mio默认ET模式,可读等价于EPOLLET|EPOLLIN|EPOLLRDHUP,可写等价于EPOLLET|EPOLLOUT,EPOLLONESHOT不可配置
        M_EPOLLFD.set(Poll::new().unwrap()).unwrap();
        M_EPOLLFD.get().unwrap().registry().register(&mut self.m_listenfd,LISTENTOKEN,Interest::READABLE).unwrap();
        //静态变量初始化,这里借用还是复制好阿？？？？？
        http_conn::M_EPOLLFD.set(M_EPOLLFD.get().unwrap().borrow()).unwrap();
        const PIPE_RECV: Token = Token(0);
        const PIPE_SEND: Token = Token(1);

        M_EPOLLFD.get().unwrap().registry().register(&mut self.m_pipefd.0,PIPE_RECV,Interest::READABLE).unwrap();
        self.m_pipefd.0.set_nonblocking(true).unwrap();
        self.m_pipefd.1.set_nonblocking(true).unwrap();
    }
    pub fn event_loop(&self){

    }
    pub fn new(_port:u32,_user:String,_pass_word:String,_database_name:String,_log_write:u32,_opt_linger:u32,_trigmode:u32,_sql_num:usize,_thread_num:usize,_close_log:u32,_actor_model:&u32)->WebServer{

        let m_listenfd=Socket::new(Domain::IPV4,Type::STREAM,None).unwrap();
        if 0==_opt_linger{
            m_listenfd.set_linger(None).unwrap();
        }
        else if 1==_opt_linger{
            m_listenfd.set_linger(Some(Duration::new(1,0))).unwrap();
        }
        m_listenfd.set_reuse_address(true).unwrap();
        let address=SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)),8080);
        let address=address.into();
        //Warning: 大小端的问题,htonl()需要考虑吗?
        m_listenfd.bind(&address).unwrap();
        //直接设置非阻塞
        m_listenfd.set_nonblocking(true).unwrap();

        m_listenfd.listen(5).unwrap();
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
            m_CONNTrigmode:0,
            m_LISTENTrigmode:0,
            m_TRIGMode:_trigmode,
            utils:timer::Utils::new(TIMESLOT),
            m_OPT_LINGER:_opt_linger,
            m_listenfd:TcpListener::from_std(m_listenfd.into()),
            events:Events::with_capacity(MAX_EVENT_NUMBER),
            m_pipefd:pipe::new().unwrap(),
        } 
    }
}
