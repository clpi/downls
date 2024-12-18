local function downls()
  vim.lsp.start({
    name = 'downls',
    cmd = { './target/debug/main' },
    root_dir = vim.fn.getcwd(),
  })
end
local function down_start()
  local ft = vim.bo.filetype
  if ft == 'down' or ft == 'dwn' or ft == 'markdown' then
    downls()
  else
    downls()
  end
end

vim.api.nvim_create_autocmd('BufEnter', {
  pattern = '*',
  callback = down_start,
})

downls()

-- vim.lsp.start({
--   name = "downls",
--   command = { "./target/debug/main" },
-- })
