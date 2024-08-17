trait MaterialCost {
    fn get_materail_cost(&self, sq_meter: f64) -> f64;
}

struct Carpet(f64);
struct Tile(f64);
struct Wood(f64);

impl MaterialCost for Carpet {
    fn get_materail_cost(&self, sq_meter: f64) -> f64 {
        return self.0 * sq_meter;
    }
}

impl MaterialCost for Tile {
    fn get_materail_cost(&self, sq_meter: f64) -> f64 {
        return self.0 * sq_meter;
    }
}

impl MaterialCost for Wood {
    fn get_materail_cost(&self, sq_meter: f64) -> f64 {
        return self.0 * sq_meter;
    }
}

fn get_total_material_cost(material_list: &Vec<Box<dyn MaterialCost>>, size: f64) -> f64 {
    return material_list
        .iter()
        .map(|material| material.get_materail_cost(size))
        .sum();
}

fn main() {
    let carpet = Box::new(Carpet(100.0));

    let tile = Box::new(Tile(60.0));

    let wood = Box::new(Wood(150.0));

    let material_list: Vec<Box<dyn MaterialCost>> = vec![carpet, tile, wood];

    println!(
        "Total Material Cost : {:?}",
        get_total_material_cost(&material_list, 100.0)
    );
}
