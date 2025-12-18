use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use bimap::BiMap;
use crate::util::*;

pub type Point3 = (f64, f64, f64);
pub type UPoint3 = (usize, usize, usize);

pub async fn advent_1(data: String, _test: bool) -> usize {
    let boxes = parse(&data);
    let mut circuits = Vec::<HashSet<usize>>::new();
    let mut shortests = Vec::<(UPoint3, UPoint3, f64)>::new();
    let mut seen = HashSet::<(UPoint3, UPoint3)>::new();
    let mut nodemap = HashMap::<UPoint3, usize>::new();

    for (i,a) in boxes.clone().iter().enumerate() {
        nodemap.insert(convert(*a), i);
        for b in boxes.iter() {
            if a.0 == b.0 && a.1 == b.1 && a.2 == b.2 {
                continue;
            }
            let ua = convert(*a);
            let ub = convert(*b);
            if seen.contains(&(ua,ub)) || seen.contains(&(ub,ua)) {
                continue;
            }
            seen.insert((ua,ub));
            shortests.push((ua,ub,dist(*a,*b)))

        }
    }

    shortests.sort_by(|a,b| {
        a.2.partial_cmp(&b.2).unwrap()
    });

    for (a,b,_) in shortests.iter().take(std::cmp::min(1000,shortests.len())) {
        let a = nodemap.get(a).unwrap();
        let b = nodemap.get(b).unwrap();
        
        if let Some(pos) = circuits.iter().position(|el| {
            el.contains(a) || el.contains(b)
        }) {
            let opta = circuits.iter().position(|el| el.contains(a));
            let optb = circuits.iter().position(|el| el.contains(b));
            if let Some(posa) = opta && let Some(posb) = optb && posa != posb {

                    let circuitb = circuits.get(posb).unwrap().clone();
                    let circuit = circuits.get_mut(posa).unwrap();
                    // dbg!(format!("merging {circuitb:?} and {circuit:?} for {a} and {b}, dist {dist}"));
                    circuit.extend(circuitb);
                    circuits.remove(posb);
            }
            else {
                let circuit = circuits.get_mut(pos).unwrap();
                circuit.insert(*a);
                circuit.insert(*b);
                // dbg!(format!("added {a} and {b} to {circuit:?} with distance {dist}"));
            }
            
        }
        else {
            let mut circuit = HashSet::new();
            // dbg!(format!("adding {a} and {b} to new circuit with distance {dist}"));
            circuit.insert(*a);
            circuit.insert(*b);
            circuits.push(circuit);
        }
    }
    // dbg!(&circuits);
    circuits.iter().sorted_by(|a,b| b.len().cmp(&a.len())).take(3).fold(1, |i,circuit| i * circuit.len())
}

pub async fn advent_2(data: String, _test: bool) -> usize {
    let boxes = parse(&data);
    let mut shortests = Vec::<(UPoint3, UPoint3, f64)>::new();
    let mut seen = HashSet::<(UPoint3, UPoint3)>::new();
    let mut nodemap = BiMap::<UPoint3, usize>::new();

    for (i,a) in boxes.clone().iter().enumerate() {
        nodemap.insert(convert(*a), i);
        for b in boxes.iter() {
            if a.0 == b.0 && a.1 == b.1 && a.2 == b.2 {
                continue;
            }
            let ua = convert(*a);
            let ub = convert(*b);
            if seen.contains(&(ua,ub)) || seen.contains(&(ub,ua)) {
                continue;
            }
            seen.insert((ua,ub));
            shortests.push((ua,ub,dist(*a,*b)))
            
        }
    }
    
    shortests.sort_by(|a,b| {
        a.2.partial_cmp(&b.2).unwrap()
    });
    let mut circuits = boxes.iter().map(|junction| {
        HashSet::from([*nodemap.get_by_left(&convert(*junction)).unwrap()])
    }).collect_vec();
    let mut i = 0;
    let mut last_a = 0;
    let mut last_b = 0;
    while circuits.len() > 1 {
        let (a,b,_) = shortests[i];
    
    // for (a,b,_) in shortests.iter().take(std::cmp::min(1000,shortests.len())) {
        let a = nodemap.get_by_left(&a).unwrap();
        let b = nodemap.get_by_left(&b).unwrap();
        
        if let Some(pos) = circuits.iter().position(|el| {
            el.contains(a) || el.contains(b)
        }) {
            let opta = circuits.iter().position(|el| el.contains(a));
            let optb = circuits.iter().position(|el| el.contains(b));
            if let Some(posa) = opta && let Some(posb) = optb && posa != posb {

                    let circuitb = circuits.get(posb).unwrap().clone();
                    let circuit = circuits.get_mut(posa).unwrap();
                    // dbg!(format!("merging {circuitb:?} and {circuit:?} for {a} and {b}, dist {dist}"));
                    circuit.extend(circuitb);
                    circuits.remove(posb);
            }
            else {
                let circuit = circuits.get_mut(pos).unwrap();
                circuit.insert(*a);
                circuit.insert(*b);
                // dbg!(format!("added {a} and {b} to {circuit:?} with distance {dist}"));
            }
            
        }
        else {
            let mut circuit = HashSet::new();
            // dbg!(format!("adding {a} and {b} to new circuit with distance {dist}"));
            circuit.insert(*a);
            circuit.insert(*b);
            circuits.push(circuit);
        }
        last_a = *a;
        last_b = *b;
        i += 1;
    }
    // dbg!(&circuits);
    // circuits.iter().sorted_by(|a,b| b.len().cmp(&a.len())).take(3).fold(1, |i,circuit| i * circuit.len())
    nodemap.get_by_right(&last_a).unwrap().0 * nodemap.get_by_right(&last_b).unwrap().0
}

fn parse(data: &str) -> Vec<Point3> {
    data.split('\n').map(|line| {
        line.split(',').map(|val| {
            val.parse::<f64>().unwrap()
        }).collect_tuple().unwrap()
    }).collect_vec()
}

fn dist(a: Point3, b: Point3) -> f64 {
    ((a.0 - b.0).powf(2.) + (a.1 - b.1).powf(2.) + (a.2 - b.2).powf(2.)).sqrt()
}

fn convert(a: Point3) -> UPoint3 {
    (a.0 as usize, a.1 as usize, a.2 as usize)
}