extern crate num;
//복합, 유리수, 범위 반복자, 일반 정수 등을 포함한 Rust의 숫자 유형 및 특성 모음
use num::bigint::BigInt;

#[derive(Debug)]
pub struct Point {
    pub x: BigInt,
    pub y: BigInt,
    pub fp: BigInt, //주요 연산은 이값을사용
}
impl Point {
    pub fn double(&self) -> Point {
        let x = self;
        let y = self;
        Point {
            x: self.fp.clone(),
            y: self.fp.clone(),
            fp: self.fp.clone(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
