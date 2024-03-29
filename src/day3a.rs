
use std::fs;

enum WireSegment {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32)
}

#[derive(Clone, Debug)]
struct Point {
    x : i32,
    y : i32
}

#[derive(Clone, Debug)]
struct Line {
    start : Point,
    end : Point,
}

pub fn manhattan_dist(t1:  (i32, i32), t2: (i32, i32)) -> i32 {
    (t1.0 - t2.0).abs() + (t1.1 - t2.1).abs()
}

fn is_between(a: i32, b : i32, x :i32) -> bool {
    if a > b {
        a > x && x > b
    } else {
        b > x && x > a
    }
}

fn do_lines_touch (line1 : &Line, line2 : &Line) -> bool {

    if line1.start.x == line1.end.x {
        if line2.start.y == line2.end.y {
            is_between(line1.start.y, line1.end.y, line2.start.y) && is_between(line2.start.x, line2.end.x, line1.start.x)
        } else {
            false
        }
    } else {
        if line2.start.x == line2.end.x {
            is_between(line1.start.x, line1.end.x, line2.start.x) && is_between(line2.start.y, line2.end.y, line1.start.y)
        } else {
            false
        }
    }
}

fn line_intersection(line1 : Line, line2 : Line) -> Option<Point> {
    if do_lines_touch(&line1, &line2) {
        if line1.start.x == line1.end.x {
            Some(Point { x: line1.start.x, y: line2.start.y })
        } else {
            Some(Point { x: line2.start.x, y: line1.start.y })
        }
    } else {
        None
    }
}

fn to_wire_seg (txt : String) -> WireSegment {
    let ch = txt.chars().next().unwrap();
    let txt_num = &txt[1..];
    let num = txt_num.parse::<i32>().unwrap();
    match ch {
        'L' => WireSegment::Left(num),
        'R' => WireSegment::Right(num),
        'U' => WireSegment::Up(num),
        'D' => WireSegment::Down(num),
        _ => WireSegment::Left(num)
    } 
}

fn convert_string(s : String) -> Vec<WireSegment> {
    s.split(",")
        .map(ToOwned::to_owned)
        .map(|v| to_wire_seg(v))
        .collect()
}

fn read_file() -> Vec<Vec<Line>> {

    let contents = fs::read_to_string("day3.txt")
        .expect("Something went wrong reading the file");
    
    let lines : Vec<String> = contents.lines().map(ToOwned::to_owned).collect();
    lines.into_iter()
        .map(|x| convert_string(x))
        .map(|x| convert_to_lines(x))
        .collect()
}

fn next_point (start_point : &Point, wire_seg : WireSegment) -> Point {
    match wire_seg {
        WireSegment::Left(num) => Point { x: start_point.x - num, y: start_point.y },
        WireSegment::Right(num) =>  Point { x: start_point.x + num, y: start_point.y },
        WireSegment::Up(num) =>  Point { x: start_point.x, y: start_point.y  + num },
        WireSegment::Down(num) => Point { x: start_point.x, y: start_point.y  - num },
    } 
}

fn convert_to_lines (wire_segs : Vec<WireSegment>) -> Vec<Line> {
    let point1 = Point { x: 0, y: 0 };
    let point2 =  Point { x: 0, y: 0 };
    let line1 = Line { start :point1, end : point2};
    let v = vec![line1];

    let mut result = wire_segs.into_iter()
        .fold(v, |mut v, x| {
            let last_line = v.last().unwrap();
            let point2 = next_point(&last_line.end, x);
            let line1 = Line { start : last_line.end.clone(), end : point2};
            v.push(line1);
            v
        } );
    result.drain(1..).collect()
}

#[allow(dead_code)]
pub fn calc() -> i32 {
    let data = read_file();
    let first_line = data.get(0).unwrap();
    let second_line = data.get(1).unwrap();
    println!("first_line:\t{:?}",first_line);
    let mut lowest_dist = 100000;
    for l1 in first_line.into_iter() {
        for l2 in second_line.into_iter() {
            if do_lines_touch(&l1, &l2) {
                let new_point = line_intersection(l1.clone(), l2.clone()).unwrap();

                if manhattan_dist((0,0), (new_point.x.clone(), new_point.y.clone())) < lowest_dist {
                    let dist = manhattan_dist((0,0), (new_point.x.clone(), new_point.y.clone()));
                    if dist != 0 {
                        lowest_dist = dist;
                        println!("lines:\t{:?}\t{:?}", l1, l2);

                       println!("int point:\t{:?}", new_point);

                       println!("new low dist:\t{:?}", lowest_dist);
                    }
                    //println!("With text:\t{:?}\t{:?}", l1, l2);
                }
            }
        }
    }
    manhattan_dist((0,0), (6,6))
}

#[test]
fn test_manhattan_dist() {
    assert_eq!(manhattan_dist((0,0), (6,6)), 12);
}

#[test]
fn test_do_lines_touch() {
    let point1 = Point { x: 0, y: 0 };
    let point2 =  Point { x: 6, y: 6 };
    let line1 = Line { start :point1, end : point2};
    let point3 = Point { x: 6, y: 0 };
    let point4 =  Point { x: 0, y: 6 };
    let line2 = Line { start :point3, end : point4};

    assert_eq!(do_lines_touch(&line1, &line2), false);

}