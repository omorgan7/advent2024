use std::{collections::{HashMap, HashSet}, hash::Hash};

fn part1(lhs: &[String], rhs: &[String]) -> i64
{
    let mut mapping = HashMap::<String, HashSet<String>>::new();

    for (x, y) in lhs.iter().zip(rhs) {
        mapping.entry(x.clone()).or_insert(HashSet::new()).insert(y.clone());
        mapping.entry(y.clone()).or_insert(HashSet::new()).insert(x.clone());
    }
    
    let triplets = mapping.iter().map(|(k, v)| {
        v.iter().filter_map(|x| {
            let l0 = mapping.get(x).unwrap();
            let u0 = l0.intersection(v).cloned().collect::<HashSet<_>>();

            if u0.len() == 0 {
                return None;
            }

            let sorted_triplets = u0.iter().filter_map(|y| {
                if !x.starts_with("t") && !k.starts_with("t") && !y.starts_with("t") {
                    return None;
                }

                let mut triplet = [k.clone(), x.clone(), y.clone()];
                triplet.sort();

                Some(triplet)
            }).collect::<Vec<_>>();

            return Some(sorted_triplets);
        }).flatten().collect::<Vec<_>>()
    }).flatten().collect::<HashSet<[String; 3]>>();

    triplets.len() as i64
}

fn find_recursive_joined_sets(k: &String, mut seen: HashSet<String>, mapping: &HashMap::<String, HashSet<String>>) -> HashSet<String>
{
    let v = mapping.get(k).unwrap();

    if seen.contains(k) {
        return seen;
    }

    seen.insert(k.clone());

    v.iter().flat_map(|x| {
        
        let l0 = mapping.get(x).unwrap();
        let u0 = l0.intersection(v).cloned().collect::<HashSet<_>>();
        // println!("{}: {:?} v {}: {:?}, union: {:?}, seen: {:?}", k, v, x, l0, u0, seen);

        u0.iter().map(|y| {
            find_recursive_joined_sets(y, seen.clone(), mapping)
        }).flatten().collect::<HashSet<String>>()
    }).collect::<HashSet<String>>()
}

fn part2(lhs: &[String], rhs: &[String]) -> i64
{
    let mut mapping = HashMap::<String, HashSet<String>>::new();

    for (x, y) in lhs.iter().zip(rhs) {
        mapping.entry(x.clone()).or_insert(HashSet::new()).insert(y.clone());
        mapping.entry(y.clone()).or_insert(HashSet::new()).insert(x.clone());
    }
    
    let sets = mapping.iter().map(|(k, _)| {
        let s0 = find_recursive_joined_sets(k, HashSet::new(), &mapping);
        
        let sum = s0.iter().map(|x| mapping.get(x).unwrap().len() as i64).sum::<i64>();
        println!("{:?}: {}", s0, sum);
        sum
    }).collect::<Vec<_>>();

    // for s in sets.iter() {
    //     println!("{:?}", s);
    // }

    sets.len() as i64
}

fn main() {
    let input = include_str!("../input.txt");

    let (lhs, rhs) : (Vec<String>, Vec<String>)= input.lines().map(|l| {
        let mut it = l.split("-");
        (it.next().unwrap().to_string(), it.next().unwrap().to_string())
    }).collect::<Vec<(String, String)>>().into_iter().unzip();

    println!("{}", part1(&lhs, &rhs));
    println!("{}", part2(&lhs, &rhs));
}
