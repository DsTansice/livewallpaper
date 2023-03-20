//windows api封装
use std::cell::RefCell;

use winsafe::{co::WS, prelude::*};
use winsafe::{EnumWindows, HWND, WINDOWINFO};

pub fn find_window_handle(pid: u32) -> HWND {
    let res: RefCell<HWND> = RefCell::new(HWND::NULL);

    EnumWindows(|hwnd: HWND| -> bool {
        let text = hwnd.GetWindowText().unwrap();
        let mut info = WINDOWINFO::default();
        hwnd.GetWindowInfo(&mut info).unwrap();

        if !text.is_empty() && (info.dwStyle & WS::VISIBLE != WS::NoValue) {
            let (_, _pid) = hwnd.GetWindowThreadProcessId();
            println!("title:{},_pid:{},hwnd:{}", text, _pid, hwnd);
            if pid == _pid {
                *res.borrow_mut() = hwnd;
            }
        }
        true
    })
    .unwrap();

    res.into_inner()
}

// pub fn find_window(lpClassName: String) -> HWND {
// unsafe {
//     let res = FindWindowA(s!("Progman"), None);
//     println!("find_window: {}", res.0);
//     res
// }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_window_handle() {
        let res = find_window_handle(0);
        println!("test_get_window_handle: {:?}", res)
    }

    #[test]
    fn test_find_window() {
        // find_window("Progman".to_string());
        // print!("test_find_window")
    }
}
