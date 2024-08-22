#[cfg(test)]
mod tests {
    use crate::Trie;

    #[test]
    fn test1() {
        let v = vec!["he", "she", "hers"];
        let trie = Trie::new(v);
        assert_eq!(trie.find("he"), true);
        assert_eq!(trie.find("she"), true);
        assert_eq!(trie.find("hers"), true);
        assert_eq!(trie.find("Random"), false);
    }

    #[test]
    fn test2() {
        let mut trie = Trie::new(vec!["Random1", "Random2", "Random3"]);
        assert_eq!(trie.find("Random1"), true);
        assert_eq!(trie.find("Random3"), true);
        assert_eq!(trie.find("Random4"), false);
        trie.add("Random4");
        assert_eq!(trie.find("Random4"), true);
    }

    #[test]
    #[should_panic(expected = "Blank string not supported")]
    fn test3() {
        let _trie = Trie::new(vec!["ab", ""]);
    }
}
