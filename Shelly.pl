#!/usr/bin/perl -w

use strict;
use warnings;
use IO::Socket;

# Use our local library.
# use lib ".";

# Load RiveScript.
use RiveScript;

# Load AI::CBR
use AI::CBR;

print "Initializing AI::CBR\n";

# Create and load the RiveScript brain.
print "Initializing RiveScript interpreter\n";
our $rs = new RiveScript;
$rs->loadDirectory ("./replies");
$rs->setVariable (master => "0");
$rs->setVariable (xrs => "./replies/learned.rs");
$rs->sortReplies;

my $socket = "/tmp/shelly";
unlink $socket;
print "Initializing socket\n";
my $server = IO::Socket::UNIX->new(Local => $socket,
				   Type => SOCK_STREAM,
				   Listen => SOMAXCONN) or die $@;
chmod(0777, $socket) || die $!;

print "Initialization complete.\n";

# Start.
while (1) {
	my $client = $server->accept();
	my $buf;
	$client->recv($buf, 1024);
	my ($who, $msg) = split(/\007/, $buf);
	print $who . ': ' . $msg . "\n";
	my $reply = '';
	my @msg_array = split(/[\.\?\!]/, $msg);
	$rs->thawUservars($who);
	foreach (@msg_array) {
		if ($_ =~ /[a-zA-Z0-9]/) {
			$reply .= $rs->reply($who, $_) . '  ';
		}
	}
	if ($reply =~ /^$/) {
		$reply = ":)";
	}
	$rs->freezeUservars($who);
	$client->send($reply);
	print 'me: ' . $reply . "\n---" . time . "\n";
	$client->close;
}
