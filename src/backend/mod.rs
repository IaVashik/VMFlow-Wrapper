pub enum Error {}

pub enum ProcessingMessage {
    /// Обычное сообщение для лога
    LogInfo(String),
    /// Сообщение об успехе (можно выделить зеленым)
    LogSuccess(String),
    /// Предупреждение (можно выделить желтым/оранжевым)
    LogWarning {
        message: String,
        details: Option<Error>, // Структурированная информация об ошибке/предупреждении
    },
    /// Ошибка (можно выделить красным)
    LogError {
        message: String,
        details: Option<Error>, // Структурированная информация об ошибке
    },

    SetNewProcessName(String),
    ProcessFinished,
    CompilationFinished,
    CompilationFailed,
}