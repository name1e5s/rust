//! Check that symbol names with pattern types in them are
//! different from the same symbol with the base type

//@ compile-flags: -Csymbol-mangling-version=v0 -Copt-level=0 --crate-type=lib

#![feature(pattern_types)]
#![feature(core_pattern_types)]
#![feature(core_pattern_type)]

use std::pat::pattern_type;

type NanoU32 = crate::pattern_type!(u32 is 0..=999_999_999);

fn foo<T>() {}

pub fn bar() {
    // CHECK: call pattern_type_symbols::foo::<u32>
    // CHECK: call void @_RINvCs3QvG2ESzx2Q_20pattern_type_symbols3foomEB2_
    foo::<u32>();
    // CHECK: call pattern_type_symbols::foo::<(u32, [(); 0], [(); 999999999], [(); true])>
    // CHECK: call void @_RINvCs3QvG2ESzx2Q_20pattern_type_symbols3fooTmAum0_Aum3b9ac9ff_Aub1_EEB2_
    foo::<NanoU32>();
}
