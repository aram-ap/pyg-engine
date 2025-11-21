#pragma once

#include <spdlog/spdlog.h>
#include <spdlog/sinks/stdout_color_sinks.h>
#include <spdlog/sinks/basic_file_sink.h>
#include <memory>
#include <string>

namespace pyg {

/**
 * @brief Logger wrapper class for spdlog
 *
 * Provides a convenient interface for logging throughout the engine.
 * Supports multiple log levels and outputs to both console and file.
 */
class Logger {
public:
    enum Type {
        Info = 0,
        Warning = 1,
        Error = 2,
        Debug = 3,
        Trace = 4,
        Critical = 5
    };

    /**
     * @brief Initialize the logging system
     * @param name Logger name (default: "pyg_engine")
     * @param logFile Optional log file path
     */
    static void init(const std::string& name = "pyg_engine", const std::string& logFile = "");

    /**
     * @brief Get the core logger instance
     * @return Shared pointer to the logger
     */
    static std::shared_ptr<spdlog::logger>& getCoreLogger() { return s_CoreLogger; }

    /**
     * @brief Shutdown the logging system
     */
    static void shutdown();

    // Convenience logging methods
    template<typename... Args>
    static void trace(Args&&... args) {
        s_CoreLogger->trace(std::forward<Args>(args)...);
    }

    template<typename... Args>
    static void debug(Args&&... args) {
        s_CoreLogger->debug(std::forward<Args>(args)...);
    }

    template<typename... Args>
    static void info(Args&&... args) {
        s_CoreLogger->info(std::forward<Args>(args)...);
    }

    template<typename... Args>
    static void warn(Args&&... args) {
        s_CoreLogger->warn(std::forward<Args>(args)...);
    }

    template<typename... Args>
    static void error(Args&&... args) {
        s_CoreLogger->error(std::forward<Args>(args)...);
    }

    template<typename... Args>
    static void critical(Args&&... args) {
        s_CoreLogger->critical(std::forward<Args>(args)...);
    }

    template<typename... Args>
    static void print(const Type type, Args&&... args) {
        switch (type) {
            case Trace:
                trace(std::forward<Args>(args)...);
                break;
            case Debug:
                debug(std::forward<Args>(args)...);
                break;
            case Info:
                info(std::forward<Args>(args)...);
                break;
            case Warning:
                warn(std::forward<Args>(args)...);
                break;
            case Error:
                error(std::forward<Args>(args)...);
                break;
            case Critical:
                critical(std::forward<Args>(args)...);
                break;
        }
    }

    /**
     * @brief Set the log level
     * @param level spdlog log level
     */
    static void setLevel(spdlog::level::level_enum level);

private:
    static std::shared_ptr<spdlog::logger> s_CoreLogger;

};

} // namespace pyg

// Convenience macros for logging
#define PYG_TRACE(...)    ::pyg::Logger::trace(__VA_ARGS__)
#define PYG_DEBUG(...)    ::pyg::Logger::debug(__VA_ARGS__)
#define PYG_INFO(...)     ::pyg::Logger::info(__VA_ARGS__)
#define PYG_WARN(...)     ::pyg::Logger::warn(__VA_ARGS__)
#define PYG_ERROR(...)    ::pyg::Logger::error(__VA_ARGS__)
#define PYG_CRITICAL(...) ::pyg::Logger::critical(__VA_ARGS__)
#define PYG_LOG(type, ...) ::pyg::Logger::print(type, __VA_ARGS__)

