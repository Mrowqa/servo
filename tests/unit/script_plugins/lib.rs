/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub mod unrooted_must_root {
    /**
    ```
    #![feature(plugin)]
    #![plugin(script_plugins)]

    #[derive(Clone, Debug)]  // derive should not be checked
    #[must_root] struct Foo(i32);
    #[must_root] struct Bar(Foo);

    impl Foo {
        fn new() -> Box<Foo> {
            let ret = Box::new(Foo(0));  // Box should be allowed as a binding type within constructors
            ret
        }

        fn new_with(x: i32) -> Foo {
            Foo(x)
        }
    }

    fn foo1(_: &Foo) {}
    fn foo2(_: &()) -> &Foo { unimplemented!() }
    fn new_foo() -> Foo { Foo::new_with(0) }

    fn main() {}
    ```
    */
    pub fn ok() {}

    /**
    ```compile_fail
    #![feature(plugin)]
    #![plugin(script_plugins)]

    #[must_root] struct Foo(i32);
    struct Bar(Foo);

    fn main() {}
    ```
    */
    pub fn struct_field() {}

    /**
    ```compile_fail
    #![feature(plugin)]
    #![plugin(script_plugins)]

    #[must_root] struct Foo(i32);

    fn foo1(_: Foo) {}

    fn main() {}
    ```
    */
    pub fn parameter() {}

    /**
    ```compile_fail
    #![feature(plugin)]
    #![plugin(script_plugins)]

    #[must_root] struct Foo(i32);

    fn foo2() -> Foo { unimplemented!() }

    fn main() {}
    ```
    */
    pub fn return_type() {}

    /**
    ```
    #![feature(plugin)]
    #![plugin(script_plugins)]

    #[must_root] struct Foo([i32; 42]);

    fn foo(x: &Foo) -> i32 {
        let y = Foo(x.0).0;
        y[4]
    }

    fn main() {}
    ```
    */
    pub fn allow_subexpression() {}

    /**
    ```compile_fail
    #![feature(plugin)]
    #![plugin(script_plugins)]

    #[must_root] struct Foo(i32);

    fn foo(x: &Foo) -> i32 {
        let y = Foo(x.0 + 3);
        y.0
    }

    fn main() {}
    ```
    */
    pub fn local_var() {}

    /**
    ```compile_fail
    #![feature(plugin)]
    #![plugin(script_plugins)]

    #[must_root] struct Foo(i32);

    fn foo(x: &Foo) -> i32 {
        let y = Box::new(Foo(x.0 + 3));
        y.0
    }

    fn main() {}
    ```
    */
    pub fn ban_box() {}

    /**
    ```
    #![feature(plugin)]
    #![plugin(script_plugins)]

    #[must_root] struct Foo(i32);

    fn foo(x: &Foo) -> i32 {
        let y = &Box::new(Foo(x.0 + 3));
        y.0
    }

    fn main() {}
    ```
    */
    pub fn allow_box_ref() {}

    /**
    ```compile_fail
    #![feature(plugin)]
    #![plugin(script_plugins)]

    #[must_root] struct Foo(i32);
    #[allow(unrooted_must_root)] struct Bar(Foo);

    fn foo(bar: Bar) -> bool {
        match bar {
            Bar(f @ Foo(_)) => f.0 == 4,
            _ => false,
        }
    }

    fn main() {}
    ```
    */
    pub fn pattern_binding() {}

    // TODO how to test against:
    // match expr.node {
    //     // Trait casts from #[must_root] types are not allowed
    //     hir::ExprCast(ref subexpr, _) => require_rooted(cx, self.in_new_function, &*subexpr),
    // 'as' is for casting between primitive types

    // TODO do we want and how to test against Boxing types that are #[must_root]
}
