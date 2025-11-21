#include "logging/Logger.h"
#include <spdlog/sinks/stdout_color_sinks.h>
#include <spdlog/sinks/basic_file_sink.h>
#include <string>
#include <vector>

namespace pyg {

std::shared_ptr<spdlog::logger> Logger::s_CoreLogger;

void Logger::init(const std::string& name, const std::string& logFile) {
    std::vector<spdlog::sink_ptr> sinks;

    // Console sink with colors
    auto console_sink = std::make_shared<spdlog::sinks::stdout_color_sink_mt>();
    console_sink->set_pattern("[%Y-%m-%d %H:%M:%S.%e] [%n] [%^%l%$] %v");
    sinks.push_back(console_sink);

    // File sink if log file is specified
    if (!logFile.empty()) {
        auto file_sink = std::make_shared<spdlog::sinks::basic_file_sink_mt>(logFile, true);
        file_sink->set_pattern("[%Y-%m-%d %H:%M:%S.%e] [%n] [%l] %v");
        sinks.push_back(file_sink);
    }

    // Create the logger with all sinks
    s_CoreLogger = std::make_shared<spdlog::logger>(name, sinks.begin(), sinks.end());
    s_CoreLogger->set_level(spdlog::level::trace);
    s_CoreLogger->flush_on(spdlog::level::trace);

    // Register the logger with spdlog
    spdlog::register_logger(s_CoreLogger);
    spdlog::set_default_logger(s_CoreLogger);

    PYG_INFO("Logger initialized: {}", name);
}

void Logger::shutdown() {
    if (s_CoreLogger) {
        PYG_INFO("Logger shutting down");
        s_CoreLogger->flush();
        spdlog::shutdown();
    }
}

void Logger::setLevel(spdlog::level::level_enum level) {
    if (s_CoreLogger) {
        s_CoreLogger->set_level(level);
    }
}

} // namespace pyg

