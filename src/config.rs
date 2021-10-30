use clap::{value_t};

#[derive(Debug)]
pub struct Config{

    //端口号
    pub port:u32,

    //日志写入方式
    pub logwrite:u32,

    //触发组合模式
    pub trigmode:u32,

    //listenfd触发模式
    pub listentrigmode:u32,

    //connfd触发模式
    pub conntrigmode:u32,

    //优雅关闭链接
    pub opt_linger:u32,

    //数据库连接池数量
    pub sql_num:usize,

    //线程池内的线程数量
    pub thread_num:u32,

    //是否关闭日志
    pub close_log:u32,

    //并发模型选择
    pub actor_model:u32,
}
impl Config{
    pub fn parse_arg(&mut self,args: &clap::ArgMatches){
        //传入端口号
        self.port = value_t!(args,"port",u32).unwrap_or(9006);
        //传入日志写入方式
        self.logwrite = value_t!(args,"logwrite",u32).unwrap_or(0);
        //传入套接字模式组合
        self.trigmode = value_t!(args,"trigmode",u32).unwrap_or(0);
        //传入是否优雅关闭连接
        self.opt_linger = value_t!(args,"opt_linger",u32).unwrap_or(0);
        //传入数据库连接数量
        self.sql_num = value_t!(args,"sql_num",usize).unwrap_or(8);
        //传入线程数量
        self.thread_num = value_t!(args,"thread_num",u32).unwrap_or(8);
        //传入是否关闭日志
        self.close_log = value_t!(args,"close_log",u32).unwrap_or(0);
        //传入是否关闭日志
        self.actor_model = value_t!(args,"actor_model",u32).unwrap_or(0);
    }
    pub fn new() ->Config{
        Config{
            //端口号默认9006
            port:9006,

            //日志写入方式,默认同步
            logwrite:0,

            //触发组合模式，默认listenfd LT + connfd LT
            trigmode:0,

            //listenfd触发模式,默认LT
            listentrigmode:0,

            //connfd触发模式，默认LT
            conntrigmode:0,

            //优雅关闭链接，默认不显示
            opt_linger:0,

            //数据库连接词数量,默认8
            sql_num:8,

            //线程池内的线程数量，默认8
            thread_num:8,

            //关闭日志,默认不关闭
            close_log:0,

            //并发模型,默认是proactor
            actor_model:0,
        }
    }
}
