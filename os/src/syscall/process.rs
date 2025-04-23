// Process management syscalls
//use core::panic::AssertUnwindSafe;

use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next,TASK_MANAGER,current_user_token,};
use crate::mm::{translate_ptr_user_kernel,translate_timeptr};
use crate::timer::get_time_us;
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    //println!("sys_time");
    let us = get_time_us();
    let (first,second) = translate_timeptr(current_user_token(),ts as usize).unwrap();
    unsafe {
        *(first as *mut usize) = us / 1_000_000;
        *(second as *mut usize) =us % 1_000_000;
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    //println!("sys_trace {:#x}",id);
    match trace_request  {
        0 | 1 => {
            let (ptr,is_readable,is_wirteable) = translate_ptr_user_kernel(current_user_token(),id);
            match trace_request{
                0 => {
                    if is_readable{
                        if let Some(ptr) = ptr{
                            unsafe { (*ptr) as isize }
                        }else{
                            -1
                        }
                    }else{
                        -1
                    }
                },
                1 => {
                    if is_wirteable{
                        if let Some(ptr) = ptr{
                            unsafe{*ptr = data as u8}
                            0 
                        }else{
                            -1
                        }
                    }else{
                        -1
                    }
                },
                _ => unreachable!()
            }
        }
        2 =>{
            match TASK_MANAGER.find_current_task_sysc(id){
             Some(count) => count,
             None => -1,
            }
         }
         _ =>1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    //println!("mmap");
    TASK_MANAGER.mmap(start, len, port)
  
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    //println!("munmap");
    TASK_MANAGER.munmap(start, len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
