pub struct Config{

    //端口号
    pub port:i32,

    //日志写入方式
    pub logwrite:i32,

    //触发组合模式
    pub trigmode:i32,

    //listenfd触发模式
    pub listentrigmode:i32,

    //connfd触发模式
    pub conntrigmode:i32,

    //优雅关闭链接
    pub opt_linger:i32,

    //数据库连接池数量
    pub sql_num:i32,

    //线程池内的线程数量
    pub thread_num:i32,

    //是否关闭日志
    pub close_log:i32,

    //并发模型选择
    pub actor_model:i32,
}
impl Config{
    pub fn parse_arg(&self,args: &Vec<String>){
        for i in args{
            println!("{}",i);
        }
    }
    pub fn new() ->Config{
        Config{
            port:0,
            logwrite:0,
            trigmode:0,
            listentrigmode:0,
            conntrigmode:0,
            opt_linger:0,
            sql_num:0,
            thread_num:0,
            close_log:0,
            actor_model:0,
        }
    }
}
