use colored::Colorize;

pub fn init() -> fern::Dispatch {
  fern::Dispatch::new()
    .format(move |out, message, record| {
      let line = record.line().unwrap_or(0);
      let level = record.level();
      let tip = (format_args!(
        "{} {} {}:{}{}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        level,
        record.target(),
        record.file().unwrap_or(""),
        if line > 0 {
          format!(":{}", line)
        } else {
          "".to_string()
        }
      ))
      .to_string();
      {
        use log::Level::{Debug, Error, Info, Trace, Warn};
        out.finish(format_args!(
          "{}\n{}\n",
          message,
          match level {
            Error => tip.bright_red(),
            Warn => tip.bright_yellow(),
            Info => tip.white(),
            Debug => tip.green(),
            Trace => tip.bright_black(),
          },
        ))
      }
    })
    .level(log::LevelFilter::Info)
    .chain(std::io::stdout())
  // .chain(fern::log_file("output.log")?)
  //.apply()
  //.unwrap()
}
