#[allow(non_camel_case_types)]
use libc;
use socket2::{Socket};
use std::os::unix::io::{AsRawFd};
use std::convert::TryInto;
pub struct Utils{
    m_TIMESLOT:u32,
}
impl Utils{
    pub fn new(_timeSlot:u32)->Utils{
        Utils{
            m_TIMESLOT:_timeSlot,
        }
    }
    //将内核事件表注册读事件,ET模式,选择开启EPOLLONESHOT
    pub fn addfd(&mut self,epollfd:libc::c_int,fd:Socket,one_shot:bool,TRIGMode:u32){
        let mut event=libc::epoll_event{
            events:if 1== TRIGMode{
                (libc::EPOLLIN|libc::EPOLLET|libc::EPOLLRDHUP).try_into().unwrap()
            } else{
                (libc::EPOLLIN|libc::EPOLLRDHUP).try_into().unwrap()
            },
            u64:fd.as_raw_fd() as u64,
        };
        if one_shot{
            event.events |= libc::EPOLLONESHOT as u32;
        }
        unsafe{
            libc::epoll_ctl(epollfd,libc::EPOLL_CTL_ADD,fd.as_raw_fd(),&mut event as *mut libc::epoll_event);
        }
    }
}
