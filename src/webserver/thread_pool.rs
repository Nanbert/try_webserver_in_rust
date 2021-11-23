use std::collections::VecDeque;
use std::sync::{Condvar,Mutex,Arc,MutexGuard};
use std::thread;
type func = Box<Fn() + Send + Sync >;
pub struct thread_pool{
    cond:Arc<Condvar>,
    isClosed:bool,
    tasks:Arc<Mutex<VecDeque<func>>>,
}
impl thread_pool{
    pub fn new(threadCount:usize)->thread_pool{
        assert!(threadCount>0);
        let tasks : Arc<Mutex<VecDeque<func>>> = Arc::new(Mutex::new(VecDeque::new()));
        let isClosed=false;
        let cond=Arc::new(Condvar::new());
        for i in (0 .. threadCount){
            let tasks = Arc::clone(&tasks);
            let cond=Arc::clone(&cond);
            let handle = thread::spawn(move||{
                while true{
                    //这锁怎么加阿！！！！！！
                    let mut taskQueue=tasks.lock().unwrap();
                    if(!taskQueue.is_empty()){
                        let task = taskQueue.pop_front().unwrap();
                        drop(taskQueue);
                        task();
                        let taskQueue=tasks.lock().unwrap();
                    }
                    else if isClosed{
                        break;
                    }
                    else {
                        cond.wait(taskQueue);
                    }
                }
            });
        }
        thread_pool{
            cond:cond,
            isClosed:isClosed,
            tasks:tasks,
        }
    }
    pub fn AddTask(&mut self,f:func){
        {
            self.tasks.lock().unwrap().push_back(f);
        }
        self.cond.notify_one()
    }
}
impl Drop for thread_pool{
    fn drop(&mut self){
        {
            let guard=self.tasks.lock().unwrap();
            self.isClosed=true;
        }
        self.cond.notify_all();
    }
}
