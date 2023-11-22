# How to add a full team of private specialists to Neovim. Local AI on steroids.

Imagine being able to have a full team of specialists at your fingertips wherever you go, with 100% private, uncensored, and predefined models tailored to your specific needs.
No need to dream anymore - modern FOSS models make this a reality! Let's implement them into your Neovim workflow. It's easier than you think.

<div style="height: 250px; overflow:hidden;">
    <video height="600" width="800" autoplay loop>
      <source src="/media/neovim-ai/grug_ai.webm" type="video/webm">
    </video>
</div>

### Disclaimer

Before we start let me give a piece of advice. This is personal opinion, so take it with a grain of salt.
LLVMs are tools. Tools are only as good as the one using it. Ai will not replace programmers any time soon.
They are incredibly useful, if used correctly. But they can also do great harm. Do not let AI write any code other than boilerplate.
They create bugs like any other programmer. They are trained on the average and that's exactly what you get.

This is a tutorial and not Plugin. If you have never tried to expand your Neovim workspace before, you are missing out on the best part.
I highly encourage you to go out and play. Lua and the Neovim api is very easy to learn, and the benefit is huge.

### Where will our Models come from?

We will employ the open-source software ollama, which enables us to access cutting-edge models from the community, that can be operated locally on your machine.
This tool is incredibly user-friendly and easy to install on most systems.
With a straightforward command-line interface, you can quickly retrieve your desired output.

You'll find the installation instructions on the [official website](https://ollama.ai). It's a Unix tool, so no Windows.
But in all honesty, I've never encountered a Windows user who utilizes Neovim. If you happen to be one ... shame on you.

The initial execution of a model will automatically download the necessary files. It is advisable to perform this action prior to running from code. Here is an example:

```bash
ollama run mistral:instruct 'Tell me joke about JS'
```

Thanks to [David](https://www.youtube.com/watch?v=FIZt7MinpMY), a fellow neovim enjoyer, for coming up with the original idea.

### Let's write some Lua

At the very core, we require a function to interact with Ollama, return the result to a specific buffer, and display it in an asynchronous manner.

```lua
local M = {}

--- @param model string
--- @param context string
--- @param prompt string
--- @param buf_nr number
--- @return number returns the job id
M.run = function(model, context, prompt, buf_nr)
    -- prepare the command string
	local cmd = ("ollama run $model $prompt"):gsub("$model", model):gsub("$prompt", vim.fn.shellescape(context .. "\n" .. prompt))

    -- print the prompt header
	local header = vim.split(prompt,"\n")
	table.insert(header, "----------------------------------------")
	vim.api.nvim_buf_set_lines(buf_nr, 0, -1, false, header)

	local line = vim.tbl_count(header) + 1
	local words = {}

    -- start the async job
	return vim.fn.jobstart(cmd, {
		on_stdout = function(_, data, _)
			for i, token in ipairs(data) do
				if i > 1 then -- if returned data array has more than one element, a line break occured.
					line = line + 1
					words = {}
				end
				table.insert(words, token)
				vim.api.nvim_buf_set_lines(buf_nr, line, line + 1, false, {table.concat(words, "")})
			end
		end,
	})
end

return M

```

Now, we can use it to access any model and begin asking questions. Let's add a key bind to get some User input and pass it to Ollama.

```lua
-- The Plenary library provides an easy way to spawn new floating windows,
-- but it's not entirely necessary and can be accomplished using other methods as well.
-- If you use telescope, you already have it

local plenary = require("plenary.window.float")
local ollama = require("path-to-the-script-above")

-- define some styles and spawn a scratch buffer
local win_options = {
	winblend = 10,
	border = "rounded",
	bufnr = vim.api.nvim_create_buf(false, true)
}

-- run a simple prompt with the mistral model
vim.keymap.set("n", "<leader>cc", function()
	plenary.clear(win_options.bufnr)
	local float = plen.percentage_range_window(0.8, 0.8, win_options)
	ollama.run("mistral:instruct", "", vim.fn.input("Prompt: "), float.bufnr)
end, { silent = true })
```

This is where the magic happens! Here, you have the freedom to incorporate as many custom key binds as needed for various models.
You might have observed that the context string is currently devoid of any content.
If you want to assign a particular task to your model, you can include a context string such as 'Only answer in JSON' or 'Utilize pseudo code'.
And a gentle reminder: treating your AI with kindness fosters exceptional results. They operate optimally when requests are made politely. **This is not a joke!**

```lua
-- math questions? Try wizard-math!
ollama.run("wizard-math", "", vim.fn.input("Prompt: "), float.bufnr)

-- need philosophical guidance?
ollama.run("samantha-mistral", "", vim.fn.input("Prompt: "), float.bufnr)

-- need a sql specialist?
ollama.run("sqlcoder", "", vim.fn.input("Prompt: "), float.bufnr)
```

The list goes on. Checkout the available [models](https://ollama.ai/library).

### Using visual selection

Certainly, we can continue beyond this point. While having a basic prompt is useful, it's even more convenient to select specific lines from your document or code and prompt them directly. Let's create a function to accomplish this as well.
First we need to get the visual selection. To be honest, reading the visual selection in lua is huge pain with many edge cases you have to consider. Big 'V' and little 'v' are 2 different modes with different behavior.
This function is literally copied from the [chatgpt-plugin](https://github.com/jackMort/ChatGPT.nvim):

```lua
local ESC_FEEDKEY = vim.api.nvim_replace_termcodes("<ESC>", true, false, true)
--- @return string
M.get_visual_lines = function(bufnr)
	vim.api.nvim_feedkeys(ESC_FEEDKEY, "n", true)
	vim.api.nvim_feedkeys("gv", "x", false)
	vim.api.nvim_feedkeys(ESC_FEEDKEY, "n", true)

	local start_row, start_col = unpack(vim.api.nvim_buf_get_mark(bufnr, "<"))
	local end_row, end_col = unpack(vim.api.nvim_buf_get_mark(bufnr, ">"))
	local lines = vim.api.nvim_buf_get_lines(bufnr, start_row - 1, end_row, false)

	if start_row == 0 then
		lines = vim.api.nvim_buf_get_lines(bufnr, 0, -1, false)
		start_row = 1
		start_col = 0
		end_row = #lines
		end_col = #lines[#lines]
	end

	start_col = start_col + 1
	end_col = math.min(end_col, #lines[#lines] - 1) + 1

	lines[#lines] = lines[#lines]:sub(1, end_col)
	lines[1] = lines[1]:sub(start_col)

	return table.concat(lines, "\n")
end
```

Now that we can get the visual selection it is time to add more key binds. Here's my go-to command prompt that I use all the time.
And speaking of which, I just used this command to revise this article.

```lua
vim.keymap.set("v", "<leader>cr", function()
	local prompt = ollama.get_visual_lines(0)
	local context = "Please rewrite these sentances in proper english:"
	plen.clear(win_options.bufnr)
	local float = plen.percentage_range_window(0.8, 0.8, win_options)
	ollama.run("mistral:instruct", context, prompt, float.bufnr)
end, { silent = true })
```

Congrats! You now have a team of specialists, ready to go on your local machine.
They might try to fool you every once in a while, but at least they won't cost you a dime.
Well that's not entirely true. You pay with your CPU or GPU, if you have a modern Nvidia card.

My 5-year-old Ryzen CPU can still handle the 7B Models with ease.
Maybe one day someone will write a similar tool that uses vulkan/wgpu for us AMD users.

If you're interested, here is my full [configuration](https://github.com/Lommix/dotfiles/blob/master/nvim/lua/lommix/scripts/init.lua).

Hope you learned something new.
