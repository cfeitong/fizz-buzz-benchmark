#![allow(dead_code)]

mod aiden4;
use std::io::{BufWriter, Write};

fn main() {
    // aiden4::run().unwrap();
    // big_num();
    // trivial_algo();
    just_print_libc();
}

fn just_print_libc() {
    let buf = [b'A'; 64 * 1024];
    loop {
        let mut buf_ptr = buf.as_slice();
        unsafe {
            while !buf_ptr.is_empty() {
                let writen = libc::write(1, buf.as_ptr() as _, buf.len()) as usize;
                buf_ptr = &buf_ptr[writen..];
            }
        }
    }
}

fn just_print() {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    loop {
        stdout.write_all(&[b'a'; 64 * 1024]).unwrap();
    }
}

struct BigUint<const N: usize> {
    buf: [u8; N],
    head: usize,
}

impl<const N: usize> BigUint<N> {
    #[inline(always)]
    fn zero() -> Self {
        let mut buf = [b'0'; N];
        buf[N - 1] = b'\n';
        BigUint { buf, head: N - 2 }
    }

    #[inline(always)]
    fn buf(&self) -> &[u8] {
        &self.buf[self.head..]
    }

    #[inline(always)]
    fn incr(&mut self, mut n: u8) {
        debug_assert!(n < 10);
        let mut cur = N - 2;
        loop {
            let v = (self.buf[cur] as u8 - '0' as u8) + n;
            self.buf[cur] = (v % 10) + '0' as u8;
            if v < 10 {
                break;
            }
            n = 1;
            cur -= 1;
        }
        self.head = std::cmp::min(cur, self.head);
    }
}

#[inline(never)]
fn trivial_algo() {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    let mut num = 1u64;
    const N: usize = 1024 * 64;
    let mut buf = [0u8; N];
    let mut cursor = 0;
    macro_rules! write_u64 {
        ($stdout:expr,$buf:expr,$num:ident,$cursor:ident) => {
            unsafe {
                if $cursor + 64 >= $buf.len() {
                    $stdout.write_all($buf.get_unchecked(..$cursor)).unwrap();
                    cursor = 0;
                }
                cursor += itoap::write_to_ptr(&mut $buf[$cursor], $num);
            }
        };
    }
    macro_rules! write_bin {
        ($stdout:expr,$buf:ident,$bin:expr,$cursor:ident) => {
            unsafe {
                if $cursor + $bin.len() >= $buf.len() {
                    $stdout.write_all($buf.get_unchecked(..$cursor)).unwrap();
                    cursor = 0;
                }
                let src = $bin.as_ptr();
                let dest = $buf.get_unchecked_mut($cursor..).as_mut_ptr();
                std::ptr::copy_nonoverlapping(src, dest, $bin.len());
            }
        };
    }
    loop {
        // loop every 15(lcm(3, 5)) numbers, starting from 1
        write_u64!(&mut stdout, buf, num, cursor); // write 1
        num += 1; // num=2
        write_u64!(&mut stdout, buf, num, cursor); // write 2
        write_bin!(&mut stdout, buf, b"Fizz", cursor); // write 3
        num += 2; // num=4
        write_u64!(&mut stdout, buf, num, cursor); // write 4
        write_bin!(&mut stdout, buf, b"Buzz", cursor); // write 5
        write_bin!(&mut stdout, buf, b"Fizz", cursor); // write 6
        num += 3; // num=7
        write_u64!(&mut stdout, buf, num, cursor); // write 7
        num += 1; // num=8
        write_u64!(&mut stdout, buf, num, cursor); // write 8
        write_bin!(&mut stdout, buf, b"Fizz", cursor); // write 9
        write_bin!(&mut stdout, buf, b"Buzz", cursor); // write 10
        num += 3; // num=11
        write_u64!(&mut stdout, buf, num, cursor); // write 11
        write_bin!(&mut stdout, buf, b"Fizz", cursor); // write 9
        num += 2; // num=13
        write_u64!(&mut stdout, &mut buf, num, cursor); // write 13
        num += 1; // num=14
        write_u64!(&mut stdout, &mut buf, num, cursor); // write 14
        write_bin!(&mut stdout, buf, b"FizzBuzz", cursor); // write 15
        num += 2; // num=16
    }
}

#[inline(never)]
fn big_num() {
    let stdout = std::io::stdout();
    let stdout = stdout.lock();
    let mut stdout = BufWriter::with_capacity(64 * 1024, stdout);
    let mut num = BigUint::<16>::zero();
    num.incr(1); // num=1
    loop {
        // loop every 15(lcm(3, 5)) numbers, starting from 1
        write_buf(&mut stdout, num.buf()); // write 1
        num.incr(1); // num=2
        write_buf(&mut stdout, num.buf()); // write 2
        write_fizz(&mut stdout); // write 3
        num.incr(2); // num=4
        write_buf(&mut stdout, num.buf()); // write 4
        write_buzz(&mut stdout); // write 5
        write_fizz(&mut stdout); // write 6
        num.incr(3); // num=7
        write_buf(&mut stdout, num.buf()); // write 7
        num.incr(1); // num=8
        write_buf(&mut stdout, num.buf()); // write 8
        write_fizz(&mut stdout); // write 9
        write_buzz(&mut stdout); // write 10
        num.incr(3); // num=11
        write_buf(&mut stdout, num.buf()); // write 11
        write_fizz(&mut stdout); // write 9
        num.incr(2); // num=13
        write_buf(&mut stdout, num.buf()); // write 13
        num.incr(1); // num=14
        write_buf(&mut stdout, num.buf()); // write 14
        write_fizzbuzz(&mut stdout); // write 15
        num.incr(2); // num=16
    }
}

#[inline(always)]
fn write_fizzbuzz(mut w: impl Write) {
    w.write_all("FizzBuzz\n".as_bytes()).unwrap();
}

#[inline(always)]
fn write_fizz(mut w: impl Write) {
    w.write_all("Fizz\n".as_bytes()).unwrap();
}

#[inline(always)]
fn write_buzz(mut w: impl Write) {
    w.write_all("Buzz\n".as_bytes()).unwrap();
}

#[inline(always)]
fn write_u64(mut w: impl Write, buf: &mut [u8], n: u64) {
    let len = unsafe { itoap::write_to_ptr(buf.as_mut_ptr(), n) };
    buf[len] = b'\n';
    w.write_all(&buf[..len + 1]).unwrap();
}

#[inline(always)]
fn write_buf(mut w: impl Write, buf: &[u8]) {
    w.write_all(buf).unwrap()
}
