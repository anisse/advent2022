fn main() {
    let trees = parse(include_str!("../input.txt"));
    //part 1
    let res = count_visible(&trees);
    println!("Summary: {}", res);
    //part 2
    //let res = count_visible2(&trees);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| {
                    assert!(x.is_ascii_digit(), "not int");
                    x as u8 - '0' as u8
                })
                .collect()
        })
        .collect()
}

fn normal<T>(s: &[Vec<T>], i: usize, j: usize) -> &T {
    &s[j][i]
}
fn normal_mut<T>(s: &mut [Vec<T>], i: usize, j: usize) -> &mut T {
    &mut s[j][i]
}
fn reversed<T>(s: &[Vec<T>], i: usize, j: usize) -> &T {
    &s[i][j]
}
fn reversed_mut<T>(s: &mut [Vec<T>], i: usize, j: usize) -> &mut T {
    &mut s[i][j]
}

fn count_visible(trees: &[Vec<u8>]) -> usize {
    let mut count = 0;
    let mut seen = vec![vec![false; trees[0].len()]; trees.len()];
    for (mut rangey, rangex, rev, tree, see) in [
        (
            0..trees[0].len(),
            0..trees.len(),
            true,
            normal::<u8> as fn(s: &[Vec<u8>], i: usize, j: usize) -> &u8,
            normal_mut::<bool> as fn(s: &mut [Vec<bool>], i: usize, j: usize) -> &mut bool,
        ),
        (
            0..trees[0].len(),
            0..trees.len(),
            false,
            normal::<u8>,
            normal_mut::<bool>,
        ),
        (
            0..trees.len(),
            0..trees[0].len(),
            false,
            reversed::<u8>,
            reversed_mut::<bool>,
        ),
        (
            0..trees.len(),
            0..trees[0].len(),
            true,
            reversed::<u8>,
            reversed_mut::<bool>,
        ),
    ]
    .into_iter()
    {
        let mut revrange1 = rangey.clone().rev();
        let range1 = if rev {
            &mut revrange1 as &mut dyn Iterator<Item = usize>
        } else {
            &mut rangey as &mut dyn Iterator<Item = usize>
        };
        for y in &mut *range1 {
            let mut min = -1;
            let mut range2 = rangex.clone();
            let mut revrange2 = rangex.clone().rev();
            let range2 = if rev {
                &mut revrange2 as &mut dyn Iterator<Item = usize>
            } else {
                &mut range2 as &mut dyn Iterator<Item = usize>
            };
            for x in &mut *range2 {
                if *tree(trees, x, y) as i16 > min {
                    min = *tree(trees, x, y) as i16;
                    if !*see(&mut seen, x, y) {
                        count += 1;
                        *see(&mut seen, x, y) = true;
                    }
                }
            }
        }
    }
    count
}

#[test]
fn test() {
    let trees = parse(include_str!("../sample.txt"));
    dbg!(&trees);
    //part 1
    let res = count_visible(&trees);
    assert_eq!(res, 21);
    //part 2
    // let res = count_visible2(&trees);
    // assert_eq!(res, 42);
}
