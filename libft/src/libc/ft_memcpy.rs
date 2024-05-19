// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   ft_memcpy.rs                                       :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: ajaidi <ajaidi@student.42.fr>              +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2024/05/19 14:00:16 by ajaidi            #+#    #+#             //
//   Updated: 2024/05/19 14:00:17 by ajaidi           ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

pub fn ft_memcpy(dest: *mut u8, src: *mut u8, size: usize) -> *mut u8 {
    if dest.is_null() || src.is_null() || size == 0 {
        return dest;
    };
    let mut i = 0;
    while i < size {
        unsafe {
            *dest.add(i) = *src.add(i);
        }
        i += 1;
    }
    dest
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use crate::libc::ft_memcpy::ft_memcpy;

    #[test]
    pub fn test_null_dest() {
        assert!(ft_memcpy(ptr::null_mut(), ptr::null_mut(), 0).is_null());
    }

    #[test]
    pub fn test_valid_data() {
        let a = [0i16; 4];
        let b = [4i16; 4];

        let a = ft_memcpy(a.as_ptr() as *mut u8, b.as_ptr() as *mut u8, 8) as *const i16;
        unsafe {
            let a: &[i16] = std::slice::from_raw_parts(a, 4);
            assert!(a == b)
        }
    }
}
