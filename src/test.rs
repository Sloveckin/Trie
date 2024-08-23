#[cfg(test)]
mod tests {
    use crate::Trie;
    use std::fmt::Display;

    #[test]
    fn test1() {
        let v = vec!["he", "she", "hers"];
        let trie = Trie::new(&v);
        assert_eq!(trie.find("he"), true);
        assert_eq!(trie.find("she"), true);
        assert_eq!(trie.find("hers"), true);
        assert_eq!(trie.find("Random"), false);
    }

    #[test]
    fn test2() {
        let mut trie = Trie::new(&vec!["Random1", "Random2", "Random3"]);
        assert_eq!(trie.find("Random1"), true);
        assert_eq!(trie.find("Random3"), true);
        assert_eq!(trie.find("Random4"), false);
        trie.add("Random4");
        assert_eq!(trie.find("Random4"), true);
    }

    #[test]
    #[should_panic(expected = "Blank string not supported")]
    fn test3() {
        let _trie = Trie::new(&vec!["ab", ""]);
    }

    #[derive(Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} : {}", &self.x, &self.y)
        }
    }

    #[test]
    fn test4() {
        let p1 = Point { x: 5, y: 32 };
        let p2 = Point { x: 3, y: 10 };
        let p3 = Point { x: -1, y: -3 };

        let p4 = Point { x: 0, y: 0 };
        let trie = Trie::new(&vec![&p1, &p2, &p3]);

        assert_eq!(trie.find(&p1), true);
        assert_eq!(trie.find(&p2), true);
        assert_eq!(trie.find(&p3), true);
        assert_eq!(trie.find(&p4), false);
    }

    #[test]
    fn test5() {
        let trie = Trie::new(&vec!["first", "second", "first"]);
        assert_eq!(trie.find("first"), true);
        assert_eq!(trie.find("second"), true);
    }
}
