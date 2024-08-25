# Бор
Trie (бор) — структура данных для хранения набора строк, в которой поиск работает пропорционально длине запроса.

## Example 1
```rust
let vec = vec!["Rust", "Cpp"];
let mut trie = Trie::new(&vec);
assert_eq!(trie.find("Rust"), true);
assert_eq!(trie.find("Cpp"), true);
assert_eq!(trie.find("Java"), false);
```

## Example 2
### Также можно хранить структуры, которые реализуют трейт ToString
```rust 
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
```

```rust
let p1 = Point { x: 5, y: 32 };
let p2 = Point { x: 3, y: 10 };
let p3 = Point { x: -1, y: -3 };
let p4 = Point { x: 0, y: 0 };

let trie = Trie::new(&vec![&p1, &p2, &p3]);

assert_eq!(trie.find(&p1), true);
assert_eq!(trie.find(&p2), true);
assert_eq!(trie.find(&p3), true);
assert_eq!(trie.find(&p4), false);
```
