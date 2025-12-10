// just got this from claude innit

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::{self, File},
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone, Copy)]
struct Pair {
    i: usize,
    j: usize,
    dist: f64,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.total_cmp(&other.dist).reverse()
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Pair {}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

/// A Union-Find data structure to manage disjoint sets (circuits).
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    num_components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            num_components: n,
        }
    }

    /// Finds the representative (root) of the set containing `i`.
    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            self.parent[i] = self.find(self.parent[i]);
            self.parent[i]
        }
    }

    /// Unites the sets containing `i` and `j`. Returns `true` if a union
    /// occurred (they were in different sets), `false` otherwise.
    fn unite(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            // Attach smaller tree under root of larger tree
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            self.num_components -= 1;
            true
        } else {
            false
        }
    }

    /// Returns the number of disjoint sets.
    fn num_components(&self) -> usize {
        self.num_components
    }

    /// Returns the size of the set containing `i`.
    #[allow(dead_code)] // only used for `circuit_sizes` in R example
    fn get_set_size(&mut self, i: usize) -> usize {
        let root = self.find(i);
        self.size[root]
    }

    /// Calculates the size of each unique circuit (root) and returns them
    /// sorted in descending order.
    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let mut root_sizes: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i); // Ensure paths are compressed
            *root_sizes.entry(root).or_insert(0) += 1;
        }
        let mut sizes: Vec<usize> = root_sizes.values().cloned().collect();
        sizes.sort_by(|a, b| b.cmp(a));
        sizes
    }
}

pub fn day_eight() {
    let now = Instant::now();

    let file = File::open("./input/eight/big.txt").unwrap();
    let reader = BufReader::new(file);

    let coords: Vec<Coords> = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.split(',');
            let x = iter.next().unwrap().parse::<usize>().unwrap();
            let y = iter.next().unwrap().parse::<usize>().unwrap();
            let z = iter.next().unwrap().parse::<usize>().unwrap();
            Coords { x, y, z }
        })
        .collect();

    let n = coords.len();

    let mut pairs = BinaryHeap::new();
    // ew
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = (coords[i].x as isize - coords[j].x as isize) as f64;
            let dy = (coords[i].y as isize - coords[j].y as isize) as f64;
            let dz = (coords[i].z as isize - coords[j].z as isize) as f64;
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();
            pairs.push(Pair { i, j, dist });
        }
    }
    let len = pairs.len();

    let mut uf = UnionFind::new(n);

    // Part 1
    // We "make" 1000 connections (even if they don't unite circuits)
    let mut iter = std::iter::from_fn(move || pairs.pop());
    for _ in 0..1000 {
        let pair = iter.next().unwrap();
        uf.unite(pair.i, pair.j);
    }

    let circuit_sizes = uf.get_circuit_sizes();
    let part1_ans: usize = circuit_sizes.iter().take(3).product();

    // Part 2
    let mut last_i = 0;
    let mut last_j = 0;
    for pair in iter {
        // Start from index 1000 because 0-999 were used for part 1
        if uf.unite(pair.i, pair.j) {
            // A union actually occurred
            if uf.num_components() == 1 {
                last_i = pair.i;
                last_j = pair.j;
                break;
            }
        }
    }

    let part2_ans = coords[last_i].x * coords[last_j].x;

    let elapsed = now.elapsed();

    println!("Part 1 Answer: {}", part1_ans);
    println!("Part 2 Answer: {}", part2_ans);
    println!("Time elapsed: {:.2?}", elapsed);
}
