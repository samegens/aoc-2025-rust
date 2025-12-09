mod segment;

use common::{InputReader, Point3d};
use segment::Segment;
use std::{collections::HashSet, str::Lines};

fn main() {
    let input_reader: InputReader = InputReader::new(8);
    println!("Part 1: {}", solve_part1(input_reader.lines(), 1000));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines, nr_connections: usize) -> i64 {
    let mut points: Vec<Point3d> = lines.map(|line| parse_3d_point(line)).collect();
    points.sort();
    let segments: Vec<Segment> = generate_segments(&points);
    let mut clusters: Vec<HashSet<Point3d>> = create_clusters_from_points(points);
    for i in 0..nr_connections {
        let segment = segments[i];
        let first_cluster_index: usize =
            get_cluster_index_containing_point(&clusters, &segment.start);
        let second_cluster_index: usize =
            get_cluster_index_containing_point(&clusters, &segment.end);
        if first_cluster_index != second_cluster_index {
            merge_clusters(&mut clusters, first_cluster_index, second_cluster_index);
        }
    }

    clusters.sort_by(|a, b| b.len().cmp(&a.len()));
    (clusters[0].len() * clusters[1].len() * clusters[2].len()) as i64
}

fn solve_part2(lines: Lines) -> i64 {
    let mut points: Vec<Point3d> = lines.map(|line| parse_3d_point(line)).collect();
    points.sort();
    let segments: Vec<Segment> = generate_segments(&points);
    let mut clusters: Vec<HashSet<Point3d>> = create_clusters_from_points(points);
    for segment in segments {
        let first_cluster_index: usize =
            get_cluster_index_containing_point(&clusters, &segment.start);
        let second_cluster_index: usize =
            get_cluster_index_containing_point(&clusters, &segment.end);
        if first_cluster_index != second_cluster_index {
            if clusters.len() == 2 {
                // Found the last two junction boxes that will connect all junction boxes in one giant circuit.
                return segment.start.x * segment.end.x;
            }
            merge_clusters(&mut clusters, first_cluster_index, second_cluster_index);
        }
    }

    panic!("No solution found!");
}

fn parse_3d_point(line: &str) -> Point3d {
    let parts: Vec<i64> = line
        .split(',')
        .map(|s: &str| s.parse::<i64>().unwrap())
        .collect();
    Point3d::new(parts[0], parts[1], parts[2])
}

fn create_clusters_from_points(points: Vec<Point3d>) -> Vec<HashSet<Point3d>> {
    let clusters: Vec<HashSet<Point3d>> = points
        .into_iter()
        .map(|point| {
            let mut hashset: HashSet<Point3d> = HashSet::new();
            hashset.insert(point);
            hashset
        })
        .collect();
    clusters
}

/// Generates a list of all possible segments, sorted on distance.
fn generate_segments(points: &Vec<Point3d>) -> Vec<Segment> {
    let mut segments: Vec<Segment> = vec![];

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let segment = Segment::new(&points[i], &points[j]);
            segments.push(segment);
        }
    }

    segments.sort_by(|a, b| a.squared_length.cmp(&b.squared_length));
    segments
}

fn get_cluster_index_containing_point(clusters: &Vec<HashSet<Point3d>>, point: &Point3d) -> usize {
    for i in 0..clusters.len() {
        if clusters[i].contains(point) {
            return i;
        }
    }

    panic!("Point {point:?} not found in any cluster!");
}

fn merge_clusters(
    clusters: &mut Vec<HashSet<Point3d>>,
    first_cluster_index: usize,
    second_cluster_index: usize,
) {
    let second_cluster = clusters[second_cluster_index].clone();
    clusters[first_cluster_index].extend(second_cluster);
    clusters.remove(second_cluster_index);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

    #[test]
    fn test_solve_part1() {
        // Arrange
        let expected: i64 = 40;

        // Act
        let actual: i64 = solve_part1(INPUT.lines(), 10);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2() {
        // Arrange
        let expected: i64 = 25272;

        // Act
        let actual: i64 = solve_part2(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_3d_point() {
        // Arrange
        let line = "162,817,812";

        // Act
        let actual = parse_3d_point(line);

        // Assert
        let expected = Point3d::new(162, 817, 812);
        assert_eq!(actual, expected);
    }
}
