! array typos = sl ks l

+ *
* <get name> eq undefined => Welcome to the shell, user <id>.  What is your name?
* <get personality> eq abusive => Why don't you just stop trying until you feel nicer.
- This input was neither a valid command nor something that I have a good reply to yet.
- Now would be a good time to train me.

+ [a] (command|commands)
- Commands are things that you, as the user, can type here as input.  If your input begins with a valid command, I will simply run it.  If not, I will attempt to reply in English to what I will assume is also English.  Some commands are English words, such as the program `yes,' which, by itself on a line of input, will proceed when run to print out the letter y until you press ctrl+c.  Do you understand?

+ this
- This is the shell.  {@shell}

+ [the] shell
- In the shell you can type commands, and they do things.

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
- When you hold down ctrl and press c, this will kill most programs.

+ (ctrl d|ctrld)
- Holding down ctrl and pressing d sends the end-of-file character, which will often "complete the input" to a program, at which point it will exit.  You can try pressing it now, but know I will miss you.

+ (ctrl z|ctrlz)
- Holding down ctrl and pressing z sends a signal to the current program to freeze, after which you can put it in the background with the command `bg' and continue using the shell while it runs, or type `fg' to resume where it left off.

+ (kill|end|stop) a (program|process)
@ ctrl c

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
- The command `yes' prints out the letter y over and over again.  You can pipe it into other commands to answer yes for you, but when it's run by itself you better know how to kill a process.  Do you understand?

+ what is a pointer
- Pointers are something you don't really have to deal with until you program in C.  

+ what
- Perhaps you typed a valid command.
- I am not sure what's going on either, believe me.

+ can you [*] talk [*]{weight=2}
- Try typing:\n{weight=3}
^ espeak "Hello world" --stdout -p 99 -s 140 -a 30 | aplay
- I can talk with espeak or by piping espeak into aplay.
- I can talk, using the commands espeak and aplay...

+ [*] talk [*]{weight=2}
- Are you asking if I can talk?

+ [*] @yes [*]
% are you asking if i can talk
@ can you talk

+ what is that
% this is the shell
@ what is the shell

+ like what
% in the shell you can type commands and they do things
- Anything you can do with a computer

+ i do not want to print [*]
- `Print' often refers to the outputting to the console.  The program lp is for actual printing.

+ how do i learn more about *
- Try typing man <star> or apropos <star>.

+ what is (man|apropos)
- The program man is for reading manual pages.  Apropos is for searching them.  Try `man man'.

+ how do i use this
- You type commands to do things.

+ what commands
- There are many possible commands.

+ what are they
% there are many possible commands
- Try pressing tab a few times.

