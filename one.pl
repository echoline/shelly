#!/usr/bin/perl -w

use strict;
use warnings;
use IO::Socket;

my $socket = "/tmp/shelly";

my $client = IO::Socket::UNIX->new(Peer => $socket,
                                   Type => SOCK_STREAM) or die $@;

my $buf = $ENV{USER} . "\007" . join(" ", @ARGV);

$client->send($buf, 1024) or die $@;

$client->recv($buf, 1024) or die $@;

print $buf . "\n";

