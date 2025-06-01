TEST = {
    name = 'test',
    kind = 'Init',
    state = { aa = 0 }
}

---@diagnostic disable-next-line: lowercase-global
function TEST.init(state)
    print("init", state.aa)
end

function TEST.update(state)
    state.aa = state.aa + 1
end

return TEST
