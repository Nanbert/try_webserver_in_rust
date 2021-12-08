use socket2::{Socket};
use nix::sys::signal::{SaFlags,Signal,SigAction,
SigHandler,sigaction,SigSet};
use std::os::unix::io::{AsRawFd};
use std::convert::TryInto;
use once_cell::sync::OnceCell;
use mio::unix::pipe::{self,Receiver,Sender};
use mio::{Events,Poll,Interest,Token};
use std::io::{Write,Read};

//pub static EPOLLFD :OnceCell<Poll>=OnceCell::new();
//pub static SENDER:OnceCell<&mut Sender>=OnceCell::new();
//pub static RECEIVER:OnceCell<Receiver>=OnceCell::new();
pub static mut SENDER:OnceCell<& Sender>=OnceCell::new();
pub static EPOLLFD :OnceCell<& Poll>=OnceCell::new();

pub extern "C" fn sighandler(sig:i32){
    unsafe{
        //这里switch数字,转成对应的bytes，待建.....
        SENDER.get_mut().unwrap().write(b"hello").unwrap();  
}}

pub struct Utils{
    m_TIMESLOT:u32,
}
impl Utils{
    pub fn new(_timeSlot:u32)->Utils{
        Utils{
            m_TIMESLOT:_timeSlot,
        }
    }
    //设置信号函数
    pub fn addsig(&self,sig: Signal,handler:SigHandler,restart:bool){
        let mut _saFlag=SaFlags::empty();
        if restart{
            _saFlag|=SaFlags::SA_RESTART;
        }
        //等价于sigfillset函数
        let _sigSet=SigSet::all();
        let sa=SigAction::new(handler,_saFlag,_sigSet);
        unsafe{
            sigaction(sig,&sa).unwrap();
        }
    }
}
