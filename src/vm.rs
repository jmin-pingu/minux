use crate::memlayout;
use crate::{print, println};
use core::cell;

pub fn testing() { 
    println!("Kernel base: 0x{:x}", memlayout::KERNBASE);
    println!("End of vm: 0x{:x}", memlayout::PHYSTOP);
}

// The reference to the struct Run must live as long as the struct
struct Run<'a>{
    run: cell::Cell<&'a Run<'a>>,
}

struct KernMem<'a> {
    freelist: cell::Cell<&'a Run<'a>>,
}

// void
// kinit()
// {
//   initlock(&kmem.lock, "kmem");
//   freerange(end, (void*)PHYSTOP);
// }
// 
// void
// freerange(void *pa_start, void *pa_end)
// {
//   char *p;
//   p = (char*)PGROUNDUP((uint64)pa_start);
//   for(; p + PGSIZE <= (char*)pa_end; p += PGSIZE)
//     kfree(p);
// }
