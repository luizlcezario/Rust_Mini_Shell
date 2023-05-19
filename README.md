
<div>
<p align="center">
	<a href="https://www.42sp.org.br/">
		<img src="./.github/42.png" alt="42" width="500"/> 
	</a>
</p>
</div>
<p align="center">	
   <a href="https://www.linkedin.com/in/luiz-lima-cezario/">
      <img alt="Luiz Cezario" src="https://img.shields.io/badge/-LuizlCezario-682998?style=flat&logo=Linkedin&logoColor=white" />
   </a>

  <a aria-label="Completed" href="https://www.42sp.org.br/">
    <img src="https://img.shields.io/badge/42.sp-Rust_MiniShell-682998?logo="></img>
  </a>
  <a href="https://github.com/luizlcezario/Rust_Mini_Shell/commits/master">
    <img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/luizlcezario/Rust_Mini_Shell?color=682998">
  </a> 

  <a href="https://github.com/luizlcezario/Rust_Mini_Shell/stargazers">
    <img alt="Stargazers" src="https://img.shields.io/github/stars/luizlcezario/Rust_Mini_Shell?color=682998&logo=github">
  </a>
</p>

<div align="center">
  <sub>Minishell of 42. Make with ❤︎ for
        <a href="https://github.com/luizlcezario">Luiz Cezario</a> 
    </a>
  </sub>
</div>

# MiniShell

This one of the most dificults projects from the first part of formation of Ecole 42, in this we need to recreate a shell with all this features:
* exec multiples commands using '|` 
* this: "<", ">", "<<", ">>" redirection tokens in working with multiples command.
* history of commnads
* signals in according with bash
* parser of the env variables, to this we need to treat "$HOME", $HOME , '$HOME'.


## Idea

For this I make a infinite loop that every time readline lexer this line in a sequel of things in a phrase like ex: ls -ls | grep NAME < Makefile => after the lexer we create this string : c|c<f.
This phrase will indicate to the executation what will run first my executation ios made in recursive mode that makes open the redirections first and then exec the commands.

## Test

	```bash
	$> make
	$> ./minishell
	$> ls 
	$> ...

	```

> Cristina: "Thanks to Rust Minishell, you’ll be able to travel through time and come back to problems people faced when Windows didn’t exist without leaks."

Give ⭐️ if you like this project, this will help me!
