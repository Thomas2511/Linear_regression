extern crate linear_regression;
extern crate csv;

type Row = (f64, f64);

fn estimate_price ((t0, t1): (f64, f64), km: f64) -> f64 {
    t0 + (t1 * km)
}

fn calc_theta ((tmp0, tmp1): (f64, f64), lr: f64, values: &Vec<Row>) -> (f64, f64) {
    let (t0, t1) = values.iter().fold((0.0, 0.0), |acc, &(km, p)| {
        let estimated_price = estimate_price((tmp0, tmp1), km) - p;
        (acc.0 + estimated_price, acc.1 + (estimated_price * km))
    });
    (tmp0 - (t0 / values.len() as f64 * lr), tmp1 - (t1 / values.len() as f64 * lr))
}

fn main () {
    let mut rdr = csv::Reader::from_file("Data.csv").unwrap().has_headers(true);
    let rows = rdr.decode().collect::<csv::Result<Vec<Row>>>().unwrap();
    let max_x = rows.iter().max_by_key(|row| row.0 as i32).unwrap().0;
    let min_x = rows.iter().min_by_key(|row| row.0 as i32).unwrap().0;
    let values: Vec<Row> = rows.iter().map(|x| ((x.0 - min_x) / (max_x - min_x), x.1)).collect();
    let (t0, t1) = (0..10).fold((0.0, 0.0), |acc, _| calc_theta(acc, 0.1, &values));
    let mut wtr = csv::Writer::from_file("Theta.csv").unwrap();
    wtr.encode((t0, t1, max_x, min_x)).ok();
}
