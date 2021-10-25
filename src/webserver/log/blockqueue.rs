use std::collections::VecDeque;
use std::sync::{Condvar,Mutex,Arc,MutexGuard};
pub struct BlockDeque{
    deq_:VecDeque<String>,
    capacity_:usize,
    mtx_:Arc<Mutex<bool>>,
    isclose_:bool,
    condconsumer_:Arc<Condvar>,
    condproducer_:Arc<Condvar>,
}
impl Drop for BlockDeque{
    fn drop(&mut self){
        {
            let guard=self.mtx_.lock().unwrap();
            self.deq_.clear();
            self.isclose_=true;
        }
        self.condproducer_.notify_all();
        self.condconsumer_.notify_all();
        println!("i am quit!");
    }
}
impl BlockDeque{
    pub fn new(capacity:usize)->BlockDeque{
        let tmpdeque=VecDeque::with_capacity(capacity);
        BlockDeque{
            deq_:tmpdeque,
            capacity_:capacity,
            mtx_:Arc::new(Mutex::new(false)),
            isclose_:false,
            condconsumer_:Arc::new(Condvar::new()),
            condproducer_:Arc::new(Condvar::new()),
        }
    }
    pub fn empty(&self)->bool{
        let guard=self.mtx_.lock().unwrap();
        self.deq_.is_empty()
    }
    pub fn full(&self)->bool{
        let guard=self.mtx_.lock().unwrap();
        self.deq_.len()>=self.capacity_
    }
    pub fn clear(&mut self){
        let guard=self.mtx_.lock().unwrap();
        self.deq_.clear();
    }
    pub fn front(&mut self)->Option<&String>{
        let guard=self.mtx_.lock().unwrap();
        self.deq_.front()
    }
    pub fn back(&mut self)->Option<&String>{
        let guard=self.mtx_.lock().unwrap();
        self.deq_.back()
    }
    pub fn size(&self)->usize{
        let guard=self.mtx_.lock().unwrap();
        self.deq_.len()
    }
    pub fn flush(&self){
        self.condconsumer_.notify_one();
    }
    pub fn capacity(&self)->usize{
        let guard=self.mtx_.lock().unwrap();
        self.capacity_
    }
    pub fn push_back(&mut self,item:&str){
        let mut guard = self.mtx_.lock().unwrap();
        while self.deq_.len()>=self.capacity_ {
            guard=self.condproducer_.wait(guard).unwrap();
        }
        self.deq_.push_back(item.to_string());
        println!("push sucess");
        self.condconsumer_.notify_one();
    }
    pub fn push_front(&mut self,item:&str){
        let mut guard = self.mtx_.lock().unwrap();
        while self.deq_.len()>=self.capacity_ {
            guard=self.condproducer_.wait(guard).unwrap();
        }
        self.deq_.push_front(item.to_string());
        self.condconsumer_.notify_one();
    }
    pub fn pop(&mut self,item:&mut String)->bool{
        let mut guard = self.mtx_.lock().unwrap();
        while self.deq_.is_empty(){
            guard=self.condconsumer_.wait(guard).unwrap();
            println!("I am Empty");
            if self.isclose_{
                return false;
            }
        }
        *item=self.deq_.front().unwrap().to_string();
        self.deq_.pop_front();
        self.condproducer_.notify_one();
        println!("pop sucess");
        true
    }
    //定时器的pop留着以后弄
}
