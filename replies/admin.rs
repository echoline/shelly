// Administrative functions.

+ reload{weight=10000}
- <call>reload</call>

> object reload perl
	my ($rs, @args) = @_;
	$rs->loadDirectory("./replies");
	$rs->sortReplies;
	return "Brain reloading... success! (maybe)";
< object

+ trace{weight=100}
- \- <reply9>
^ \+ <input9>
^ \- <reply8>
^ \+ <input8>
^ \- <reply7>
^ \+ <input7>
^ \- <reply6>
^ \+ <input6>
^ \- <reply5>
^ \+ <input5>
^ \- <reply4>
^ \+ <input4>
^ \- <reply3>
^ \+ <input3>
^ \- <reply2>
^ \+ <input2>
^ \- <reply1>
^ \+ <input1>

