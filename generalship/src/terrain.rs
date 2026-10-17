use crate::model::{Config, Point, segment_clear};
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
pub const CELL: i32 = 1000;
pub const W: usize = 90;
pub const H: usize = 60;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Terrain {
    pub cells: Vec<u8>,
    pub short_sight: i32,
    pub woods_visibility: i32,
    pub sites: Vec<Point>,
    pub headquarters: [Point; 2],
}
// 0 grass, 1 road, 2 woods, 3 water, 4 bridge, 5 ford, 6 hill.
impl Terrain {
    pub fn new(seed: u64) -> Self {
        Self::configured(seed, &Config::default())
    }
    pub fn configured(_seed: u64, config: &Config) -> Self {
        let mut m = Self {
            cells: vec![0; W * H],
            short_sight: config.short_sight,
            woods_visibility: config.woods_visibility,
            sites: vec![
                Point::new(26000, 20000),
                Point::new(38000, 18000),
                Point::new(52000, 18000),
                Point::new(64000, 20000),
                Point::new(34000, 50000),
                Point::new(56000, 50000),
            ],
            headquarters: [Point::new(8000, 28000), Point::new(82000, 28000)],
        };
        for y in 0..H {
            for x in 0..W {
                let v = if (44..46).contains(&x) {
                    if (26..29).contains(&y) {
                        4
                    } else if (47..50).contains(&y) {
                        5
                    } else {
                        3
                    }
                } else if (26..29).contains(&y) {
                    1
                } else if ((25..38).contains(&x) || (52..65).contains(&x)) && (33..44).contains(&y)
                {
                    2
                } else {
                    0
                };
                m.cells[y * W + x] = v;
            }
        }
        for p in m.sites.clone() {
            let (x, y) = Self::cell(p);
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if let Some(i) = Self::index(x + dx, y + dy)
                        && m.cells[i] == 0
                    {
                        m.cells[i] = 6;
                    }
                }
            }
        }
        m
    }
    pub fn cell(p: Point) -> (i32, i32) {
        (p.x.div_euclid(CELL), p.y.div_euclid(CELL))
    }
    fn index(x: i32, y: i32) -> Option<usize> {
        if x >= 0 && y >= 0 && x < W as i32 && y < H as i32 {
            Some(y as usize * W + x as usize)
        } else {
            None
        }
    }
    pub fn kind(&self, p: Point) -> u8 {
        let (x, y) = Self::cell(p);
        Self::index(x, y).map(|i| self.cells[i]).unwrap_or(3)
    }
    pub fn walkable(&self, p: Point) -> bool {
        self.kind(p) != 3
    }
    pub fn sight(&self, a: Point, b: Point) -> bool {
        if a.near(b, self.short_sight) {
            return true;
        }
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let n = (dx.abs().max(dy.abs()) / 400).max(1);
        for i in 1..=n {
            let p = Point::new(a.x + dx * i / n, a.y + dy * i / n);
            if self.kind(p) == 2 && !a.near(p, self.woods_visibility) {
                return false;
            }
        }
        true
    }
    pub fn path(&self, a: Point, b: Point) -> Vec<Point> {
        self.path_avoiding(a, b, &[], 0)
    }
    pub(crate) fn path_avoiding(
        &self,
        a: Point,
        b: Point,
        occupied: &[Point],
        clearance: i32,
    ) -> Vec<Point> {
        let mut crowded = vec![false; W * H];
        for &p in occupied {
            let (x, y) = Self::cell(p);
            if let Some(i) = Self::index(x, y) {
                crowded[i] = true;
            }
        }
        if !self.walkable(b) {
            return vec![];
        }
        let (ax, ay) = Self::cell(a);
        let (bx, by) = Self::cell(b);
        let (Some(start), Some(end)) = (Self::index(ax, ay), Self::index(bx, by)) else {
            return vec![];
        };
        if start == end {
            return vec![b];
        }
        let mut cost = vec![u32::MAX; W * H];
        let mut parent = vec![usize::MAX; W * H];
        let mut open = BinaryHeap::new();
        cost[start] = 0;
        open.push(Reverse((0u32, start)));
        while let Some(Reverse((_, i))) = open.pop() {
            if i == end {
                break;
            }
            let x = (i % W) as i32;
            let y = (i / W) as i32;
            for (dx, dy) in [
                (0, -1),
                (-1, 0),
                (1, 0),
                (0, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
                (1, 1),
            ] {
                let Some(j) = Self::index(x + dx, y + dy) else {
                    continue;
                };
                if self.cells[j] == 3 {
                    continue;
                }
                if dx != 0
                    && dy != 0
                    && (self.cells[Self::index(x + dx, y).unwrap()] == 3
                        || self.cells[Self::index(x, y + dy).unwrap()] == 3)
                {
                    continue;
                }
                let terrain = match self.cells[j] {
                    1 => 8,
                    2 => 20,
                    5 => 25,
                    _ => 10,
                };
                let step = if dx != 0 && dy != 0 { 14 } else { 10 };
                let from = if i == start {
                    a
                } else {
                    Point::new(x * CELL + CELL / 2, y * CELL + CELL / 2)
                };
                let to = if j == end {
                    b
                } else {
                    Point::new((x + dx) * CELL + CELL / 2, (y + dy) * CELL + CELL / 2)
                };
                let obstructed = occupied.iter().any(|&p| {
                    from.distance2(p) >= i64::from(clearance).pow(2)
                        && !segment_clear(from, to, p, clearance)
                });
                let c = cost[i]
                    + step * terrain
                    + if obstructed || crowded[j] && j != end {
                        2000
                    } else {
                        0
                    };
                if c < cost[j] {
                    cost[j] = c;
                    parent[j] = i;
                    let h = ((bx - x - dx).abs().max((by - y - dy).abs()) as u32) * 80;
                    open.push(Reverse((c + h, j)));
                }
            }
        }
        if parent[end] == usize::MAX {
            return vec![];
        }
        let mut result = vec![b];
        let mut i = end;
        while i != start {
            result.push(Point::new(
                (i % W) as i32 * CELL + CELL / 2,
                (i / W) as i32 * CELL + CELL / 2,
            ));
            i = parent[i];
        }
        result.reverse();
        result
    }
}
