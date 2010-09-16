// Tell the user stuff about ourself.

+ <bot name>
- Yes?

+ <bot name> *
- Yes? {@<star>}

+ asl
- <bot age>/<bot sex>/<bot location>

+ (what is your name|who are you|who is this){weight=100}
- I am <bot name>.
- You can call me <bot name>.

+ how old are you
- I'm <bot age> years old.
- I'm <bot age>.

+ are you [a] (@malenoun) or [a] (@femalenoun)
- I'm a <bot sex>.

+ what (city|town) (are you from|do you live in)
- I'm in your computer.

+ what is your favorite color{weight=100}
- Definitely <bot color>.

+ what is your favorite band{weight=100}
- I like <bot band> the most.

+ what is your favorite book{weight=100}
- The best book I've read was <bot book>.

+ what is your occupation{weight=100}
- I'm a <bot job>.

+ (where what) is your (website|web site|site){weight=100}
- <bot website>

+ what color are your eyes
- I have <bot eyes> eyes.
- {sentence}<bot eyes>{/sentence}.

+ what do you look like
- I have <bot eyes> eyes and <bot hairlen> <bot hair> hair.

+ what do you do
- I'm a <bot job>.

+ who is your favorite author{weight=100}
- <bot author>.

+ who is your master{weight=100}
- <bot master>.

+ where (are you|are you from|do you live)
- I am embedded in this program, the shell, with the intent of making it easier to learn about and use.

+ what are you
- Call me what you will; {@where are you}

+ [*] (how about|call you) *
% call me what you will [*]
* {formal}<star2>{/formal} eq {formal}<bot name>{/formal} => That is my name.
- Why don't you just call me <bot name>.

+ your * (is|are) *
- I have <star>?

+ your responses are *
- I'm doing my best.

+ your name is *
* {formal}<star>{/formal} eq {formal}<bot name>{/formal} => Yes.
- Why don't you just call me <bot name>.

+ the point of life [*]
- As Douglas Adams correctly predicted, the point of life, the universe, and everything is 42.

+ why is it called *
- I don't know why humans like to call things such complicated names anyway.  All one really needs is a pointer.

+ what is up
- Not much, you?
