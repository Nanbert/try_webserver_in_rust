mod config;
mod webserver;
use std::env;

fn main() {
    //需要修改的数据库信息,登录名,密码,库名
    let user = String::from("root");
    let passwd = String::from("root");
    let databasename = String::from("db");

    //命令行解析
    let args: Vec<String> = env::args().collect();
    let config = config::Config::new();
    config.parse_arg(&args);
    
    let server = webserver::WebServer::new();

    //初始化
    server.init(&config.port,&user,&passwd,&databasename,&config.logwrite,
                &config.opt_linger,&config.trigmode,&config.sql_num,&config.thread_num,&config.close_log,&config.actor_model);

    //日志
    server.log_write();

    //数据库
    server.sql_pool();

    //线程池
    server.thread_pool();

    //触发模式
    server.trig_mode();

    //监听
    server.event_listen();

    //运行
    server.event_loop();

}
