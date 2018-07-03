// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(existential_type)]

fn main() {
    assert_eq!(foo().to_string(), "foo");
    assert_eq!(bar1().to_string(), "bar1");
    assert_eq!(bar2().to_string(), "bar2");
    let mut x = bar1();
    x = bar2();
    assert_eq!(boo::boo().to_string(), "boo");
    assert_eq!(my_iter(42u8).collect::<Vec<u8>>(), vec![42u8]);
}

// single definition
existential type Foo: std::fmt::Display;

fn foo() -> Foo {
    "foo"
}

// two definitions
existential type Bar: std::fmt::Display;

fn bar1() -> Bar {
    "bar1"
}

fn bar2() -> Bar {
    "bar2"
}

// definition in submodule
existential type Boo: std::fmt::Display;

mod boo {
    pub fn boo() -> super::Boo {
        "boo"
    }
}

existential type MyIter<T>: Iterator<Item = T>;

fn my_iter<T>(t: T) -> MyIter<T> {
    std::iter::once(t)
}

fn my_iter2<T>(t: T) -> MyIter<T> {
    std::iter::once(t)
}

// param names should not have an effect!
fn my_iter3<U>(u: U) -> MyIter<U> {
    std::iter::once(u)
}

// param position should not have an effect!
fn my_iter4<U, V>(_: U, v: V) -> MyIter<V> {
    std::iter::once(v)
}

// param names should not have an effect!
existential type MyOtherIter<T>: Iterator<Item = T>;

fn my_other_iter<U>(u: U) -> MyOtherIter<U> {
    std::iter::once(u)
}

trait Trait {}
existential type GenericBound<T: Trait>: 'static;

fn generic_bound<T: Trait>(_: T) -> GenericBound<T> {
    unimplemented!()
}

mod pass_through {
    pub existential type Passthrough<T>: 'static;

    fn define_passthrough<T: 'static>(t: T) -> Passthrough<T> {
        t
    }
}

fn use_passthrough(x: pass_through::Passthrough<u32>) -> pass_through::Passthrough<u32> {
    x
}

existential type PartiallyDefined<T>: 'static;

// doesn't declare all PartiallyDefined for all possible `T`, but since it's the only
// function producing the value, noone can ever get a value that is problematic
fn partially_defined<T: std::fmt::Debug>(_: T) -> PartiallyDefined<T> {
    4u32
}

existential type PartiallyDefined2<T>: 'static;

fn partially_defined2<T: std::fmt::Debug>(_: T) -> PartiallyDefined2<T> {
    4u32
}

// fully defines PartiallyDefine2
fn partially_defined22<T>(_: T) -> PartiallyDefined2<T> {
    4u32
}
