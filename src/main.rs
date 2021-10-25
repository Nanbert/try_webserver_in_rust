mod config;
mod webserver;
use clap::{Arg,App};

fn main() {
    //需要修改的数据库信息,登录名,密码,库名
    let user = String::from("root");
    let passwd = String::from("root");
    let databasename = String::from("db");

    //命令行解析
    let args = App::new("tinyRustServer")
                        .version("1.0")
                        .about("a tiny web server in rust")
                        .arg(Arg::with_name("port")
                             .short("p")
                             .long("port")
                             .value_name("PORT NUM")
                             .help("设置端口号,默认为9006")
                             .takes_value(true))
                        .arg(Arg::with_name("logwrite")
                             .short("l")
                             .long("logwrite")
                             .value_name("MODE NUM")
                             .help("日志写入方式,0代表同步写入,1代表异步写入,默认为0")
                             .takes_value(true))
                        .arg(Arg::with_name("trigmode")
                             .short("m")
                             .long("trigmode")
                             .value_name("MODE NUM")
                             .help("listenfd和connfd的模式组合,0代表LT+LT,1代表LT+ET,2代表ET+LT,3代表ET+ET,默认为0")
                             .takes_value(true))
                        .arg(Arg::with_name("opt_linger")
                             .short("o")
                             .long("opt_linger")
                             .value_name("MODE NUM")
                             .help("是否优雅关闭连接,0代表不使用,1代表使用,默认为0")
                             .takes_value(true))
                        .arg(Arg::with_name("sql_num")
                             .short("s")
                             .long("sql_num")
                             .value_name("NUM")
                             .help("设置数据库连接数量,默认为8")
                             .takes_value(true))
                        .arg(Arg::with_name("thread_num")
                             .short("t")
                             .long("thread_num")
                             .value_name("NUM")
                             .help("设置线程数量,默认为8")
                             .takes_value(true))
                        .arg(Arg::with_name("close_log")
                             .short("c")
                             .long("close_log")
                             .value_name("MODE NUM")
                             .help("是否关闭日志,0代表打开日志,1代表关闭日志,默认为0")
                             .takes_value(true))
                        .arg(Arg::with_name("actor_model")
                             .short("a")
                             .long("actor_model")
                             .value_name("MODE NUM")
                             .help("选择反应堆模型,0代表Proactor,1代表Reactor,默认0")
                             .takes_value(true))
                        .get_matches();

    let mut config = config::Config::new();
    config.parse_arg(&args);
    
    println!("{:#?}",config);
    //初始化
    let mut server = webserver::WebServer::new(config.port,&user,&passwd,&databasename,config.logwrite, &config.opt_linger,&config.trigmode,&config.sql_num,&config.thread_num,config.close_log,&config.actor_model);

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
