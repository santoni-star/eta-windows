# Eta Windows

System-level AI Assistant for Windows — українською мовою.

## Можливості

- **Чат з AI** — розмова з будь-якою моделлю (OpenAI, Anthropic, Gemini, Codex, Grok, Ollama)
- **Файли** — читання, запис, перегляд каталогів
- **Термінал** — виконання команд cmd, PowerShell, WSL2
- **Керування вікнами** — пошук, фокус, клік, ввід тексту через Windows UI Automation
- **Веб-пошук** — пошук в інтернеті через провайдера
- **MCP** — підключення до Model Context Protocol серверів
- **Голос** — STT/TTS через Windows SAPI
- **Пам'ять** — локальна база даних SQLite для розмов
- **Гарячі клавіші** — Ctrl+Alt+Space за замовчуванням для виклику Eta
- **Системний трей** — робота у фоні

## Технології

- **Tauri** (Rust) — бекенд, системна інтеграція
- **HTML/CSS/JS** — фронтенд інтерфейсу
- **SQLite** — локальна пам'ять
- **Win32 API** — керування вікнами, буфером обміну, клавіатурою
- **SAPI** — голосовий синтез Windows

## Встановлення та запуск

```bash
# Клонування
git clone https://github.com/santoni-star/eta-windows.git
cd eta-windows

# Встановлення залежностей (потрібно Rust + Tauri CLI)
cargo install tauri-cli

# Запуск у режимі розробки
cargo tauri dev

# Збірка MSI інсталятора
cargo tauri build
```

## Гарячі клавіші

| Клавіші | Дія |
|---------|-----|
| Ctrl+Alt+Space | Показати/приховати Eta |
| Ctrl+Shift+S | Скриншот екрану |

## Налаштування providers

У `config.yaml` (створиться автоматично):

```yaml
providers:
  - name: "OpenAI"
    api_key: "sk-..."
    model: "gpt-4"
    enabled: true
  - name: "Anthropic"
    api_key: "sk-ant-..."
    model: "claude-3-sonnet"
    enabled: true
```

## Структура проєкту

```
eta-windows/
├── src/                    # Фронтенд (HTML/JS)
│   └── main.html
├── src-tauri/
│   ├── Cargo.toml          # Залежності Rust
│   ├── tauri.conf.json     # Конфігурація Tauri
│   ├── src/
│   │   ├── main.rs         # Точка входу
│   │   ├── agent/          # Агентний цикл, пам'ять
│   │   ├── providers/      # LLM провайдери
│   │   ├── tools/          # Інструменти (файли, термінал, вікна)
│   │   ├── voice/          # Голосовий ввід/вивід
│   │   ├── system/         # Гарячі клавіші, трей, Win32 API
│   │   ├── mcp/            # MCP клієнт
│   │   └── config/         # Конфігурація
│   └── migrations/         # SQL міграції SQLite
└── README.md
```

## Відмінності від Android-версії

| Android | Windows |
|---------|---------|
| AccessibilityService | Windows UI Automation |
| Xposed хуки | Глобальні хуки клавіатури/миші |
| Termux/PRoot | WSL2 / нативний Windows PTY |
| VoiceInteractionService | Гарячі клавіші + системний трей |
| LSPosed | Власний логер подій |

## Ліцензія

Подивіться LICENSE в оригінальному проєкту Eta.
