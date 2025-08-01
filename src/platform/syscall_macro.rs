macro_rules! syscall {
    ($syscall_no: expr) => {
        {
        let ret: i32;
        unsafe {
            #[cfg(target_arch = "arm")]
            {
                asm!(
                    "svc 0",
                    out("r0") => ret,
                    in("r7") $syscall_no,
                )
            }
            #[cfg(target_arch = "aarch64")]
            {
                asm!(
                    "svc 0",
                    out("x0") => ret,
                    in("w8") $syscall_no,
                )
            }
            #[cfg(target_arch = "x86_64")]
            {
                asm!(
                    "syscall",
                    inout("rax") $syscall_no => ret,
                )
            }
        }
        ret}
    };

    ($syscall_no: expr, $a0: expr) => {
        {
        let ret: i32;
        unsafe {
            #[cfg(target_arch = "arm")]
            {
                asm!(
                    "svc 0",
                    inout("r0") $a0 => ret,
                    in("r7") $syscall_no,
                )
            }
            #[cfg(target_arch = "aarch64")]
            {
                asm!(
                    "svc 0",
                    inout("x0") $a0 => ret,
                    in("w8") $syscall_no,
                )
            }
            #[cfg(target_arch = "x86_64")]
            {
                asm!(
                    "syscall",
                    in("rdi") $a0,
                    inout("rax") $syscall_no => ret,
                )
            }
        }
        ret
        }
    };

    ($syscall_no: expr, $a0: expr, $a1: expr) => {
        {
        let ret: i32;
        unsafe {
            #[cfg(target_arch = "arm")]
            {
                asm!(
                    "svc 0",
                    inout("r0") $a0 => ret,
                    in("r1") $a1,
                    in("r7") $syscall_no,
                )
            }
            #[cfg(target_arch = "aarch64")]
            {
                asm!(
                    "svc 0",
                    inout("x0") $a0 => ret,
                    in("x1") $a1,
                    in("w8") $syscall_no,
                )
            }
            #[cfg(target_arch = "x86_64")]
            {
                asm!(
                    "syscall",
                    in("rdi") $a0,
                    in("rsi") $a1,
                    inout("rax") $syscall_no => ret,
                )
            }
        }
        ret}
    };

    ($syscall_no: expr, $a0: expr, $a1: expr, $a2: expr) => {
        {
        let ret: i32;
        unsafe {
            #[cfg(target_arch = "arm")]
            {
                asm!(
                    "svc 0",
                    inout("r0") $a0 => ret,
                    in("r1") $a1,
                    in("r2") $a2,
                    in("r7") $syscall_no,
                )
            }
            #[cfg(target_arch = "aarch64")]
            {
                asm!(
                    "svc 0",
                    inout("x0") $a0 => ret,
                    in("x1") $a1,
                    in("x2") $a2,
                    in("w8") $syscall_no,
                )
            }
            #[cfg(target_arch = "x86_64")]
            {
                asm!(
                    "syscall",
                    in("rdi") $a0,
                    in("rsi") $a1,
                    in("rdx") $a2,
                    inout("rax") $syscall_no => ret,
                )
            }
        }
        ret}
    };

    ($syscall_no: expr, $a0: expr, $a1: expr, $a2: expr, $a3: expr) => {
        {
        let ret: i32;
        unsafe {
            #[cfg(target_arch = "arm")]
            {
                asm!(
                    "svc 0",
                    inout("r0") $a0 => ret,
                    in("r1") $a1,
                    in("r2") $a2,
                    in("r3") $a3,
                    in("r7") $syscall_no,
                )
            }
            #[cfg(target_arch = "aarch64")]
            {
                asm!(
                    "svc 0",
                    inout("x0") $a0 => ret,
                    in("x1") $a1,
                    in("x2") $a2,
                    in("x3") $a3,
                    in("w8") $syscall_no,
                )
            }
            #[cfg(target_arch = "x86_64")]
            {
                asm!(
                    "syscall",
                    in("rdi") $a0,
                    in("rsi") $a1,
                    in("rdx") $a2,
                    in("r10") $a3,
                    inout("rax") $syscall_no => ret,
                )
            }
        }
        ret
        }
    };

    ($syscall_no: expr, $a0: expr, $a1: expr, $a2: expr, $a3: expr, $a4: expr) => {
        {
            let ret: i32;
        unsafe {
            #[cfg(target_arch = "arm")]
            {
                asm!(
                    "svc 0",
                    inout("r0") $a0 => ret,
                    in("r1") $a1,
                    in("r2") $a2,
                    in("r3") $a3,
                    in("r4") $a4,
                    in("r7") $syscall_no,
                )
            }
            #[cfg(target_arch = "aarch64")]
            {
                asm!(
                    "svc 0",
                    inout("x0") $a0 => ret,
                    in("x1") $a1,
                    in("x2") $a2,
                    in("x3") $a3,
                    in("x4") $a4,
                    in("w8") $syscall_no,
                )
            }
            #[cfg(target_arch = "x86_64")]
            {
                asm!(
                    "syscall",
                    in("rdi") $a0,
                    in("rsi") $a1,
                    in("rdx") $a2,
                    in("r10") $a3,
                    in("r8") $a4,
                    inout("rax") $syscall_no => ret,
                )
            }
        }
        ret}
    };

    ($syscall_no: expr, $a0: expr, $a1: expr, $a2: expr, $a3: expr, $a4: expr, $a5: expr) => {
        {
            let ret: i32;
        unsafe {
            #[cfg(target_arch = "arm")]
            {
                asm!(
                    "svc 0",
                    inout("r0") $a0 => ret,
                    in("r1") $a1,
                    in("r2") $a2,
                    in("r3") $a3,
                    in("r4") $a4,
                    in("r5") $a5,
                    in("r7") $syscall_no,
                )
            }
            #[cfg(target_arch = "aarch64")]
            {
                asm!(
                    "svc 0",
                    inout("x0") $a0 => ret,
                    in("x1") $a1,
                    in("x2") $a2,
                    in("x3") $a3,
                    in("x4") $a4,
                    in("x5") $a5,
                    in("w8") $syscall_no,
                )
            }
            #[cfg(target_arch = "x86_64")]
            {
                asm!(
                    "syscall",
                    in("rdi") $a0,
                    in("rsi") $a1,
                    in("rdx") $a2,
                    in("r10") $a3,
                    in("r8") $a4,
                    in("r9") $a5,
                    inout("rax") $syscall_no => ret,
                )
            }
        }
        ret}
    };
}

pub(crate) use syscall;
