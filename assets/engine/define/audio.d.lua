---@class Compressor
---@field attack_duration number|nil
---@field  makeup_gain number|nil
---@field  mix number|nil
---@field  ratio number|nil
---@field  release_duration number|nil
---@field  threshold number|nil


---@class  Delay
---@field  delay_time number|nil
---@field  feedback number|nil
---@field  mix number|nil

---@class  Distortion
---@field  drive number|nil
---@field  kind "HardClip" | "SoftClip" | nil
---@field  mix number|nil


---@class  EqFilter
---@field  kind "Bell" | "LowShelf" | "HighShelf" | nil
---@field  frequency number|nil
---@field  gain number|nil
---@field  q number|nil


---@class Filter
---@field cutoff number|nil
---@field mix number|nil
---@field mode "LowPass" | "BandPass" | "HighPass" | "Notch" | nil
---@field resonance number|nil

---@class PanningControl
---@field panning number

---@class  Reverb
---@field  damping number|nil
---@field  feedback number|nil
---@field  mix number|nil
---@field  stereo_width number|nil
---@class  VolumeControl 
---@field  volume number

---@class EffectConfig
---@field Compressor Compressor| nil
---@field Delay Delay | nil
---@field EqFilter EqFilter | nil
---@field Filter Filter | nil
---@field PanningControl PanningControl | nil
---@field Reverb Reverb | nil



---@class Audio
local Audio = {}
---@param name string
---@param volume number
---@param persist boolean
---@param effects EffectConfig|nil
function Audio:add_group(name, volume, persist, effects)
end

---@param group string
---@param audio string
---@param volume number | nil
---@param panning number | nil
---@param position number | nil
function Audio:play(group, audio, volume, panning, position)
end

---@param group string
---@param audio string
---@param duration number
function Audio:pause(group, audio, duration)
end
---@param group string
---@param audio string
---@param duration number
function Audio:resume(group, audio, duration)
end
---@param group string
---@param audio string
---@param duration number
function Audio:stop(group, audio, duration)
end

---@param group string
---@param audio string
---@param amount number
function Audio:seek_by(group, audio, amount)
end

---@param group string
---@param audio string
---@param position number
function Audio:seek_to(group, audio, position)
end


---@param group string
---@param audio string
---@param volume number
---@param duration number
function Audio:set_volume(group, audio, volume, duration)
end


---@param group string
---@param audio string
---@param panning number
---@param duration number
function Audio:set_panning(group, audio, panning, duration)
end

---@param group string
---@param audio string
---"Playing"|"Pausing" | "Paused" | "WaitingToResume" | "Resuming" | "Stopping"| "Stopped"
---@return string | nil
function Audio:state(group, audio)
    return ""
end

---@param group string
---@param effect string
---@param config EffectConfig
---@param tween number|nil
function Audio:set_effect(group, effect, config, tween)
end

---@param duration number
function Audio:pause_all(duration)
end

---@param duration number
function Audio:resume_all(duration)
end


---@param volume number
---@param duration number
function Audio:set_volume_all(volume, duration)
end

---@param duration number
function Audio:stop_all(duration)
end