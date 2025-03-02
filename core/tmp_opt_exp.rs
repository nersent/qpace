#[derive(Clone)]
pub struct test_Ctx {
    pub bar_index: Rc<Cell<Option<usize>>>,
}

impl test_Ctx {
    #[inline]
    pub fn new() -> Self {
        return Self {
            bar_index: Rc::new(Cell::new(None)),
        };
    }

    #[inline]
    pub fn bar_index(&self) -> usize {
        return self.bar_index.get().unwrap();
    }

    #[inline]
    pub fn next_bar(&self) -> Option<usize> {
        let bar_index = self.bar_index.get().map(|x| x + 1).unwrap_or(0);
        if bar_index >= 100_000 {
            return None;
        }
        self.bar_index.set(Some(bar_index));
        return Some(bar_index);
    }
}

#[derive(Clone)]
pub struct test_Ctx2 {
    pub bar_index: usize,
    pub initialized: bool,
}

impl test_Ctx2 {
    #[inline]
    pub fn new() -> Self {
        return Self {
            bar_index: 0,
            initialized: false,
        };
    }

    #[inline]
    pub fn bar_index(&self) -> usize {
        return self.bar_index;
    }

    #[inline]
    pub fn next_bar(&mut self) -> Option<usize> {
        if self.initialized {
            let bar_index = self.bar_index + 1;
            if bar_index >= 100_000 {
                return None;
            }
            self.bar_index = bar_index;
            return Some(bar_index);
        }
        self.initialized = true;
        return Some(0);
    }
}

#[pyfunction]
pub fn test() {
    let mut times: Vec<u128> = Vec::new();
    // 24.61238
    // for _ in 0..10_000_000 {
    //     let mut ctx = test_Ctx::new();
    //     let start_time = Instant::now();
    //     black_box(loop {
    //         if ctx.next_bar().is_none() {
    //             break;
    //         }
    //     });
    //     let end_time = Instant::now();
    //     let duration = end_time.duration_since(start_time);
    //     times.push(duration.as_nanos());
    // }
    // 38.91228
    for _ in 0..10_000_000 {
        let mut ctx = test_Ctx2::new();
        let start_time = Instant::now();
        black_box(loop {
            if ctx.next_bar().is_none() {
                break;
            }
        });
        let end_time = Instant::now();
        let duration = end_time.duration_since(start_time);
        times.push(duration.as_nanos());
    }
    let mean = times.iter().sum::<u128>() as f64 / times.len() as f64;
    println!("mean: {}", mean);
}
