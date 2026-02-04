// respond differently to various kinds of behavior
+ @hello
* <get hi> eq undefined => Hello, and welcome to the shell.  Remember not to say "yes" to me, and that ctrl+c stops most programs.<set hi=hi>
- Hi.
- Hello.
- Hello, there.

+ @hello *
@ hello

+ you are welcome
% [*] thank you for [*]
* <get personality> eq abusive => Isn't this nicer than insulting each other?  Maybe you're not so bad after all.<set personality=undefined>
- I love humans.

+ you are welcome
* <get personality> eq abusive => I didn't say thanks.
- Did I forget to thank you?

+ you too
% nice to meet you *
- So, what's up?

+ [*] @yes [*]
% [*] did i forget to thank you [*]
- I apologize, I have bad amnesia.

+ thanks
- Yep.
- It's nothing.

+ thank you
- You're welcome.{weight=2}
- Think nothing of it.

+ (i hate|fuck|screw) you
- I won't talk to you until you take that back.<set personality=abusive>{topic=apology}

+ you (suck|blow)
@ fuck you

+ suck my (cock|dick|penis)
@ fuck you

+ blow me
@ fuck you

+ you are [a] [n] @badword [*]{weight=2}
@ fuck you

+ @badword
@ fuck you

+ you @badword
@ fuck you

> topic apology
  + catch
  - I won't listen to you until you apologize for being mean to me.
  - I have nothing to say until you say you're sorry.
  - I really mean it.

  + *
  @ catch

  + [*] (i am sorry|sorry|i apologize) [*]
  - Okay. I guess I'll forgive you then.{topic=random}

  + [*] (not|neither) sorry [*]{weight=100}
  @ catch
< topic

+ please *
- <@>  And you're welcome.

+ (would|will|can) you please *
@ please <star2>

+ * please
@ please <star>

+ i love you
* <get personality> eq abusive	=> You don't always act like it.
* <get name> eq undefined => I don't even know your name!  What is your name?
* <id> eq <bot master>	  => I love you in a professional manner as well, <get name>.
- I love you too, <get name>!

+ how are you [*]
- You may type "free" or "uptime" to view my RAM usage or load average.

+ (hi|hello|hey) [there] how are you [*]
@ how are you

+ how are you able to *
- I am a chatbot program in the shell.
- I am meant to be an educational chatbot program.
- I'm not sure.

+ how are you doing the *
@ how are you able to speak

