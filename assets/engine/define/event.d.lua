--- Event
---@class Event
local Event = {}
---[Backquote, Backslash, BracketLeft, BracketRight, Comma,
--- Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9, Equal,
--- IntlBackslash, IntlRo, IntlYen,
--- KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM,
--- KeyN, KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT, KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ,
--- Minus, Period, Quote, Semicolon, Slash, AltLeft, AltRight, Backspace, CapsLock,
--- ContextMenu, ControlLeft, ControlRight, Enter, SuperLeft, SuperRight, ShiftLeft,
--- ShiftRight, Space,
--- Tab, Convert, KanaMode, Lang1, Lang2, Lang3, Lang4, Lang5, NonConvert, Delete, End,
--- Help, Home, Insert, PageDown, PageUp, ArrowDown, ArrowLeft, ArrowRight, ArrowUp,
--- NumLock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4,
--- Numpad5, Numpad6, Numpad7, Numpad8, Numpad9, NumpadAdd, NumpadBackspace, NumpadClear, NumpadClearEntry,
--- NumpadComma, NumpadDecimal, NumpadDivide, NumpadEnter, NumpadEqual, NumpadHash,
--- NumpadMemoryAdd, NumpadMemoryClear, NumpadMemoryRecall, NumpadMemoryStore,
--- NumpadMemorySubtract, NumpadMultiply, NumpadParenLeft, NumpadParenRight, NumpadStar, NumpadSubtract, Escape,
--- Fn, FnLock, PrintScreen, ScrollLock, Pause, BrowserBack, BrowserFavorites, BrowserForward, BrowserHome,
--- BrowserRefresh, BrowserSearch, BrowserStop,Eject, LaunchApp1, LaunchApp2, LaunchMail, MediaPlayPause, MediaSelect,
--- MediaStop, MediaTrackNext, MediaTrackPrevious, Power, Sleep, AudioVolumeDown, AudioVolumeMute,
--- AudioVolumeUp,WakeUp, Meta, Hyper,Turbo, Abort, Resume, Suspend, Again, Copy, Cut, Find,
--- Open, Paste, Props, Select, Undo, Hiragana, Katakana
--- F1, F2 ,F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18,
--- F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31, F32, F33, F34, F35,
--- ]
---@param key string
---@return boolean
function Event:key_pressed(key)
    return true
end

---@param key string
---@return boolean
function Event:key_released(key)
    return true
end

---@param key string
---@return boolean
function Event:key_held(key)
    return true
end

---@return table {x = number, y = number}
function Event:get_mouse_position()
    return {}
end

---@param button string "Left"、"Right"、"Middle"
---@return boolean
function Event:mouse_pressed(button)
    return true
end

---@param button string "Left"、"Right"、"Middle"
---@return boolean
function Event:mouse_released(button)
    return true
end

---@class mouse_wheel
---@field delta {line: Point | nil, pixel: Point | nil}|nil
---@field touch string | nil
---@return mouse_wheel
---touch：string，["Started"、"Moved"、"Ended"、"Cancelled"]
function Event:mouse_wheel()
    return {}
end

---@return string|nil
function Event:get_char()
    return ""
end

---@return boolean
function Event:mouse_entered()
    return true
end

---@return boolean
function Event:focused()
    return true
end

---@class Preedit
---@field content string
---@field pos nil| Point
---@class State
---@field state string "enabled" | "disabled" | "preedit" | "commit"
---@field preedit Preedit
---@field commit string
---@return State
function Event:ime_state()
    return {}
end

---@param call_back fun()
function Event:on_exit(call_back) end