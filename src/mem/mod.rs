use std::ptr;

pub mod nts;

pub fn follow_offsets<T>(address: usize, offsets: impl IntoIterator<Item = usize>) -> *mut T{


    let mut my_ptr = ptr::from_exposed_addr_mut::<usize>(address);
    for offset in offsets {
        unsafe{
          my_ptr = ptr::from_exposed_addr_mut(my_ptr.read_unaligned() + offset);
        }
    }
    my_ptr as *mut T
}

