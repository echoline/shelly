default:
	@echo "Run \`make .bashrc' to install custom command-not-found handler"
	@echo "Run \`make auto' as root to add daemon to /etc/rc.local"
	@echo "Run \`make run' to run the daemon."

.bashrc: 
	@echo modifying .bashrc
	@/bin/echo -e "function command_not_found_handle {" >> $$HOME/.bashrc
	@/bin/echo -e "	perl `pwd`/one.pl \$$*" >> $$HOME/.bashrc
	@/bin/echo -e "	return 127" >> $$HOME/.bashrc
	@/bin/echo -e "}" >> $$HOME/.bashrc
	@echo "type \`source $$HOME/.bashrc' in running shells"

auto:
	@echo making shelly daemon auto-start
	@/bin/echo -e "perl `pwd`/Shelly.pl > /tmp/shelly.log &" >> /etc/rc.local || echo failed - try again with sudo or as root.

run:
	@echo running daemon
	@perl `pwd`/Shelly.pl > /tmp/shelly.log &

