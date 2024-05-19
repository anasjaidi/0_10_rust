pub fn ft_memset(ptr: *mut u8, c: i32, size: usize) -> *mut u8 {
    if ptr.is_null() {
        return ptr;
    }
    let mut i = 0;
    while i < size {
        unsafe {
            *ptr.add(i) = c as u8;
        }
        i += 1;
    }
    ptr
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use std::{
        alloc::{self, dealloc, Layout},
        error::Error,
        ptr,
    };

    use super::*;
    fn allocate_memory(size: usize, al: usize) -> Result<(*mut u8, Layout), Box<dyn Error>> {
        let layout = Layout::from_size_align(size, al)?;
        unsafe { Ok((alloc::alloc(layout), layout)) }
    }
    fn deallocate_memory(ptr: *mut u8, layout: Layout) {
        unsafe { dealloc(ptr, layout) }
    }

    #[test]
    fn test_null_ptr() {
        assert_eq!(
            ft_memset(ptr::null_mut(), 0, std::mem::size_of::<i32>()),
            ptr::null_mut()
        )
    }

    #[test]
    fn test_real_data() {
        let num = 12i32;
        let num_ptr = &num as *const i32 as *mut i32 as *mut u8;
        unsafe {
            assert_eq!(
                *(ft_memset(num_ptr, 0, std::mem::size_of::<i32>()) as *mut i32),
                0i32
            )
        }
    }
}
