// aiml2rs -- Generated on Mon Jan 18 21:14:58 2010

+ good answer{weight=100}
* <reply2> ne undefined => <set badanswer-input=<input>><set badanswer-that=<reply2>>"<set badanswer-newresp={sentence}<reply>{/sentence}><get badanswer-newresp>..."?  Does this depend on me having just said, "<get badanswer-that>"?{topic=badanswer}
- I haven't said anything yet.

+ bad answer{weight=100}
* <reply2> ne undefined => <set badanswer-input=<input>><set badanswer-that=<reply2>>Ok, what should I have said?
- I haven't said anything yet.

+ *{weight=100}
% ok what should i have said
* <get badanswer-that> ne undefined => "<set badanswer-newresp={sentence}<star>{/sentence}><get badanswer-newresp>..."?  Does this depend on me having just said, "<get badanswer-that>"?{topic=badanswer}
- "<set badanswer-newresp={sentence}<star>{/sentence}><get badanswer-newresp>..."?  Do you want me to remember that?{topic=badanswer2}

+ [*] @badword [*]{weight=100}
% ok what should i have said
- Besides that.
- I won't say that.
- Teach me something else.

> topic badanswer
	+ *
	- Yes, no or cancel?

	+ @yes
	- <call>learn <bot xrs> <get badanswer-newresp>:<get badanswer-input>:<get badanswer-that></call>{topic=random}

	+ @no
	- <call>learn <bot xrs> <get badanswer-newresp>:<get badanswer-input></call>{topic=random}

	+ cancel
	- Cancelled learning sequence.{topic=random}

	+ _ _
	@ <star>
< topic

> topic badanswer2
	+ *
	- Yes or no?

	+ @yes
	- <call>learn <bot xrs> <get badanswer-newresp>:<get badanswer-input></call>{topic=random}

	+ @no
	- Ok, let's forget it then.{topic=random}

	+ _ _
	@ <star>
< topic

> object learn perl
	my ($rs, $xrs, @args) = @_;
	@args = split(':', join(' ', @args));
	if (scalar(@args) == 2) {
		open(APPEND, '>>' . $xrs);
		print APPEND '+ ' . $rs->_formatMessage($args[1]) . "\n";
		print APPEND '- ' . $args[0] . "\n\n";
		close(APPEND);
		$rs->loadFile($xrs);
		$rs->sortReplies;
		return "Okay, I'll try to remember to respond, \"" . $args[0] . "\" when you say, \"" . $args[1] . "\"";
	} elsif (scalar(@args) == 3) {
		open(APPEND, '>>' . $xrs);
		print APPEND '+ ' . $rs->_formatMessage($args[1]) . "\n";
		print APPEND '% ' . $rs->_formatMessage($args[2]) . "\n";
		print APPEND '- ' . $args[0] . "\n\n";
		close(APPEND);
		$rs->loadFile($xrs);
		$rs->sortReplies;
		return "Okay, I'll try to remember to respond, \"" . $args[0] . "\" when you say, \"" . $args[1] . "\" if I have just said, \"" . $args[2] . "\"";
	} else {
		return scalar(@args) . " is not a valid arity to this object";
	}
< object

+ wrong
- {@bad answer}

+ not right
- {@bad answer}

+ that is wrong
- {@bad answer}

+ that is not right
- {@bad answer}

+ that is incorrect
- {@bad answer}

+ that answer is not correct
- {@bad answer}

+ that answer is incorrect
- {@bad answer}

+ that answer is wrong
- {@bad answer}

+ that answer is not right
- {@bad answer}

+ that answer was bad
- {@bad answer}

+ that was a bad answer
- {@bad answer}

+ that was an incorrect answer
- {@bad answer}

+ that was the wrong answer
- {@bad answer}

+ that answer was not right
- {@bad answer}

+ wrong answer
- {@bad answer}

+ your answer was wrong
- {@bad answer}

+ your answer was not right
- {@bad answer}

+ your answer was not correct
- {@bad answer}

+ can i teach you
- Yes, if I give you a bad answer, just say "Bad answer" and you can teach me a new response.

+ can you learn
- {@can i teach you}

+ do you learn
- {@can i teach you}

+ can i teach you *
- {@can i teach you}

+ can you learn *
- {@can i teach you}

+ will you learn *
- {@can i teach you}

+ if * will you learn *
- {@can i teach you}

+ do you learn *
- {@can i teach you}
