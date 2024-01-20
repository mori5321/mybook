## 0. Background
I wrote video processing program with C++/clang++/OpenCV. 
I wanted to make the complement for OpenCV work well on my neovim. 
This is a memorandum for myself. 

## 1. Check the lspconfig
I use neovim built-in lsp and Mason.
For language-server, I use clangd (installed with `MasonInstall clangd`).

``` init.lua
// ...
require("mason").setup()
require("mason-lspconfig").setup()
require("mason-lspconfig").setup_handlers({

	function(server_name) -- default handler (optional)
		lspconfig = require("lspconfig")

		if server_name == "tsserver" then
            // ...
		elseif server_name == "clangd" then
			lspconfig[server_name].setup({
				on_attach = on_attach,
				filetypes = { "c", "cpp" },
			})
		else
			lspconfig[server_name].setup({
				on_attach = on_attach,
			})
		end
	end,
})
// ...
```


## 2. Write CmakeLists.txt
I write CMaikeLists.txt for my building.

```
cmake_minimum_required(VERSION 3.1)
project(HelloOpenCV)

# Activate C+11 (required for OpenCV)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED TRUE)

# find OpenCV package
find_package(OpenCV REQUIRED)

# include OpenCV
include_directories(${OpenCV_INCLUDE_DIRS})

# executable
add_executable(main_app main.cpp)

# link OpenCV Libs
target_link_libraries(main_app ${OpenCV_LIBS})
```

## 3. Exec CMake
Let's exec make. Keep in mind that you need to add `-DCMAKE_EXPORT_COMPILE_COMMANDS=ON`. 
This option let the cmake generates `compile_commands.json`. 
clangd language-server read this file for complement.

```
mkdir build
cd build

cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON ..
```

## 4. link complie_commands.json to the project root.

```
cd /move/to/project/root
ln -s build/compile_commands.json .
```

## 5. (build)

```
cd build
cmake --build .
```
