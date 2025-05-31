use chrono::Local;
use log::{debug, error, info, trace, warn, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, rolling_file::RollingFileAppender},
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf, time::Duration};
use std::sync::Arc;
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
struct LogConfig {
    root_level: String,
    loggers: Vec<LoggerConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoggerConfig {
    name: String,
    level: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        LogConfig {
            root_level: "DEBUG".to_string(),
            loggers: vec![
                LoggerConfig {
                    name: "app::backend".to_string(),
                    level: "INFO".to_string(),
                }
            ],
        }
    }
}

pub struct LogManager {
    config_path: PathBuf,
}

impl LogManager {
    pub fn new() -> Self {
        // 确保配置目录存在
        let config_dir = PathBuf::from("config");
        fs::create_dir_all(&config_dir).unwrap();
        
        // 配置文件路径
        let config_path = config_dir.join("log4rs.yaml");
        
        // 如果配置文件不存在，创建默认配置
        if !config_path.exists() {
            let default_config = LogConfig::default();
            let yaml = serde_yaml::to_string(&default_config).unwrap();
            fs::write(&config_path, yaml).unwrap();
        }
        
        // 初始化日志系统
        Self::init_logger(&config_path);
        
        LogManager {
            config_path,
        }
    }
    
    fn create_log_config(config_path: &PathBuf) -> Config {
        // 读取日志级别配置
        let log_config: LogConfig = serde_yaml::from_str(
            &fs::read_to_string(config_path).unwrap()
        ).unwrap();
        
        // 获取基础路径
        let log_base_path = get_log_base_path();
        
        // 创建控制台输出
        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "{d(%Y-%m-%d %H:%M:%S.%3f)} {l} [{I}] [{t}] {m}{n}"
            )))
            .build();

        // 设置当前日期路径
        let current_date = Local::now().format("%Y-%m-%d").to_string();
        let log_dir = log_base_path.join(&current_date);
        fs::create_dir_all(&log_dir).unwrap();

        // 设置滚动文件策略
        let window_roller = FixedWindowRoller::builder()
            .build(
                &format!(
                    "{}/{}/{}_{{}}.log",
                    log_base_path.to_string_lossy(),
                    current_date,
                    current_date
                ),
                5,
            )
            .unwrap();

        let size_trigger = SizeTrigger::new(10 * 1024 * 1024); // 10MB
        let compound_policy = CompoundPolicy::new(
            Box::new(size_trigger),
            Box::new(window_roller),
        );

        // 创建滚动文件输出
        let rolling_file = RollingFileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "{d(%Y-%m-%d %H:%M:%S.%3f)} - {l} - [{I}] - [{t}] - {m}{n}"
            )))
            .build(
                log_dir.join(format!("{}.log", current_date)),
                Box::new(compound_policy),
            )
            .unwrap();

        // 创建配置构建器
        let mut builder = Config::builder()
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(str_to_level_filter(&log_config.root_level))))
                    .build("stdout", Box::new(stdout)),
            )
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(str_to_level_filter(&log_config.root_level))))
                    .build("rolling_file", Box::new(rolling_file)),
            );

        // 添加所有logger配置
        for logger_config in &log_config.loggers {
            builder = builder.logger(
                Logger::builder()
                    .appender("rolling_file")
                    //.appender("stdout")
                    .additive(false)
                    .build(
                        &logger_config.name,
                        str_to_level_filter(&logger_config.level),
                    ),
            );
        }

        // 构建最终配置
        builder
            .build(
                Root::builder()
                    //.appender("stdout")
                    .appender("rolling_file")
                    .build(str_to_level_filter(&log_config.root_level)),
            )
            .unwrap()
    }
    
    fn init_logger(config_path: &PathBuf) {
        let config = Self::create_log_config(config_path);
        log4rs::init_config(config).unwrap();
    }
    
    pub fn start_config_monitor(log_manager: Arc<LogManager>) {
        thread::spawn(move || {
            let mut last_modified = fs::metadata(&log_manager.config_path)
                .unwrap()
                .modified()
                .unwrap();
            
            loop {
                thread::sleep(Duration::from_secs(10)); // 每10秒检查一次
                
                if let Ok(metadata) = fs::metadata(&log_manager.config_path) {
                    if let Ok(modified) = metadata.modified() {
                        if modified > last_modified {
                            info!("检测到日志配置文件变更，重新加载配置");
                            // 创建新配置并初始化
                            let config = Self::create_log_config(&log_manager.config_path);
                            log4rs::init_config(config).unwrap();
                            last_modified = modified;
                        }
                    }
                }
            }
        });
    }

    pub fn reload_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = Self::create_log_config(&self.config_path);
        log4rs::init_config(config)?;
        Ok(())
    }
}

fn get_process_name() -> String {
    env::current_exe()
        .ok()
        .and_then(|pb| pb.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "unknown".to_string())
}

fn get_log_base_path() -> PathBuf {
    let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| {
        env::var("HOME").unwrap_or_else(|_| ".".to_string()) + "/.config"
    });
    
    PathBuf::from(&local_app_data)
        .join(get_process_name())
        .join("logs")
}

fn str_to_level_filter(level: &str) -> LevelFilter {
    match level.to_uppercase().as_str() {
        "OFF" => LevelFilter::Off,
        "ERROR" => LevelFilter::Error,
        "WARN" => LevelFilter::Warn,
        "INFO" => LevelFilter::Info,
        "DEBUG" => LevelFilter::Debug,
        "TRACE" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    }
}