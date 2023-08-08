use std::{
    path::Path,
    fs::File,
    io::{
        BufReader,
        BufRead,
        Result,
    },
    collections::HashSet,
    ops::RangeInclusive
};

fn is_range_contain(left: RangeInclusive<i32>, right: RangeInclusive<i32>) -> bool {
    let mut set_left = HashSet::new();
    left.into_iter().for_each(|i| { set_left.insert(i); });

    let mut set_right = HashSet::new();
    right.into_iter().for_each(|i| { set_right.insert(i); });

    // set_left.is_superset(&set_right) || set_right.is_superset(&set_left)
    set_left.intersection(&set_right).count() > 0   //is_superset(&set_right) || set_right.is_superset(&set_left)
}

fn string_to_range(str: &str) -> RangeInclusive<i32> {
    let split: Vec<&str> = str.split("-").collect();
    RangeInclusive::new(split[0].parse::<i32>().unwrap(), split[1].parse::<i32>().unwrap())
}


fn main() -> Result<()> { 
    let path = Path::new("./bin/day4/input");
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let mut superset_count = 0;
    for l in lines {
        let line = l.unwrap();

        let split: Vec<&str> = line.split(",").collect();
        if let [left, right] = split.as_slice() {
            let left_range = string_to_range(left);
            let right_range = string_to_range(right);

            if is_range_contain(left_range, right_range) {
                superset_count += 1;
            }
        }
    }

    println!("{}", superset_count);

    Ok(())
}



// #![no_std]
// #![no_main]
//
// extern crate libc;
//
// #[no_mangle]
// pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
// 	// Similar to previous version, but unneccessary:
// 	// unsafe { libc::exit(42) }
//
//     42
// }
//
// #[panic_handler]
// fn my_panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }


// #![no_std]
// #![no_main]
//
// use core::arch::asm;
//
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     unsafe {
//         asm!(
//             "mov edi, 42",
//             "mov eax, 60",
//             "syscall",
//             options(nostack, noreturn)
//         )
//         // nostack prevents `asm!` from push/pop rax
//         // noreturn prevents it putting a 'ret' at the end
//         //  but it does put a ud2 (undefined instruction) instead
//     }
// }
//
// #[panic_handler]
// fn my_panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }
