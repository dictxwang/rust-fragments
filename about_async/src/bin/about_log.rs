
use log::{debug, error, info, warn, Level};
use log::LevelFilter;
use std::fs::File;
use anyhow::Result;

fn use_env_logger() {

    use std::env::set_var;
    use chrono::Local;
    use std::io::Write;
    use env_logger::fmt::Color;

    // set_var("RUST_LOG", "debug");  // 通过环境变量设置日志级别
    // env_logger::init();  // 默认级别是error级别
    // env_logger::builder().filter_level(log::LevelFilter::Debug).init();  // 设置日志级别为Debug

    // 修改writln!宏，修改输出格式
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "trace");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {

            // 设置日志颜色
            let level_color = match record.level() {
                Level::Error => Color::Red,
                Level::Warn => Color::Yellow,
                Level::Info => Color::Green,
                Level::Debug | Level::Trace => Color::Cyan,
            };
            let mut level_style = buf.style();
            level_style.set_color(level_color).set_bold(true);

            let mut normal_style = buf.style();
            normal_style.set_color(Color::White).set_dimmed(true);
            
            writeln!(
                buf,
                "{} {} [{}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),  // 不设置日期格式，默认按照0时区时间输出
                level_style.value(record.level()),
                normal_style.value(record.module_path().unwrap_or("<unnamed>")),
                record.line().unwrap_or(0),  // 输出行号
                &record.args()
            )
        })
        .filter_module("bin", log::LevelFilter::Error)  // 设置模块过滤级别
        .init();
    debug!("this is a debug {}", "'some message'");
    error!("this is printed by default");
    info!("this is a info {}", "'some message'");
    warn!("this is a warn {}", "'some message'");
}

fn use_env_logger_file() {

    use chrono::Local;
    use std::io::Write;

    let target = Box::new(File::create("/tmp/rust_envlogger.log").expect("something wrong"));let target = Box::new(File::create("/tmp/rust.log").expect("something wrong"));
    let _target2 = Box::new(File::create("/tmp/rust_envlogger2.log").expect("something wrong"));
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(target))  // 通过pipe输出日志到file
        // .target(env_logger::Target::Pipe(_target2))  // 尝试输出到多个target，之前的设置会被覆盖
        .filter(None, log::LevelFilter::Info)
        .init();

    info!("this is a info {}", "'some messages'");
    error!("this is a error {}", "'some messages'");
}

fn use_log4rs() {
}

fn use_fern() -> Result<()> {

    use fern::colors::{Color, ColoredLevelConfig};
    let colors = ColoredLevelConfig {
        trace: Color::Cyan,
        debug: Color::Magenta,
        info: Color::Green,
        warn: Color::Red,
        error: Color::BrightRed,
        ..ColoredLevelConfig::new()
    };

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                colors.color(record.level()),
                message
            ))
        })
        .chain(std::io::stdout())
        .chain(fern::log_file("/tmp/rust_fern.log")?)
        .level(log::LevelFilter::Debug)
        .level_for("bin", log::LevelFilter::Error)
        .apply()?;

    debug!("this is a debug {}", "'some message'");
    info!("this is a debug {}", "'some message'");
    warn!("this is a debug {}", "'some message'");
    warn!("this is a debug {}", "'some message'");

    Ok(())
}

// cargo run --bin about_log
fn main() {

    // use_env_logger();
    // use_env_logger_file();
    use_fern();
}