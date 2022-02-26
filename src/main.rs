struct Struct {
    nums: Vec<u32>,
    sums: Vec<u32>,
}

impl Struct {
    fn new() -> Struct {
        Struct {
            nums: Vec::new(),
            sums: Vec::new(),
        }
    }

    fn push_num(&mut self, v: u32) {
        self.nums.push(v);
    }

    fn push_sum(&mut self, v: u32) {
        self.sums.push(v);
    }

    fn process(&mut self) {
        let mut sum = 0u32;
        for i in self.nums.iter() {
            sum += i;
            self.push_sum(sum); // Uncommenting this causes compile error E0502
            //self.sums.push(sum);
        }
    }
}

fn main() {
    let mut v = Struct::new();
    v.push_num(1);
    v.push_num(2);
    v.push_sum(0);
    v.process();

    assert_eq!(v.nums.len(), 2);
    assert_eq!(v.nums[0], 1);
    assert_eq!(v.nums[1], 2);
    assert_eq!(v.sums.len(), 3);
    assert_eq!(v.sums[0], 0);
    assert_eq!(v.sums[1], 1);
    assert_eq!(v.sums[2], 3);
}
