
pub struct Monkey<'a> {
    pub items: Vec<usize>,
    pub operation: &'a dyn Fn(usize) -> usize,
    pub divisor: usize,
    pub on_success: usize,
    pub on_failure: usize
}

pub fn get_example_monkeys () -> Vec<Monkey<'static>> {
    let m0 = Monkey {
        items: vec!(79, 98),
        operation: &|x| x * 19,
        divisor: 23,
        on_success: 2,
        on_failure: 3,
    };

    let m1 = Monkey {
        items: vec!(54, 65, 75, 74),
        operation: &|x| x + 6,
        divisor: 19,
        on_success: 2,
        on_failure: 0,
    };

    let m2 = Monkey {
        items: vec!(79, 60, 97),
        operation: &|x| x * x,
        divisor: 13,
        on_success: 1,
        on_failure: 3,
    };

    let m3 = Monkey {
        items: vec!(74),
        operation: &|x| x + 3,
        divisor: 17,
        on_success: 0,
        on_failure: 1,
    };

    return vec![m0, m1, m2, m3];
}

pub fn get_actual_monkeys() -> Vec<Monkey<'static>> {
    let m0 = Monkey {
        items: vec!(83, 88, 96, 79, 86, 88, 70),
        operation: &|x| x * 5,
        divisor: 11,
        on_success: 2,
        on_failure: 3,
    };

    let m1 = Monkey {
        items: vec!(59, 63, 98, 85, 68, 72),
        operation: &|x| x * 11,
        divisor: 5,
        on_success: 4,
        on_failure: 0,
    };

    let m2 = Monkey {
        items: vec!(90, 79, 97, 52, 90, 94, 71, 70),
        operation: &|x| x + 2,
        divisor: 19,
        on_success: 5,
        on_failure: 6,
    };

    let m3 = Monkey {
        items: vec!(97, 55, 62),
        operation: &|x| x + 5,
        divisor: 13,
        on_success: 2,
        on_failure: 6,
    };

    let m4 = Monkey {
        items: vec!(74, 54, 94, 76),
        operation: &|x| x * x,
        divisor: 7,
        on_success: 0,
        on_failure: 3,
    };

    let m5 = Monkey {
        items: vec!(58),
        operation: &|x| x + 4,
        divisor: 17,
        on_success: 7,
        on_failure: 1,
    };

    let m6 = Monkey {
        items: vec!(66, 63),
        operation: &|x| x + 6,
        divisor: 2,
        on_success: 7,
        on_failure: 5,
    };

    let m7 = Monkey {
        items: vec!(56, 56, 90, 96, 68),
        operation: &|x| x + 7,
        divisor: 3,
        on_success: 4,
        on_failure: 1,
    };

    return vec![m0, m1, m2, m3, m4, m5, m6, m7];
}