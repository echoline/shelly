default:
	@echo "Run \`make install' to install this command-not-found handler on Ubuntu 9.10"
	@echo "Run \`make run' to run the daemon."

install:
	@echo modifying .bashrc
	@/bin/echo -e "function command_not_found_handle {" >> $$HOME/.bashrc
	@/bin/echo -e "	`pwd`/one.py \$$*" >> $$HOME/.bashrc
	@/bin/echo -e "	return 127" >> $$HOME/.bashrc
	@/bin/echo -e "}" >> $$HOME/.bashrc
	@echo making it auto-start
	@/bin/echo -e "perl `pwd`/Shelly.pl > /tmp/shelly.log &" >> /etc/rc.local || echo failed to make it auto-start
	make run

run:
	@echo running daemon
	@perl `pwd`/Shelly.pl > /tmp/shelly.log &
	@echo please open a new shell
