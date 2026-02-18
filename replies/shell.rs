! array typos = sl ks l

+ *
* <get personality> eq abusive => Why don't you just stop trying until you feel nicer.
- CBR

+ cbr
- I have no idea what you're talking about.
- I have no idea.
- I honestly have no idea what you're talking about.

+ [a] (command|commands)
- Commands are things that you, as the user, can type here as input.  If your input begins with a valid command, I will simply run it.  If not, I will attempt to reply in English to what I will assume is also English.  Some commands are English words, such as the program "yes," which, by itself on a line of input, will proceed when run to print out the letter y until you press ctrl+c.  Do you understand?

+ this
- This is the shell.  {@shell}

+ shell
- In the shell you can type commands, and they do things.

+ [ok] what do i do
@ shell

+ [ok] how (do|can) i (use|do) this
@ shell

+ [ok] how (do|can) i (use|do) the shell
@ shell

+ [ok] tell me what to do
@ shell

+ [ok] how do i
@ shell

+ [*] [the] shell
@ shell

+ what is this [*]
- This is the shell.

+ how (do|can) i learn [*] command*
- You can often use the programs `man' and `info'.   Try typing `man man' to learn about the command for viewing manual pages, `man' itself.

+ how (do|can) i (see|view|get|list|some) [*] commands
- Try pressing the tab key at an empty prompt, a feature called tab completion will list all possible commands.

+ tab completion
- Tab completion is a wonderful feature of the shell.  When you don't want to type a command, filename, or sometimes even options to a command, you can often type the first part and press the tab key.

+ @typos
- Whoops!
- Yeah... I don't think you typed that right...

+ ctrl
- This refers to the control key on your keyboard.  It should be down in the corner.

+ (ctrl c|ctrlc)
- When you hold down ctrl and press c, this will interrupt most programs running in the foreground.

+ (ctrl d|ctrld)
- Holding down ctrl and pressing d sends the end-of-file character, which will often "complete the input" to a program, at which point it will exit.  You can try pressing it now, but know I will miss you.

+ (ctrl z|ctrlz)
- Holding down ctrl and pressing z sends a signal to the current program to freeze, after which you can put it in the background with the command `bg' and continue using the shell while it runs, or type `fg' to resume where it left off.

+ [how] (can|do) [i] (kill|end|stop|interrupt) a (program|process)
@ ctrl c

+ [how] (can|do) [i] (kill|end|stop|interrupt) a background (program|process)
@ kill

+ what is (kill|killall)
@ kill

+ kill
- The "kill" program stops a running process by its PID, a numeric process ID.  The "killall" or "pkill" programs end processes by their program name.

+ killing [*]
% [*] which part requires clarification [*]
@ ctrl c

+ why is it called [the] kill [*]
- Beats me, but my best guess is that a human thought it up.  It basically tries to remove the program from ram, but it will still reside somewhere until all of the copies of its data are deleted.

+ why is it called yes [*]
- Well, no one imagined I would come along and confuse the matter this way.

+ why (not|should not i) [*]{weight=2}
% [*] try to remember not to start a sentence with yes [*]
- Why shouldn't you start a sentence with yes?

+ why{weight=2}
% [*] try to remember not to start a sentence with yes [*]
- Why shouldn't you start a sentence with yes? 

+ @yes
% why should not you start a sentence with yes
- The command "yes" prints out the letter y over and over again.  You can pipe it into other commands to answer yes for you, but when it's run by itself you better know how to kill a process.  Do you understand?

+ what is a pointer
- Pointers are something you don't really have to deal with until you program in C.  

+ what
- Perhaps you typed a valid command.
- I am not sure what's going on either, believe me.

+ what is that
% this is the shell
@ what is the shell

+ like what
% in the shell you can type commands and they do things
- Anything you can do with a computer.

+ i do not want to print [*]
- `Print' often refers to the outputting to the console.  The program lp is for actual printing.

+ how do i learn more about *
- Try typing man <star> or apropos <star>.

+ what is (man|apropos)
- The program man is for reading manual pages.  Apropos is for searching through them.  Try "man man".

+ how do i use this
- You type commands to do things.

+ what commands
- There are many possible commands.

+ what are they
% there are many possible commands
- Try pressing tab a few times.

+ how (can|do) i see [system] (info|information)
- There are many things to know about, "free" shows information about memory, "uptime" shows information about uptime and load, "uname" shows OS information, "ps" shows information about running programs, "man" shows manual pages.

+ unix
- That's a good question... It's an operating system. The basics of the design philosophy are to have many simple interoperable programs and files.

+ linux
@ unix

+ what is (linux|unix)
@ unix

+ file
- Files aren't necessarily what you thought. They are simply system objects that can be opened, read from, written to, closed, etcetera.

+ what (is a|are) (file|files)
@ file

+ what are (some|the|all of the|all the) commands
- There are many commands.  You can press the tab key to auto-complete commands, "ls" is a command to list out directories, "/bin" is a directory full of commands, so "ls /bin" or "ls /usr/bin" are good places to see lists of commands.  Many commands have manual pages accessed with the "man" command, as in "man ls"

+ where are the instructions
- Try typing "man bash" to read the manual page for the bash shell.

+ how (do|can) i use the shell
@ where are the instructions

+ how (do|can) i (do|use) this
@ where are the instructions

+ how (does|can) one (do|use) (this|it)
@ where are the instructions

+ how (does|can) one (do|use) the shell
@ where are the instructions

