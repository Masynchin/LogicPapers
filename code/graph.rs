use crepe::crepe;

crepe! {
    @input
    struct Edge(u8, u8);

    @input
    struct Single(u8);

    @output
    #[derive(Debug)]
    struct CanWalk(u8, u8);

    struct Node(u8);

    Node(x) <- Single(x);
    Node(x) <- Edge(x, _);
    Node(x) <- Edge(_, x);

    CanWalk(x, x) <- Node(x);

    CanWalk(x, y) <- Edge(x, y);
    CanWalk(x, y) <- Edge(y, x);

    CanWalk(x, y) <- CanWalk(x, z), CanWalk(z, y);
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn edge() {
        let mut runtime = Crepe::new();
        runtime.extend([Edge(1, 2), Edge(1, 3), Edge(3, 4)]);
        runtime.extend([Single(5)]);
        let (walk,) = runtime.run();
        assert!(walk.contains(&CanWalk(1, 2)));
    }

    #[test]
    fn reflex() {
        let mut runtime = Crepe::new();
        runtime.extend([Edge(1, 2), Edge(1, 3), Edge(3, 4)]);
        runtime.extend([Single(5)]);
        let (walk,) = runtime.run();
        assert!(walk.contains(&CanWalk(4, 4)));
        assert!(walk.contains(&CanWalk(5, 5)));
    }

    #[test]
    fn symmetric() {
        let mut runtime = Crepe::new();
        runtime.extend([Edge(1, 2), Edge(1, 3), Edge(3, 4)]);
        runtime.extend([Single(5)]);
        let (walk,) = runtime.run();
        assert!(walk.contains(&CanWalk(2, 1)));
    }

    #[test]
    fn transitive() {
        let mut runtime = Crepe::new();
        runtime.extend([Edge(1, 2), Edge(1, 3), Edge(3, 4)]);
        runtime.extend([Single(5)]);
        let (walk,) = runtime.run();
        assert!(walk.contains(&CanWalk(2, 4)));
    }

    #[test]
    fn no_path() {
        let mut runtime = Crepe::new();
        runtime.extend([Edge(1, 2), Edge(1, 3), Edge(3, 4)]);
        runtime.extend([Single(5)]);
        let (walk,) = runtime.run();
        assert!(!walk.contains(&CanWalk(3, 5)));
    }

    #[test]
    fn to_three() {
        let mut runtime = Crepe::new();
        runtime.extend([Edge(1, 2), Edge(1, 3), Edge(3, 4)]);
        runtime.extend([Single(5)]);
        let (walk,) = runtime.run();
        let to_three: HashSet<u8> = walk
            .into_iter()
            .filter_map(|walk| match walk {
                CanWalk(x, 3) => Some(x),
                _ => None,
            })
            .collect();
        assert_eq!(to_three, [1, 2, 3, 4].into_iter().collect());
    }
}
