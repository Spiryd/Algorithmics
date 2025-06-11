use rand::Rng; // Import random number generation traits

/// Simple undirected graph in adjacency‐list form.
struct Graph {
    n: usize,                  // Number of vertices
    adj: Vec<Vec<usize>>,      // Adjacency list: adj[v] = neighbors of v
    edges: Vec<(usize, usize)>,// List of edges (u, v)
}

impl Graph {
    /// Create an empty graph on n vertices.
    fn new(n: usize) -> Self {
        Graph { n, adj: vec![Vec::new(); n], edges: Vec::new() }
    }

    /// Add an undirected edge (u,v). No checks for duplicates.
    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);      // Add v to u's adjacency list
        self.adj[v].push(u);      // Add u to v's adjacency list
        self.edges.push((u, v));  // Store the edge in the edge list
    }

    /// Count how many edges cross the cut defined by `side[v]` in {false,true}.
    /// An edge (u, v) crosses the cut if side[u] != side[v].
    fn cut_size(&self, side: &[bool]) -> usize {
        self.edges.iter()
            .filter(|&&(u,v)| side[u] != side[v])
            .count()
    }

    /// Randomized max‐cut: assign each vertex to side = random<bool>().
    /// Returns the side assignment and the cut size.
    fn random_cut(&self) -> (Vec<bool>, usize) {
        let mut rng = rand::rng();
        // Randomly assign each vertex to true or false
        let side: Vec<bool> = (0..self.n).map(|_| rng.random()).collect();
        let c = self.cut_size(&side); // Compute the cut size
        (side, c)
    }

    /// Derandomize via conditional expectations to guarantee cut ≥ |E|/2.
    ///
    /// Assign vertices in order 0,1,…,n-1. At each step, choose the side
    /// (true or false) that maximizes the number of new crossing edges
    /// to already-assigned neighbors. This ensures the expected cut size
    /// is at least half the number of edges.
    fn derandomized_cut(&self) -> (Vec<bool>, usize) {
        let mut side = vec![false; self.n];      // Side assignment for each vertex
        let mut assigned = vec![false; self.n];  // Track which vertices are assigned

        // Track for each vertex the number of neighbors already assigned to 'true'
        let mut true_nbrs = vec![0usize; self.n];
        // and to 'false'
        let mut false_nbrs = vec![0usize; self.n];

        for v in 0..self.n {
            // For vertex v, compute the number of crossing edges if v is assigned false or true
            // If v=false, edges crossing to neighbors in true_nbrs[v]
            // If v=true, edges crossing to neighbors in false_nbrs[v]
            let cross_if_false = true_nbrs[v];
            let cross_if_true  = false_nbrs[v];

            // Assign v to the side that maximizes the number of new crossing edges
            side[v] = cross_if_true > cross_if_false;
            assigned[v] = true;

            // Update the neighbor counts for unassigned neighbors
            for &w in &self.adj[v] {
                if !assigned[w] {
                    if side[v] {
                        true_nbrs[w] += 1;
                    } else {
                        false_nbrs[w] += 1;
                    }
                }
            }
        }

        let c = self.cut_size(&side); // Compute the final cut size
        (side, c)
    }
}

fn main() {
    // --- Build a random test graph ---
    let n = 100;         // Number of vertices
    let p = 0.1;         // Probability of including each possible edge
    let mut g = Graph::new(n);
    let mut rng = rand::rng();
    // For each pair of vertices (u, v), add an edge with probability p
    for u in 0..n {
        for v in (u+1)..n {
            if rng.random::<f64>() < p {
                g.add_edge(u, v);
            }
        }
    }

    let m = g.edges.len(); // Total number of edges
    println!("Graph: n = {}, |E| = {}", n, m);

    // --- Randomized cut ---
    let (_r_side, r_cut) = g.random_cut();
    println!("Randomized cut size: {}", r_cut);

    // --- Derandomized cut ---
    let (_d_side, d_cut) = g.derandomized_cut();
    println!("Derandomized cut size: {}", d_cut);

    // Print the guarantee for the derandomized cut
    println!("\nGuarantee: derandomized_cut ≥ |E|/2 = {:.1}", (m as f64)/2.0);
    // Assert that the derandomized cut meets the guarantee
    assert!(d_cut * 2 >= m, "Derandomization failed the |E|/2 bound!");
}
