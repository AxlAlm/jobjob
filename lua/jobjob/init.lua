local popup = require("plenary.popup")

local M = {}

JOBJOB_WIN_ID = nil

local function ShowMenu(opts, cb)
	print(JOBJOB_WIN_ID)
	local height = 20
	local width = 100
	local borderchars = { "─", "│", "─", "│", "╭", "╮", "╯", "╰" }

	JOBJOB_WIN_ID = popup.create(opts, {
		title = "JobJob Jobs",
		highlight = "MyProjectWindow",
		line = math.floor(((vim.o.lines - height) / 1.2) - 1),
		col = math.floor((vim.o.columns - width) / 2),
		minwidth = width,
		minheight = height,
		borderchars = borderchars,
		callback = cb,
	})
	local bufnr = vim.api.nvim_win_get_buf(JOBJOB_WIN_ID)
	-- vim.api.nvim_buf_set_keymap(bufnr, "n", "q", "<cmd>lua CloseMenu()<CR>", { silent = false })
	vim.api.nvim_buf_set_keymap(
		bufnr,
		"n",
		"q",
		"<Cmd>lua require('jobjob').toggle_quick_menu()<CR>",
		{ silent = true }
	)
	vim.api.nvim_buf_set_keymap(
		bufnr,
		"n",
		"<ESC>",
		"<Cmd>lua require('jobjob').toggle_quick_menu()<CR>",
		{ silent = true }
	)
end

function M.toggle_quick_menu()
	if JOBJOB_WIN_ID ~= nil and vim.api.nvim_win_is_valid(JOBJOB_WIN_ID) then
		M.close_menu()
		return
	end

	local opts = {}
	local cb = function(_, sel)
		print("it works")
	end
	ShowMenu(opts, cb)
end

function M.close_menu()
	vim.api.nvim_win_close(JOBJOB_WIN_ID, true)
end

return M
