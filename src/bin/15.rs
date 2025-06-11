use rand::seq::IteratorRandom; // For randomly choosing elements from iterators
use std::collections::HashMap; // For representing the graph as an adjacency map

/// Perform one run of Karger’s contraction algorithm on the given graph.
/// The graph is represented as an adjacency map: each node → a Vec of its neighbors.
///
/// Returns the size of the cut found (i.e. the number of edges crossing when only two “super-nodes” remain).
fn karger_contraction(adj: &HashMap<u32, Vec<u32>>) -> usize {
    // Make a working copy of the adjacency map so we can mutate it
    let mut adj = adj.clone();
    let mut rng = rand::rng();

    // While more than 2 super-nodes remain, keep contracting
    while adj.len() > 2 {
        // Pick a random edge (u, v):
        // 1. Randomly pick a node u from the keys
        // 2. Randomly pick a neighbor v of u
        let (&u, nbrs) = adj.iter().choose(&mut rng).unwrap();
        let &v = nbrs.iter().choose(&mut rng).unwrap();

        // Merge v into u:
        // 1) Append v’s adjacency list onto u’s adjacency list
        let v_nbrs = adj.remove(&v).unwrap(); // Remove v and get its neighbors
        let u_nbrs = adj.get_mut(&u).unwrap();
        u_nbrs.extend(v_nbrs.iter().cloned());

        // 2) For every neighbor w of v, replace occurrences of v with u in their adjacency lists
        for w in v_nbrs {
            if let Some(w_nbrs) = adj.get_mut(&w) {
                for x in w_nbrs.iter_mut() {
                    if *x == v {
                        *x = u; // Replace v with u
                    }
                }
            }
        }

        // 3) Remove self‐loops at u (edges from u to itself)
        adj.get_mut(&u)
           .unwrap()
           .retain(|&x| x != u);
    }

    // Now exactly two nodes remain; pick one and return its number of edges (the cut size)
    let (_, nbrs) = adj.iter().next().unwrap();
    nbrs.len()
}

/// Run Karger’s algorithm `reps` times and return the smallest cut found.
/// More repetitions increase the probability of finding the true min-cut.
fn min_cut(adj: &HashMap<u32, Vec<u32>>, reps: usize) -> usize {
    (0..reps)
        .map(|_| karger_contraction(adj)) // Run the contraction algorithm multiple times
        .min() // Take the smallest cut found
        .unwrap()
}

fn main() {
    // Number of repetitions; more repetitions → higher probability of finding true min-cut
    const REPS: usize = 50;

    // --- Test graph 1: a simple path 1–2–3  (min-cut = 1) ---
    let mut g1 = HashMap::new();
    g1.insert(1, vec![2]);
    g1.insert(2, vec![1, 3]);
    g1.insert(3, vec![2]);

    // --- Test graph 2: a triangle 1–2–3–1  (min-cut = 2) ---
    let mut g2 = HashMap::new();
    g2.insert(1, vec![2, 3]);
    g2.insert(2, vec![1, 3]);
    g2.insert(3, vec![1, 2]);

    // --- Test graph 3: two triangles connected by a single bridge (min-cut = 1) ---
    // Triangle on {1,2,3}, triangle on {4,5,6}, plus edge 3–4
    let mut g3: HashMap<u32, Vec<u32>> = HashMap::new();
    // Add all triangle vertices
    for &u in &[1, 2, 3] {
        g3.entry(u).or_default();
    }
    // Add triangle edges for first triangle
    for &(u, v) in &[(1,2),(2,3),(3,1)] {
        g3.get_mut(&u).unwrap().push(v);
        g3.get_mut(&v).unwrap().push(u);
    }
    // Add all triangle vertices for second triangle
    for &u in &[4, 5, 6] {
        g3.entry(u).or_default();
    }
    // Add triangle edges for second triangle
    for &(u, v) in &[(4,5),(5,6),(6,4)] {
        g3.get_mut(&u).unwrap().push(v);
        g3.get_mut(&v).unwrap().push(u);
    }
    // Add the bridge between the two triangles
    g3.get_mut(&3).unwrap().push(4);
    g3.get_mut(&4).unwrap().push(3);

    // List of test graphs with names
    let graphs = vec![
        ("Path 1-2-3", g1),
        ("Triangle 1-2-3", g2),
        ("Two triangles + bridge", g3),
    ];

    // Run Karger's algorithm on each test graph and print the estimated min-cut
    for (name, graph) in graphs {
        let cut = min_cut(&graph, REPS);
        println!("{} → estimated min-cut = {}", name, cut);
    }
}
