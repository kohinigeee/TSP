#[allow(non_snake_case)]
#[allow(unused)]
#[allow(non_camel_case_types)]
#[allow(nonstandard_style)]

use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
use std::collections::{BTreeMap};
use log::{info};
use anyhow::{self, Context};
use thiserror::{Error};
use rand::{Rng};

mod modules;

use modules::tspinstance::{TspInstance, ProblemPath};
use modules::point::Point;
use modules::construct;
use modules::tsp::*;
use modules::opttour::OptTour;
use modules::segmenttree::SegmentTree;

use crate::modules::construct::{Insertion, insertion_demo};


fn main() -> anyhow::Result<()>{

    let mut opt_scores : BTreeMap<String, i64> = BTreeMap::new();
    opt_scores.insert("pr1002".to_string(),259045);
    opt_scores.insert("fnl4461".to_string(),182566);
    opt_scores.insert("brd14051".to_string(),469385);
    opt_scores.insert("pla33810".to_string(), 66048945);


    let problem_name = "pla33810".to_string();
    let problem : ProblemPath = ProblemPath::new(&problem_name);
    let fpath : String = problem.getInstPath(); 
    let inst : TspInstance = TspInstance::fromFile(&fpath)?;
    let tsp : Tsp = Tsp::from(&inst);

    println!("Problem size = {}\n", tsp.size);

    // let optpath = problem.getOptPath();
    // let opttour : OptTour = OptTour::fromFile(&optpath, &tsp).unwrap();
    let opt_score = *opt_scores.get(&problem_name).unwrap();
    // println!("opt score = {}\n", opttour.score);
    // println!("{:?}\n", opttour);

    let start = Instant::now();
    let ord_near = construct::nearest(&tsp, 0);
    // let ord_near = construct::nearest_all(&tsp);
    let duration = start.elapsed();
    let near_score = tsp.calcScore(&ord_near).unwrap();
    println!("Nearest score = {}", near_score);
    println!("Nearest score / Opt score = {}", near_score as f64 / opt_score as f64);
    println!("Nearest time = {}", duration.as_millis());
    println!("\n");

    // // let ord = construct::Kruskal(&tsp);
    // let krus_score = tsp.calcScore(&ord).unwrap();
    // println!("Kruskal score = {}", krus_score); 
    // println!("Kruskal score / Opt score = {}\n", ( krus_score as f64 / opttour.score as f64));


    let start = Instant::now();
    // let psed_ord = construct::psedo_nearest(&tsp,500, 500, 0);
    let psed_ord = construct::psedo_nearest_all(&tsp, 500, 500);
    // let psed_ord = construct::random_psedo(&tsp, 600, 600, 10);
    let duration = start.elapsed();
    let psed_score = tsp.calcScore(&psed_ord).unwrap();
    println!("Psed Nearest score = {}", psed_score);
    println!("Psed Nearest time = {}", duration.as_millis());
    println!("Psed score / Opt score = {}", ( psed_score as f64 / opt_score as f64));
    // println!("Psed score / Nerarest score = {}", ( psed_score as f64 / near_score as f64));
    // println!("ord = {:?}", psed_ord);
    println!();

    let near2_ord = construct::nearest(&tsp, psed_ord[0]);
    let near2_score = tsp.calcScore(&near2_ord).unwrap();
    println!("Nearest2 score = {}", near2_score);
    println!("Nearest2 score / Opt score = {}", (near2_score as f64 / opt_score as f64));
    println!();

    println!("Nearest score = {}", near_score);
    println!("Nearest score / Opt score = {}", near_score as f64 / opt_score as f64);
    println!("Nearest time = {}", duration.as_millis());
    println!();

    // let mut gosa_near = 0.0;
    // let mut time_near = 0;

    // let mut gosa_psed1 = 0.0;
    // let mut time_psed1 = 0;

    // let mut gosa_psed2= 0.0;
    // let mut time_psed2 = 0;

    // let mut gosa_psed3= 0.0;
    // let mut time_psed3 = 0;

    // let mut gosa_psed4= 0.0;
    // let mut time_psed4 = 0;

    // let mut rng = rand::thread_rng();
    // let n = 10;
    // for i in 0..n {
    //     let no = rng.gen_range(0, tsp.size-1);
    //     println!("i = {}", i);

    //     let start = Instant::now();
    //     let near_ord = construct::nearest(&tsp, no);
    //     let duration = start.elapsed();
    //     let near_score = tsp.calcScore(&near_ord).unwrap();
    //     gosa_near += ( near_score as f64 / opt_score as f64);
    //     time_near += duration.as_millis();

    //     let start = Instant::now();
    //     let psed_ord4= construct::psedo_nearest(&tsp, 100, 100, no);
    //     let duration = start.elapsed();
    //     let psed_score4= tsp.calcScore(&psed_ord4).unwrap();
    //     gosa_psed4 += ( psed_score4 as f64 / opt_score as f64);
    //     time_psed4 += duration.as_millis();

    //     let start = Instant::now();
    //     let psed_ord1= construct::psedo_nearest(&tsp, 500, 500, no);
    //     let duration = start.elapsed();
    //     let psed_score1= tsp.calcScore(&psed_ord1).unwrap();
    //     gosa_psed1 += ( psed_score1 as f64 / opt_score as f64);
    //     time_psed1 += duration.as_millis();


    //     let start = Instant::now();
    //     let psed_ord2= construct::psedo_nearest(&tsp, 1000, 1000, no);
    //     let duration = start.elapsed();
    //     let psed_score2= tsp.calcScore(&psed_ord2).unwrap();
    //     gosa_psed2 += ( psed_score2 as f64 / opt_score as f64);
    //     time_psed2 += duration.as_millis();

    //     let start = Instant::now();
    //     let psed_ord3= construct::psedo_nearest(&tsp, 1500, 1500, no);
    //     let duration = start.elapsed();
    //     let psed_score3= tsp.calcScore(&psed_ord3).unwrap();
    //     gosa_psed3 += ( psed_score3 as f64 / opt_score as f64);
    //     time_psed3 += duration.as_millis();
    // }

    // println!("Nearest gosa = {}", gosa_near/(n as f64));
    // println!("Nearest time = {}", time_near/n);
    // println!();

    // println!("100 x 100 gosa = {}", gosa_psed4/(n as f64));
    // println!("100 x 100 time = {}", time_psed4/n);
    // println!();
    

    // println!("500 x 500 gosa = {}", gosa_psed1/(n as f64));
    // println!("500 x 500 time = {}", time_psed1/n);
    // println!();
    
    // println!("1000 x 1000 gosa = {}", gosa_psed2/(n as f64));
    // println!("1000 x 1000 time = {}", time_psed2/n);
    // println!();

    // println!("1500 x 1500 gosa = {}", gosa_psed3/(n as f64));
    // println!("1500 x 1500 time = {}", time_psed3/n);
    // println!();
    Ok(())
}
