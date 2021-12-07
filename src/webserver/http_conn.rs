use mysql::*;
use mysql::prelude::*;
use std::collections::HashMap;
use once_cell::sync::OnceCell;
use mio::Poll;
pub static M_EPOLLFD :OnceCell<&Poll>=OnceCell::new();
pub struct http_conn{

}
pub struct useerRecord{
    name:String,
    pass:String,
}
impl http_conn{
    pub fn initmysql_result(&self,connPool:&Pool){
        //取个连接
        let mut conn = connPool.get_conn().unwrap();
        //
        let results=conn
            .query_map(
            "SELECT username,passwd FROM user",
            |(name,pass)|{
                useerRecord {name,pass}
            },).expect("sql select error!");
        let mut users=HashMap::new();
        for each in results{
            users.insert(each.name,each.pass);
        }
        for (key,value) in &users{
            println!("Username:{},Passwd:{}",key,value);
        }
    }
    pub fn new()->http_conn{
        http_conn{

        }
    }
}
