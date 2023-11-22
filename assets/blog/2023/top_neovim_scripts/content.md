# My top 3 Neovim snipptets

The primary advantage of Neovim lies in its extensibility.
It allows for the seamless addition of any step, process, or functionality you may require through Lua.
More often than not, you'll find a plugin that caters to your specific needs.
However, it's not uncommon to encounter a plugin designed for a single function that could be accomplished in just a few lines.

Here are my top 3 snippets that keep dependencies low and the update experience pleasant.

### 1) Fast Feedback. Build, Run and Test any Codebase.

Regardless of the project at hand, regular build/test/run steps are always a necessity. Working across multiple environments simultaneously means there's no room for creating something for a specific use case within the config. The ideal solution? Press a key bind, run a bash script, and receive the result without any disruption to the workflow.

```lua
-- run a bash script, pipe stdout to :term or buffer prompt (replace ':term' with ':!')
local function run_shell(filename)
	local file = vim.fn.findfile(filename, ".;")
	if file == "" then
		P("file not found: ".. filename)
		return
	else
		vim.cmd(":term ./"..file)
	end
end

-- general purpose build
vim.api.nvim_set_keymap("n", "<leader>b", function()
	run_shell("build.sh")
end,{ silent = true })

-- general purpose run
vim.api.nvim_set_keymap("n", "<leader>r", function()
	run_shell("run.sh")
end,{ silent = true })
```

Upon invoking the function, it carries out your run.sh/build.sh in the current project root and displays the output in either the inbuilt terminal or the command prompt.
This proves particularly beneficial during test-driven development.

A bit of bash goes a long way. But if you exceed 30 line, consider switching to a less arcane REPL.

### 2) Project specific notes / to-do lists

Easily manage project-specific notes. Simply open a buffer, which is identified by the project name beyond the project's scope, and save it. This is an excellent method for managing to-do lists, notes, or any information that you don't want to permanently store in the repository of your current project.

```lua
-- open a buffer and save it cross session
local buffer = vim.api.nvim_create_buf(false, true)
local function open_notes ()
    local dir = "~/.notes"
    local path = dir .. "/" .. string.gsub(vim.fn.getcwd(), "/", "") .. ".txt"
    if not vim.fn.filereadable(path) == true then
        print("cannot open " .. path)
        return
    end

    vim.api.nvim_open_win(buffer, true, {
        height = vim.api.nvim_get_option("lines") - 10,
        width = vim.api.nvim_get_option("columns") - 10,
        border = "double",
        relative = "editor",
        col = 5,
        row = 5,
    })

    vim.api.nvim_buf_call(buffer, function ()
        vim.cmd("e ".. path)
    end)

    vim.api.nvim_buf_set_keymap(buffer, "n", "q", ":wq<CR>", { silent = true })
end)

-- bind it to a key
vim.api.nvim_set_keymap("n", "<leader>n", open_notes, { silent = true })

```

Here's what's going on: The function transforms your existing project path into a string, eliminating any slashes. It then opens or creates a text file with the same name in your designated note directory (in my case ’~/.notes’) in a floating buffer. That's it!

In vim, you can also set up key binds that are specific to a single buffer. In this case, when pressing ’q’ in normal mode, the note is saved and subsequently closed.

### 3) Run the current copy register

Here's the process: Whatever is in your copy register will be executed as a bash command, and the result will be displayed at the cursor. This is particularly useful for executing curl requests and basic Linux commands.

```lua
local function run_current_reg()
	local cmd = vim.fn.getreg('"')
	local output = vim.fn.system(cmd)

	output = output:gsub("[\t]+", "") -- Remove tabs
	output = output:gsub("[\x1b]+%[.-m", "") -- Remove ANSI escape sequences

	local current_line, current_col = unpack(vim.api.nvim_win_get_cursor(0))
	local lines = {}

	for line in output:gmatch("[^\r\n]+") do
		table.insert(lines, line)
	end

	vim.api.nvim_buf_set_lines(0, current_line - 1, current_line - 1, false, lines)
end

vim.api.nvim_set_keymap("n", "<leader>p", run_current_reg, { silent = true })
```

For instance, you can query your API service. Then, pipe the result into 'jq' (a JSON parser) and configure it to only return the 'name' field.

```bash
curl -s -X GET \
-H "Content-Type: application/json" \
your_service.com/api/user |jq '.[] | select(.type=="name")'
```

The next step is simply to yank (copy) it, and then press your bind to execute the command. The result will then be displayed right at your cursor in your current buffer.

---

I hope this article has sparked your interest to finally code that one workflow you've always envisioned for yourself. Ultimately, it could be easier than you think. If you have any innovative thoughts or recommendations, feel free to drop me a [message](/contact).
