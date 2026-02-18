default:
	@echo "Run \`make replies/tldr.rs' to make tldr.rs file"
	@echo "Run \`make .bashrc' to install custom command-not-found handler"
	@echo "Run \`make auto' as root to add daemon to /etc/rc.local"
	@echo "Run \`make run' to run the daemon."

replies/tldr.rs: tldr2rive tldr
	@echo converting tldr to rivescript
	@for f in tldr/pages.en/common/* tldr/pages.en/linux/*; do \
		./tldr2rive < $$f; \
	done > tldr.rs.tmp
	@./collapse.pl < tldr.rs.tmp > replies/tldr.rs

tldr2rive: tldr2rive.c

tldr:
	@echo git clone tldr
	@git clone https://github.com/tldr-pages/tldr

.bashrc: 
	@echo modifying .bashrc
	@/bin/echo -e "function command_not_found_handle {" >> $$HOME/.bashrc
	@/bin/echo -e "	perl `pwd`/one.pl \$$*" >> $$HOME/.bashrc
	@/bin/echo -e "	return 127" >> $$HOME/.bashrc
	@/bin/echo -e "}" >> $$HOME/.bashrc
	@echo "type \`source $$HOME/.bashrc' in running shells"

auto:
	@echo shelly daemon auto-start
	@/bin/echo -e "perl `pwd`/Shelly.pl > /tmp/shelly.log &" >> /etc/rc.local || echo failed - try again with sudo or as root.

run:
	@echo running daemon
	@perl `pwd`/Shelly.pl > /tmp/shelly.log &

