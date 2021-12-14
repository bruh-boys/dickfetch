use colored::Colorize;
use sysinfo::SystemExt;

mod ascii;
mod bench;
mod rank;

fn print_score(_score: i64) {
    println!("-o current: {} points", _score.to_string().green().bold());
}

fn main() {
    let mut _score: i64 = 0;
    println!("-o {}", "benching your dick...".green().bold());
    _score += bench::bench::Bench::case1();
    print_score(_score);
    println!("-> {}", "running case 2");
    _score += bench::bench::Bench::case2();
    print_score(_score);

    let mut _ascii = ascii::ascii::Ascii::new(_score);
    let mut _ranked_enum: rank::rank::BenchMarkRank = _ascii.bench_score_to_rank_enum();
    let mut _ranked_string: String = _ranked_enum.to_string();
    let mut _ranked_ascii: String = _ascii.rank_to_ascii(_ranked_enum);

    println!("{}", _ranked_ascii);

    // get string of BenchMarkRank
    println!("-o bench comment: {}", _ranked_string.green().bold());

    let system = sysinfo::System::new_all();
    println!(
        "-o memory: {} / {}",
        system.get_used_memory(),
        system.get_total_memory()
    );
    println!("-o System name:             {:?}", system.get_name());
    println!("-o System host name:        {:?}", system.get_host_name());
}
