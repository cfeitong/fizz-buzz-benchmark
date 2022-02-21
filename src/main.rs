#![allow(dead_code)]

mod aiden4;
use std::io::{BufWriter, Write};

fn main() {
    // aiden4::run().unwrap();
    baseline();
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
fn baseline() {
    let stdout = std::io::stdout();
    let stdout = stdout.lock();
    let mut stdout = BufWriter::with_capacity(64 * 1024, stdout);
    let mut num = BigUint::<32>::zero();
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
fn write_buf(mut w: impl Write, buf: &[u8]) {
    w.write_all(buf).unwrap()
}
