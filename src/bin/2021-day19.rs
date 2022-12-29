use aoc::*;

use eyre::ensure;
use itertools::{iproduct, Itertools};
use regex::Regex;
use smallvec::SmallVec;

use std::collections::VecDeque;
use std::iter;

type Vec3 = [i64; 3];
type Mat3x3 = [Vec3; 3];

const N_PAIRS: usize = n_pairs(12);

const fn n_pairs(n_common_beacons: usize) -> usize {
    n_common_beacons * (n_common_beacons - 1) / 2
}

struct Frame {
    position: Vec3,
    orientation: Mat3x3,
}

impl Frame {
    fn identity() -> Self {
        Self { position: [0; 3], orientation: [[1, 0, 0], [0, 1, 0], [0, 0, 1]] }
    }
}

struct Scanner {
    frame: Option<Frame>,
    beacons: Vec<Vec3>,
}

trait Position {
    fn add(&self, rhs: &Self) -> Self;
    fn sub(&self, rhs: &Self) -> Self;
    fn abs(&self) -> Self;
    fn scale(&self, factor: i64) -> Self;
    fn norm(&self) -> i64;
    fn norm2(&self) -> i64;
    fn dot(&self, rhs: &Self) -> i64;
    fn apply(&self, matrix: &Mat3x3) -> Self;
}

impl Position for Vec3 {
    fn add(&self, rhs: &Self) -> Self {
        [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]]
    }

    fn sub(&self, rhs: &Self) -> Self {
        [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]]
    }

    fn abs(&self) -> Self {
        [self[0].abs(), self[1].abs(), self[2].abs()]
    }

    fn scale(&self, factor: i64) -> Self {
        [factor * self[0], factor * self[1], factor * self[2]]
    }

    fn norm(&self) -> i64 {
        self[0].abs() + self[1].abs() + self[2].abs()
    }

    fn norm2(&self) -> i64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    fn dot(&self, rhs: &Self) -> i64 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    fn apply(&self, matrix: &Mat3x3) -> Self {
        [self.dot(&matrix[0]), self.dot(&matrix[1]), self.dot(&matrix[2])]
    }
}

fn matmul(m1: &Mat3x3, m2: &Mat3x3) -> Mat3x3 {
    [
        [
            m1[0][0] * m2[0][0] + m1[0][1] * m2[1][0] + m1[0][2] * m2[2][0],
            m1[0][0] * m2[0][1] + m1[0][1] * m2[1][1] + m1[0][2] * m2[2][1],
            m1[0][0] * m2[0][2] + m1[0][1] * m2[1][2] + m1[0][2] * m2[2][2],
        ],
        [
            m1[1][0] * m2[0][0] + m1[1][1] * m2[1][0] + m1[1][2] * m2[2][0],
            m1[1][0] * m2[0][1] + m1[1][1] * m2[1][1] + m1[1][2] * m2[2][1],
            m1[1][0] * m2[0][2] + m1[1][1] * m2[1][2] + m1[1][2] * m2[2][2],
        ],
        [
            m1[2][0] * m2[0][0] + m1[2][1] * m2[1][0] + m1[2][2] * m2[2][0],
            m1[2][0] * m2[0][1] + m1[2][1] * m2[1][1] + m1[2][2] * m2[2][1],
            m1[2][0] * m2[0][2] + m1[2][1] * m2[1][2] + m1[2][2] * m2[2][2],
        ],
    ]
}

type PairDist = ([usize; 2], i64);
type PairDistList = Vec<Vec<PairDist>>;

fn compute_pair_dists_list(scanners: &[Scanner]) -> Result<PairDistList> {
    scanners
        .iter()
        .map(|s| {
            let pair_dists = s
                .beacons
                .iter()
                .enumerate()
                .tuple_combinations()
                .map(|((index1, b1), (index2, b2))| ([index1, index2], b2.sub(b1).norm2()))
                .sorted_unstable_by_key(|&(_, dist)| dist)
                .collect_vec();

            ensure!(pair_dists.windows(2).all(|x| x[0].1 < x[1].1), "two beacons have the same distance criteria");

            Ok(pair_dists)
        })
        .try_collect()
}

type Overlap = [(usize, [Vec3; 2], [Vec3; 2]); 2];

