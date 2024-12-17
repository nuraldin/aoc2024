/*
 Advent of Code 2024 Day 12: Garden Groups

 The input is a map of garden plots. Each garden plot grows a signle type of plant indicated by a single letter on the map.
 The plots form regions (when the same plant is touching horizontally or vertically with another plot)
 Each region has an area and a perimeter.
 An area is just the amount of plots of the region. The perimeter is only the sides that touch another region (our map side)
 Regions can appear within regions. And plants of the same type can appear in multiple separate regions.
 The `price` of fence required for a region is found by multiplying the region are by its perimeter.
 The `total price` of fencing all regions is the sum of all fence price of every region.

 Part one:

 What is the total price of fencing all regions on your map?

 Part two:

 To calculate the perimeter this time you use the sides of each region has.
 What would be the new total price?

 Solution: 

*/
use utils::{get_challenge_config, read_puzzle_input, ChallengePart, Coordinate, Direction, TopographicMap};

fn main() {
    let challenge_config = get_challenge_config();

    let puzzle_map = parse_plots(challenge_config.is_test);

    let regions = get_regions(puzzle_map);
    println!("regions: {:?}", regions);
    
    match challenge_config.part {
      ChallengePart::One => println!("The total price for fencing the regions is: {}", calculate_price(&regions)),
      ChallengePart::Two => println!("Not implemented yet"),
    }
}

fn parse_plots(is_test: bool) -> TopographicMap<char> {
  let mut plot_map = TopographicMap::new();

  let file_path = if is_test { "./src/example_input.txt" } else { "./src/puzzle_input.txt" };

  for (x, plots) in read_puzzle_input(file_path).enumerate() {
    for (y, plant) in plots.chars().enumerate() {
      plot_map.insert(Coordinate { x: x as i32, y: y as i32 }, plant);
    }
  }

  // println!("Plot map: {:?}", plot_map);
  plot_map
}

#[derive(Debug, Clone)]
struct Region {
  plant: char,
  plots: Vec<Coordinate>,
  area: i32,
  perimeter: i32
}

impl Region {
  fn new(plant: char, coordinate: &Coordinate) -> Self {
    Self {
      plant: plant,
      plots: vec![coordinate.clone()],
      area: 1,
      perimeter: 4,
    }
  }

  fn price(&self) -> i32 {
    self.area * self.perimeter
  }

  fn is_adjacent(&self, coordinate: &Coordinate) -> bool {
    for direction in Direction::to_vec() {
      let next = coordinate.add_delta(direction);

      if self.plots.contains(&next) {
        return true;
      } 
    }

    false
  }

  fn add_plot(&mut self, plot: &Coordinate) {
    let adjacent_plots = self.plots
      .iter()
      .filter(|other_plot| other_plot.is_adjacent(plot))
      .count();

    self.perimeter += 4 - 2 * adjacent_plots as i32;
    self.area += 1;
    self.plots.push(plot.clone());
    // println!("after adding plot region area: {} region perimeter: {}", self.area, self.perimeter);
  }

  fn merge_region(&mut self, other_region: &Region) {
    for plot in other_region.clone().plots {
      self.add_plot(&plot);
    }
  }
}

fn get_regions(plot_map: TopographicMap<char>) -> Vec<Region> {
  let mut regions: Vec<Region> = Vec::new();

  for (coordinate, plant) in plot_map {
    // println!("Regions: {:?}", regions);
    // println!("plant: {plant}, coordinate: {coordinate:?}");

    if regions.is_empty() {
      regions.push(Region::new(plant, &coordinate));
      continue;
    }

    let mut other_regions = Vec::new();
    let mut adjacent_regions = Vec::new();
    for region in regions.clone() {
      if region.plant == plant && region.is_adjacent(&coordinate) {
        adjacent_regions.push(region.clone());
      } else {
        other_regions.push(region.clone());
      }
    } 

    if !adjacent_regions.is_empty() {
      // println!("adjacent regions: {:?}\nother regions: {:?}\n", adjacent_regions, other_regions);
      let (new_region, rest) = adjacent_regions.split_first_mut().unwrap();
      new_region.add_plot(&coordinate);

      for region in rest {
        new_region.merge_region(region);
      }

      other_regions.push(new_region.clone());
      regions = other_regions.clone();
    } else {
      regions.push(Region::new(plant, &coordinate));
    }
  }

  regions
}

fn calculate_price(regions: &Vec<Region>) -> i32 {
  let mut total_price = 0;
  for region in regions {
    println!("Region plant: {} area: {} perimeter {}", region.plant, region.area, region.perimeter);
    total_price += region.price();
  }
  total_price
}