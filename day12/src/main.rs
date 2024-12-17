/*
 Advent of Code 2024 Day 12

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
use utils::{get_challenge_config, read_puzzle_input, ChallengePart};

fn main() {
    let challenge_config = get_challenge_config();

    let puzzle_map = parse_plots(challenge_config.is_test);

    let regions = get_regions(puzzle_map);
    // println!("regions: {:?}", regions);
    
    match challenge_config.part {
      ChallengePart::One => println!("The total price for fencing the regions is: {}", calculate_price(&regions)),
      ChallengePart::Two => println!("Not implemented yet"),
    }
}

fn parse_plots(is_test: bool) -> Vec<Vec<Plot>> {
  let mut plot_map = Vec::new();

  let file_path = if is_test { "./src/example_input.txt" } else { "./src/puzzle_input.txt" };

  for (row, pattern) in read_puzzle_input(file_path).enumerate() {
    let mut plot_row = vec![];
    for (column, plant) in pattern.chars().enumerate() {
      plot_row.push(Plot {
        plant,
        row,
        column
      });

    }
    plot_map.push(plot_row);
  }

  // println!("Plot map: {:?}", plot_map);

  plot_map
}

#[derive(Debug, Copy, Clone)]
struct Plot {
  plant: char,
    row: usize,
    column: usize
}

impl Plot {
  fn is_adjacent(&self, other_plot: &Plot) -> bool {
    (self.row as i32 - 1 == other_plot.row as i32 && self.column == other_plot.column) || // is on top of a plot
    (self.row + 1 == other_plot.row && self.column == other_plot.column) || // is below a plot
    (self.row == other_plot.row && self.column as i32 - 1 == other_plot.column as i32) || // is to the left of a plot
    (self.row == other_plot.row && self.column + 1 == other_plot.column) 
  }
}

#[derive(Debug, Clone)]
struct Region {
  id: usize,
  plots: Vec<Plot>,
  area: i32,
  perimeter: i32
}

impl Region {
  fn price(&self) -> i32 {
    self.area * self.perimeter
  }

  fn plant(&self) -> char {
    self.plots[0].plant
  }

  fn contains(&self, plot: &Plot) -> bool {
    if plot.plant != self.plant() {
      return false;
    }
    self.plots.iter().any(|region_plot| region_plot.is_adjacent(plot))
  }

  fn add_plot(&mut self, plot: &Plot) {
    let adjacent_plots = self.plots
      .iter()
      .filter(|region_plot| region_plot.is_adjacent(plot))
      .count();

    self.perimeter += 4 - 2 * adjacent_plots as i32;
    self.area += 1;
    self.plots.push(*plot);
    // println!("after adding plot region area: {} region perimeter: {}", self.area, self.perimeter);
  }

  fn merge_region(&mut self, other_region: &Region) {
    for plot in other_region.clone().plots {
      self.add_plot(&plot);
    }
  }
}

fn get_regions(plot_map: Vec<Vec<Plot>>) -> Vec<Region> {
  let mut regions: Vec<Region> = Vec::new();

  for plot_row in plot_map {
    let mut adjacent_plots: Vec<Plot> = Vec::new();

    for plot in plot_row {
      if adjacent_plots.len() > 0 && adjacent_plots[0].plant != plot.plant {
        add_new_region(&mut regions, adjacent_plots);
        adjacent_plots = Vec::new();
      }
      
      let mut new_regions = Vec::new();
      let mut contained_regions = Vec::new();
      for region in regions.clone(){
        if region.contains(&plot) {
          contained_regions.push(region);
        } else {
          new_regions.push(region);
        }
      }

      if contained_regions.len() > 0 {
        let mut new_region = contained_regions[0].clone();
        new_region.add_plot(&plot);
        for plot in adjacent_plots {
          new_region.add_plot(&plot);
        }

        for idx in 1..contained_regions.len() {
          new_region.merge_region(&contained_regions[idx]);
        }

        new_regions.push(new_region);
        regions = new_regions.clone();

        adjacent_plots = Vec::new();
      } else {
        adjacent_plots.push(plot);
      }
    }
    
    if adjacent_plots.len() > 0 {
      add_new_region(&mut regions, adjacent_plots);
    }
  }

  regions
}

fn add_new_region(regions: &mut Vec<Region>, plots: Vec<Plot>) {
    let mut new_region = Region {
        id: regions.len(),
        area: 0,
        perimeter: 0,
        plots: Vec::new()
      };

    for plot in plots {
        new_region.add_plot(&plot);
      }

    regions.push(new_region);
}

fn calculate_price(regions: &Vec<Region>) -> i32 {
  let mut total_price = 0;
  for region in regions {
    println!("Region plant: {} area: {} perimeter {}", region.plant(), region.area, region.perimeter);
    total_price += region.price();
  }
  total_price
}