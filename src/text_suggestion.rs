pub mod suggestion_engine
{
    use std::{collections::HashMap, fs};

    const END: char = '#';


    #[derive(Debug)]
    struct Trie(HashMap<char, Box<Trie>>);

    impl Trie {
        fn new() -> Self
        {
            Trie(HashMap::new())
        }

        fn insert(&mut self, text: &str)
        {
            let mut trie = self;

            for c in text.chars()
            {
                trie = trie.0.entry(c).or_insert_with(|| Box::new(Trie::new()));
            }

            trie.0.insert(END, Box::new(Trie::new()));
        }

        fn find(&self, prefix: &str) -> Vec<String>
        {
            let mut trie = self;

            for c in prefix.chars()
            {
                match trie.0.get(&c)
                {
                    Some(t) => { trie = t },
                    None => { return vec![]; }
                }
            }

            Self::_elements(trie)
                .iter()
                .map(|s| prefix.to_owned() + s)
                .collect()
        }

        fn _elements(map: &Trie) -> Vec<String>
        {
            let mut results = vec![];

            for (c, v) in map.0.iter()
            {
                let mut sub_result: Vec<String> = vec![];

                if c == &END
                {
                    sub_result.push("".to_owned())
                }
                else 
                {
                    Self::_elements(v)
                        .iter()
                        .map(|s| sub_result.push(c.to_string() + s))
                        .collect()
                }

                results.extend(sub_result)
            }
            results
        }
    }
    pub struct SuggestionEngine
    {
        trie: Trie
    }

    impl SuggestionEngine {
        pub fn new() -> Self
        {
            let words = fs::read_to_string(r"words.txt").expect("Error reading file");

            let words_list: Vec<&str> = words.lines().collect();

            let mut obj = Self { trie: Trie::new() };

            obj.insert_words(&words_list);

            obj
        }

        fn insert_words<T: AsRef<str>>(&mut self, words: &[T])
        {
            for word in words
            {
                self.trie.insert(word.as_ref());
            }
        }

        pub fn find_words(&self, prefix: &str) -> Vec<String>
        {
            self.trie.find(prefix)
        }
    }

    impl Default for SuggestionEngine {
        fn default() -> Self {
            Self::new()
        }
    }
}

