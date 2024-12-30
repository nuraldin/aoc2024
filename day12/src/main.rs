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

 For the first solutions I map the input into a hashmap of coordinates and plants. From this one I start creating regions and merging existing ones if necessary.
 For the second part found a solution in internet that made me realize I just had to check for each element its four corners.
 if any corner is an inner or outer edge you add up a point, as the sum of all edges is the number of sides in a polygon.
 That made the calculation easy. Still the first part of making the regions is not very performant but it works. 
 It is not performant specially because I'm trying to merge regions every time I check for an item and that is costly.

*/
use utils::{ChallengeConfig, ChallengePart, Coordinate, Direction, TopographicMap};

fn main() {
    let challenge_config = ChallengeConfig::get();

    let puzzle_map = parse_plots(&challenge_config);

    let regions = get_regions(puzzle_map);
    // println!("regions: {:?}", regions);
    
    match challenge_config.part {
      ChallengePart::One => println!("The total price for fencing the regions is: {}", calculate_price(&regions, challenge_config.part)),
      ChallengePart::Two => println!("The total price for fencing the regions with discount is: {}", calculate_price(&regions, challenge_config.part)),
    }
}

fn parse_plots(config: &ChallengeConfig) -> TopographicMap<char> {
  let mut plot_map = TopographicMap::new();

  for (x, plots) in config.read_puzzle_input(None).enumerate() {
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
  perimeter: i32,
}

impl Region {
  fn new(plant: char, coordinate: &Coordinate) -> Self {
    Self {
      plant,
      plots: vec![coordinate.clone()],
      area: 1,
      perimeter: 4,
    }
  }

  fn price(&self) -> i32 {
    self.area * self.perimeter
  }

  fn price_with_discount(&self) -> i32 {
    self.area * self.sides()
  }

  fn sides(&self) -> i32 {
    let mut sides = 0;

    for plot in &self.plots {
      let surrounding_plots = get_surrounding_plots(plot.clone(), &self.plots);
      sides += get_side_delta(plot.clone(), &surrounding_plots);
    }

    // println!("plant: {}, plots: {:?}, sides: {}", self.plant, self.plots, sides);


    sides
  }

  fn is_adjacent(&self, coordinate: &Coordinate) -> bool {
    for direction in Direction::iter() {
      let next = coordinate.add_delta(&direction);

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

fn calculate_price(regions: &Vec<Region>, challenge_part: ChallengePart) -> i32 {
  let mut total_price = 0;
  for region in regions {
    // println!("Region plant: {} area: {} perimeter: {}  sides: {}", region.plant, region.area, region.perimeter, region.sides());
    total_price += match challenge_part {
      ChallengePart::One => region.price(),
      ChallengePart::Two => region.price_with_discount(),
    }
  }
  total_price
}

fn get_surrounding_plots(plot: Coordinate, other_plots: &Vec<Coordinate>) -> Vec<Coordinate> {
  let deltas = [
    (0,1),    // right
    (0, -1),  // left
    (1,0),   // down
    (-1, 0),  // up
    (1, 1),    // diagonal down right
    (1, -1), // diagonal down left
    (-1,-1),  // diagonal up left
    (-1, 1),  // diagonal up right
  ];

  let mut surrounding_plot = vec![];
  for delta in deltas {
    let next_coordinate = Coordinate { x: plot.x + delta.0, y: plot.y + delta.1 };
    if other_plots.contains(&next_coordinate) && next_coordinate != plot {
      surrounding_plot.push(next_coordinate);
    }
  }

  surrounding_plot
}

fn get_side_delta(plot: Coordinate, surrounding_plots: &Vec<Coordinate>) -> i32 {
  let mut corners = 0;
  for corner in [Direction::UpRight, Direction::UpLeft, Direction::DownLeft, Direction::DownRight] {
    if plot.is_outer_edge(&corner, &surrounding_plots) || plot.is_inner_edge(&corner, &surrounding_plots) {
      corners += 1;
    }
  }

  corners
}