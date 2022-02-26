# Experiment cannot borrow mutable error 0502

With line 26 enabled I get error E0502 and I don't understand why it happens:
```
wink@3900x:~/prgs/rust/myrepos/expr-cannot-borrow-mutable-e0502 (main)
$ cat -n src/main.rs 
     1	struct Struct {
     2	    nums: Vec<u32>,
     3	    sums: Vec<u32>,
     4	}
     5	
     6	impl Struct {
     7	    fn new() -> Struct {
     8	        Struct {
     9	            nums: Vec::new(),
    10	            sums: Vec::new(),
    11	        }
    12	    }
    13	
    14	    fn push_num(&mut self, v: u32) {
    15	        self.nums.push(v);
    16	    }
    17	
    18	    fn push_sum(&mut self, v: u32) {
    19	        self.sums.push(v);
    20	    }
    21	
    22	    fn process(&mut self) {
    23	        let mut sum = 0u32;
    24	        for i in self.nums.iter() {
    25	            sum += i;
    26	            self.push_sum(sum); // Uncommenting this causes compile error E0502
    27	            //self.sums.push(sum);
    28	        }
    29	    }
    30	}
    31	
    32	fn main() {
    33	    let mut v = Struct::new();
    34	    v.push_num(1);
    35	    v.push_num(2);
    36	    v.push_sum(0);
    37	    v.process();
    38	
    39	    assert_eq!(v.nums.len(), 2);
    40	    assert_eq!(v.nums[0], 1);
    41	    assert_eq!(v.nums[1], 2);
    42	    assert_eq!(v.sums.len(), 3);
    43	    assert_eq!(v.sums[0], 0);
    44	    assert_eq!(v.sums[1], 1);
    45	    assert_eq!(v.sums[2], 3);
    46	}

wink@3900x:~/prgs/rust/myrepos/expr-cannot-borrow-mutable-e0502 (main)
$ cargo check
    Checking expr-cannot-borrow-mutable-e0502 v0.1.0 (/home/wink/prgs/rust/myrepos/expr-cannot-borrow-mutable-e0502)
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
  --> src/main.rs:26:13
   |
24 |         for i in self.nums.iter() {
   |                  ----------------
   |                  |
   |                  immutable borrow occurs here
   |                  immutable borrow later used here
25 |             sum += i;
26 |             self.push_sum(sum); // Uncommenting this causes compile error E0502
   |             ^^^^^^^^^^^^^^^^^^ mutable borrow occurs here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `expr-cannot-borrow-mutable-e0502` due to previous error
```

If I comment out line 26 and enable line 27 all is well:
```
wink@3900x:~/prgs/rust/myrepos/expr-cannot-borrow-mutable-e0502 (main)
$ cat -n src/main.rs 
     1	struct Struct {
     2	    nums: Vec<u32>,
     3	    sums: Vec<u32>,
     4	}
     5	
     6	impl Struct {
     7	    fn new() -> Struct {
     8	        Struct {
     9	            nums: Vec::new(),
    10	            sums: Vec::new(),
    11	        }
    12	    }
    13	
    14	    fn push_num(&mut self, v: u32) {
    15	        self.nums.push(v);
    16	    }
    17	
    18	    fn push_sum(&mut self, v: u32) {
    19	        self.sums.push(v);
    20	    }
    21	
    22	    fn process(&mut self) {
    23	        let mut sum = 0u32;
    24	        for i in self.nums.iter() {
    25	            sum += i;
    26	            //self.push_sum(sum); // Uncommenting this causes compile error E0502
    27	            self.sums.push(sum);
    28	        }
    29	    }
    30	}
    31	
    32	fn main() {
    33	    let mut v = Struct::new();
    34	    v.push_num(1);
    35	    v.push_num(2);
    36	    v.push_sum(0);
    37	    v.process();
    38	
    39	    assert_eq!(v.nums.len(), 2);
    40	    assert_eq!(v.nums[0], 1);
    41	    assert_eq!(v.nums[1], 2);
    42	    assert_eq!(v.sums.len(), 3);
    43	    assert_eq!(v.sums[0], 0);
    44	    assert_eq!(v.sums[1], 1);
    45	    assert_eq!(v.sums[2], 3);
    46	}
wink@3900x:~/prgs/rust/myrepos/expr-cannot-borrow-mutable-e0502 (main)
$ cargo run
   Compiling expr-cannot-borrow-mutable-e0502 v0.1.0 (/home/wink/prgs/rust/myrepos/expr-cannot-borrow-mutable-e0502)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/expr-cannot-borrow-mutable-e0502`
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
