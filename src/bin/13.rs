use rand::Rng;

/// Simple undirected graph in adjacency‐list form.
struct Graph {
    n: usize,
    adj: Vec<Vec<usize>>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    /// Create an empty graph on n vertices.
    fn new(n: usize) -> Self {
        Graph { n, adj: vec![Vec::new(); n], edges: Vec::new() }
    }

    /// Add an undirected edge (u,v). No checks for duplicates.
    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        self.adj[v].push(u);
        self.edges.push((u, v));
    }

    /// Count how many edges cross the cut defined by `side[v]` in {false,true}.
    fn cut_size(&self, side: &[bool]) -> usize {
        self.edges.iter()
            .filter(|&&(u,v)| side[u] != side[v])
            .count()
    }

    /// Randomized max‐cut: assign each vertex to side = random<bool>().
    fn random_cut(&self) -> (Vec<bool>, usize) {
        let mut rng = rand::rng();
        let side: Vec<bool> = (0..self.n).map(|_| rng.random()).collect();
        let c = self.cut_size(&side);
        (side, c)
    }

    /// Derandomize via conditional expectations to guarantee cut ≥ |E|/2.
    ///
    /// We assign vertices in order 0,1,…,n-1.  At each step, 
    /// we look at edges to already‐assigned neighbors: putting v on the side
    /// that maximizes #new crossing edges so far can only increase the
    /// conditional expectation.
    fn derandomized_cut(&self) -> (Vec<bool>, usize) {
        let mut side = vec![false; self.n];
        let mut assigned = vec![false; self.n];

        // Track for each v the number of neighbors already assigned to 'true'
        let mut true_nbrs = vec![0usize; self.n];
        // and to 'false'
        let mut false_nbrs = vec![0usize; self.n];

        for v in 0..self.n {
            // Evaluate placing v=false vs v=true
            // If v=false, edges crossing to neighbors in true_nbrs[v]
            // If v=true, edges crossing to neighbors in false_nbrs[v]
            let cross_if_false = true_nbrs[v];
            let cross_if_true  = false_nbrs[v];

            side[v] = cross_if_true > cross_if_false;
            assigned[v] = true;

            // Update neighbors' counts
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

        let c = self.cut_size(&side);
        (side, c)
    }
}

fn main() {
    // --- Build a random test graph ---
    let n = 100;
    let p = 0.1;              // edge‐probability
    let mut g = Graph::new(n);
    let mut rng = rand::rng();
    for u in 0..n {
        for v in (u+1)..n {
            if rng.random::<f64>() < p {
                g.add_edge(u, v);
            }
        }
    }

    let m = g.edges.len();
    println!("Graph: n = {}, |E| = {}", n, m);

    // --- Randomized cut ---
    let (_r_side, r_cut) = g.random_cut();
    println!("Randomized cut size: {}", r_cut);

    // --- Derandomized cut ---
    let (_d_side, d_cut) = g.derandomized_cut();
    println!("Derandomized cut size: {}", d_cut);

    println!("\nGuarantee: derandomized_cut ≥ |E|/2 = {:.1}", (m as f64)/2.0);
    assert!(d_cut * 2 >= m, "Derandomization failed the |E|/2 bound!");
}
