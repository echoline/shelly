#!/usr/bin/python 
import socket
import sys
import os

def main(argv=None):
	if argv is None:
		argv = sys.argv
	input = "%d\007" % os.getuid();
	i = 0;
	for arg in argv:
		if i == 0:
			i = 1;
		else:
			input += arg + " "
	#create a UNIX, STREAMing socket
	s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
	# connect to socket
	s.connect("/tmp/shelly")
	s.send(input)
	input = s.recv(4096)
	print input 
	#close socket
	s.close()

if __name__ == "__main__":
	main()
