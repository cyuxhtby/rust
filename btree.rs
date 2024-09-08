// B-tree is a self-balancing tree, some properties:
//  - Allow nodes to have more than two children
//  - The left children of the root is smaller than the root and the right children is larger
//  - Each node keys in ascending order, and each key has two references to child nodes
//  - All operations can be performed in logarithmic time

// Struct Defention:
// pub struct BTreeMap<K, V, A = Global>
// where
//     A: Allocator + Clone,
// { /* private fields */ }

use std::collections::BTreeMap;

fn main() {
    let mut movie_reviews = BTreeMap::new();

    movie_reviews.insert("Space Jam 2", "Ehh");
    movie_reviews.insert("Pulp Fiction", "Real good!");
    movie_reviews.insert("The Godfather", "I fell asleep tbh");

    if !movie_reviews.contains_key("Les Misérables") {
        println!(
            "We've got {} reviews, but Les Misérables ain't one.",
            movie_reviews.len()
        );
    }

    let to_find = ["Up!", "Pulp Fiction"];
    for movie in &to_find {
        match movie_reviews.get(movie) {
            Some(review) => println!("{movie}: {review}"),
            None => println!("{movie} is unreviewed."),
        }
    }

    // iterate over everything.
    for (movie, review) in &movie_reviews {
        println!("{movie}: \"{review}\"");
    }
}
