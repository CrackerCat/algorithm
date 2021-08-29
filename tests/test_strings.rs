#![feature(is_sorted)]
#![allow(non_snake_case)]
use algo::strings::{Quick3String, Quick3Way, TrieST, LSD, MSD};
use std::collections::HashMap;

const WORDS3: &'static str = include_str!("../res/strings/words3.txt");
const SHELLS: &'static str = include_str!("../res/strings/shells.txt");
const SHELLS_ST: &'static str = include_str!("../res/strings/shellsST.txt");

#[test]
fn LSD_radix_sort() {
    let i = WORDS3;
    let mut a = extract_words(i);
    let w = a[0].len();
    LSD::sort(&mut a, w);
    assert!(a.is_sorted());

    // license plate data
    let mut a = vec![
        "4PGC938", "2IYE230", "3CIO720", "1ICK750", "1OHV845", "4JZY524", "1ICK750", "3CIO720",
        "1OHV845", "1OHV845", "2RLA629", "2RLA629", "3ATW723",
    ];
    let w = a[0].len();
    LSD::sort(&mut a, w);
    assert!(a.is_sorted());
}

#[test]
fn LSD_radix_sort_i32() {
    let mut a: Vec<i32> = (0..10).rev().collect();
    LSD::sort_i32(&mut a);
    assert!(a.is_sorted());

    let mut a = vec![1, 2, 3, -1, -2, -3];
    LSD::sort_i32(&mut a);
    assert!(a.is_sorted());
}

#[test]
fn MSD_radix_sort() {
    // empty
    let mut data: Vec<&str> = vec![];
    MSD::sort(&mut data);
    assert!(data.is_sorted());

    // normal
    let i = SHELLS;
    let mut data = extract_words(i);
    MSD::sort(&mut data);
    assert!(data.is_sorted());
}

#[test]
fn quick3str() {
    // empty
    let mut data: Vec<&str> = vec![];
    Quick3String::sort(&mut data);
    assert!(data.is_sorted());

    // normal
    let i = SHELLS;
    let mut data = extract_words(i);
    Quick3String::sort(&mut data);
    assert!(data.is_sorted());
}

#[test]
fn quick3way() {
    // empty
    let mut data: Vec<&str> = vec![];
    Quick3Way::sort(&mut data);
    assert!(data.is_sorted());

    // normal
    let i = SHELLS;
    let mut data = extract_words(i);
    Quick3Way::sort(&mut data);
    assert!(data.is_sorted());
}

// also fine for sorted data
#[test]
fn sorted_data() {
    let mut a = vec![
        "4PGC938", "2IYE230", "3CIO720", "1ICK750", "1OHV845", "4JZY524", "1ICK750", "3CIO720",
        "1OHV845", "1OHV845", "2RLA629", "2RLA629", "3ATW723",
    ];
    a.sort();

    // lsd
    let w = a[0].len();
    LSD::sort(&mut a, w);
    assert!(a.is_sorted());

    // msd
    MSD::sort(&mut a);
    assert!(a.is_sorted());

    // three-way quick
    Quick3String::sort(&mut a);
    assert!(a.is_sorted());

    // lsd32
    let mut a: Vec<i32> = (0..10).collect();
    LSD::sort_i32(&mut a);
    assert!(a.is_sorted());
}

#[test]
fn trie_st() {
    let mut trie_st = TrieST::default();
    let i = SHELLS_ST;
    let mut hm = HashMap::new();
    let a = extract_words(i);

    // test len & empty
    assert!(trie_st.is_empty());
    assert_eq!(0, trie_st.len());

    for (i, &s) in a.iter().enumerate() {
        hm.insert(s, i);
        // test put
        trie_st.put(s, Some(i));
    }

    // test len & empty
    assert!(!trie_st.is_empty());
    assert_eq!(7, trie_st.len());

    for (&k, v) in hm.iter() {
        // test get & contains
        assert_eq!(trie_st.get(k), Some(v));
        assert!(trie_st.contains(k));
    }

    // test keys_with_prefix
    let mut matches = trie_st.keys_with_prefix("shor");
    assert_eq!(Some("shore"), matches.dequeue().as_deref());

    // test keys_that_match
    let mut matches = trie_st.keys_that_match(".he.l.");
    assert_eq!(Some("shells"), matches.dequeue().as_deref())
}

fn extract_words(i: &str) -> Vec<&str> {
    i.split_whitespace().collect()
}