fn compute_overlaps(scanners: &[Scanner], pair_dists_list: &PairDistList) -> VecDeque<Overlap> {
    pair_dists_list
        .iter()
        .enumerate()
        .tuple_combinations()
        .filter_map(|((index1, p1), (index2, p2))| {
            let mut p2_iter = p2.iter();

            let mut unique_pairs_iter = p1.iter().filter_map(|&(pair_indices_1, dist1)| {
                p2_iter
                    .take_while_ref(|&&(_, dist2)| dist2 <= dist1)
                    .find(|&&(_, dist2)| dist2 == dist1)
                    .map(|&(pair_indices_2, _)| [pair_indices_1, pair_indices_2])
            });

            let two_pairs: SmallVec<[[[usize; 2]; 2]; 2]> = unique_pairs_iter.by_ref().take(2).collect();
            let count = two_pairs.len() + unique_pairs_iter.count();

            let (N_PAIRS.., &[[idx_pair_1a, idx_pair_2a], [idx_pair_1b, idx_pair_2b]]) = (count, two_pairs.as_slice()) else {
                return None;
            };

            let beacons1 = &scanners[index1].beacons;
            let pair_1a = [beacons1[idx_pair_1a[0]], beacons1[idx_pair_1a[1]]];
            let pair_1b = [beacons1[idx_pair_1b[0]], beacons1[idx_pair_1b[1]]];

            let beacons2 = &scanners[index2].beacons;
            let pair_2a = [beacons2[idx_pair_2a[0]], beacons2[idx_pair_2a[1]]];
            let pair_2b = [beacons2[idx_pair_2b[0]], beacons2[idx_pair_2b[1]]];

            Some([(index1, pair_1a, pair_1b), (index2, pair_2a, pair_2b)])
        })
        .collect()
}

fn compute_scanner_frames(scanners: &mut [Scanner], mut overlaps: VecDeque<Overlap>) -> Result<()> {
    while let Some(mut overlap) = overlaps.pop_front() {
        match (&scanners[overlap[0].0].frame, &scanners[overlap[1].0].frame) {
            (Some(_), None) => (),
            (Some(_), Some(_)) => continue,
            (None, Some(_)) => overlap.swap(0, 1),
            (None, None) => {
                overlaps.push_back(overlap);
                continue;
            }
        }

        let [(index1, pair_1a, pair_1b), (index2, pair_2a, pair_2b)] = overlap;

        let diff1 = pair_1a[1].sub(&pair_1a[0]);
        let diff2 = pair_2a[1].sub(&pair_2a[0]);

        ensure!(diff1.iter().chain(&diff2).all(|&v| v != 0), "all differences of beacon pairs should have non-zero coordinates");

        let diff1_abs = diff1.abs();
        let diff2_abs = diff2.abs();

        let permutations = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
        let &[ix, iy, iz] = permutations.iter().find(|&&[ix, iy, iz]| [diff1_abs[ix], diff1_abs[iy], diff1_abs[iz]] == diff2_abs).value()?;

        let mut axis = [[0; 3]; 3];
        axis[ix] = [1, 0, 0].scale(diff2[0].signum() * diff1[ix].signum());
        axis[iy] = [0, 1, 0].scale(diff2[1].signum() * diff1[iy].signum());
        axis[iz] = [0, 0, 1].scale(diff2[2].signum() * diff1[iz].signum());

        let pair_2a_corr = [pair_2a[0].apply(&axis), pair_2a[1].apply(&axis)];
        let pair_2b_corr = [pair_2b[0].apply(&axis), pair_2b[1].apply(&axis)];

        let (sign, offset) = iproduct!([1, -1], iproduct!(pair_1a, pair_2a_corr).chain(iproduct!(pair_1b, pair_2b_corr)))
            .map(|(sign, (p1, p2c))| (sign, p1.sub(&p2c.scale(sign))))
            .sorted_unstable_by_key(|&(_, x)| x)
            .dedup_by_with_count(|(_, x1), (_, x2)| x1 == x2)
            .find(|&(count, _)| count == 4)
            .map(|(_, x)| x)
            .value()?;

        let frame1 = scanners[index1].frame.as_ref().value()?;

        scanners[index2].frame = Some(Frame {
            position: frame1.position.add(&offset.apply(&frame1.orientation)),
            orientation: matmul(&frame1.orientation, &[axis[0].scale(sign), axis[1].scale(sign), axis[2].scale(sign)]),
        });
    }

    Ok(())
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^(.+?),(.+?),(.+?)$"#)?;

    let mut scanners: Vec<_> = input
        .split("--- scanner")
        .filter(|group| !group.is_empty())
        .map(|group| {
            let beacons = re.captures_iter(group).map(|cap| Result::Ok([cap[1].parse()?, cap[2].parse()?, cap[3].parse()?])).try_collect()?;
            Result::Ok(Scanner { frame: None, beacons })
        })
        .try_collect()?;

    scanners[0].frame = Some(Frame::identity());

    let pair_dists_list = compute_pair_dists_list(&scanners)?;
    let overlaps = compute_overlaps(&scanners, &pair_dists_list);
    compute_scanner_frames(&mut scanners, overlaps)?;

    let frames: Vec<_> = scanners.iter().map(|s| s.frame.as_ref().value()).try_collect()?;

    let mut unique_beacons = iter::zip(&scanners, &frames)
        .map(|(scanner, frame)| Ok(scanner.beacons.iter().map(|beacon| frame.position.add(&(beacon.apply(&frame.orientation))))))
        .try_process(|iter| iter.flatten().collect_vec())?;

    unique_beacons.sort_unstable();
    unique_beacons.dedup();

    let result1 = unique_beacons.len();
    let result2 = frames.iter().tuple_combinations().map(|(f1, f2)| f1.position.sub(&f2.position).norm()).max().value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
