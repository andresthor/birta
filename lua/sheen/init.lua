local M = {}

-- Per-buffer job handles: buf_number -> job_id
local jobs = {}

local SHEEN_CMD = "sheen"

--- Start preview for the current buffer.
function M.preview()
  local buf = vim.api.nvim_get_current_buf()

  if jobs[buf] then
    vim.notify("sheen: already running for this buffer", vim.log.levels.WARN)
    return
  end

  local file = vim.api.nvim_buf_get_name(buf)
  if file == "" then
    vim.notify("sheen: buffer has no file", vim.log.levels.ERROR)
    return
  end

  local cmd = { SHEEN_CMD, file }
  local job_id = vim.fn.jobstart(cmd, {
    detach = true,
    on_exit = function(_, code)
      jobs[buf] = nil
      if code ~= 0 and code ~= 143 then -- 143 = SIGTERM, normal shutdown
        vim.schedule(function()
          vim.notify("sheen: exited with code " .. code, vim.log.levels.WARN)
        end)
      end
    end,
  })

  if job_id <= 0 then
    vim.notify("sheen: failed to start (is sheen in PATH?)", vim.log.levels.ERROR)
    return
  end

  jobs[buf] = job_id

  -- Clean up if the buffer is deleted
  vim.api.nvim_create_autocmd("BufDelete", {
    buffer = buf,
    once = true,
    callback = function()
      M.stop(buf)
    end,
  })
end

--- Stop preview for a buffer (defaults to current).
---@param buf? number
function M.stop(buf)
  buf = buf or vim.api.nvim_get_current_buf()
  local job_id = jobs[buf]
  if not job_id then
    vim.notify("sheen: not running for this buffer", vim.log.levels.WARN)
    return
  end
  vim.fn.jobstop(job_id)
  jobs[buf] = nil
end

--- Toggle preview for the current buffer.
function M.toggle()
  local buf = vim.api.nvim_get_current_buf()
  if jobs[buf] then
    M.stop(buf)
  else
    M.preview()
  end
end

--- Check if preview is running for a buffer (defaults to current).
---@param buf? number
---@return boolean
function M.is_running(buf)
  buf = buf or vim.api.nvim_get_current_buf()
  return jobs[buf] ~= nil
end

return M
