use num_cpus;

fn get_cpu_num() {

    let num = num_cpus::get();
    println!("logic cpu core number is {num}");
}

// cargo run --bin about_sys_info
fn main() {

    get_cpu_num();
}