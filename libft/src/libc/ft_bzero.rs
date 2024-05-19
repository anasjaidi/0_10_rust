// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   ft_bzero.rs                                        :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: ajaidi <ajaidi@student.42.fr>              +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2024/05/19 04:13:16 by ajaidi            #+#    #+#             //
//   Updated: 2024/05/19 04:13:27 by ajaidi           ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

pub fn ft_bzero(ptr: *mut u8, size: usize) -> *mut u8 {
    if ptr.is_null() {
        return ptr;
    }
    let mut i = 0;
    while i < size {
        unsafe {
            *ptr.add(i) = 0;
        }
        i += 1;
    }
    ptr
}
#[cfg(test)]
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
        let data = ptr::null::<u8>() as *mut u8;
        let ptr = ft_bzero(data, 12);
        assert_eq!(ptr, data)
    }

    #[test]
    fn test_raw_data() -> Result<(), Box<dyn Error>> {
        let (data, layout) = allocate_memory(4, 4)?;
        let mut sum = 0;
        let ptr = ft_bzero(data, 4);
        unsafe {
            for i in std::slice::from_raw_parts(ptr, 4) {
                sum += *i;
            }
        }
        assert_eq!(sum, 0);
        unsafe {
            dealloc(ptr, layout);
        }
        Ok(())
    }
}
