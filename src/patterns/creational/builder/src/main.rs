#[derive(Debug)]
struct House {
    walls: u8,
    doors: u8,
    windows: u8,
    has_garage: bool,
    has_swimming_pool: bool,
}

struct HouseBuilder {
    walls: u8,
    doors: u8,
    windows: u8,
    has_garage: bool,
    has_swimming_pool: bool,
}

impl HouseBuilder {
    fn new() -> Self {
        HouseBuilder {
            walls: 0,
            doors: 0,
            windows: 0,
            has_garage: false,
            has_swimming_pool: false,
        }
    }

    fn walls(mut self, count: u8) -> Self {
        self.walls = count;
        self
    }

    fn doors(mut self, count: u8) -> Self {
        self.doors = count;
        self
    }

    fn windows(mut self, count: u8) -> Self {
        self.windows = count;
        self
    }

    fn garage(mut self, has_garage: bool) -> Self {
        self.has_garage = has_garage;
        self
    }

    fn swimming_pool(mut self, has_swimming_pool: bool) -> Self {
        self.has_swimming_pool = has_swimming_pool;
        self
    }

    fn build(self) -> House {
        House {
            walls: self.walls,
            doors: self.doors,
            windows: self.windows,
            has_garage: self.has_garage,
            has_swimming_pool: self.has_swimming_pool,
        }
    }
}

fn main() {
    let house = HouseBuilder::new()
        .walls(4)
        .doors(2)
        .windows(6)
        .garage(true)
        .swimming_pool(false)
        .build();

    println!("{:?}", house);
}