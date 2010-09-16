#!/usr/bin/perl

use strict;
use warnings;
use IO::Socket;

my $socket = "/tmp/shelly";
my $server = IO::Socket::UNIX->new($socket) or die $@;

my $msg = join(" ", @ARGV);
my $stuff = $< . "\007" . $msg;

$server->send($stuff);
$server->read($msg, 1024);

print $msg . "\n";
