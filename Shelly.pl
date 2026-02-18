#!/usr/bin/perl -w

use strict;
use warnings;
use IO::Socket;
use Storable;

use Data::Dumper;

# Load RiveScript.
use RiveScript;

# Load AI::CBR
use AI::CBR::Sim qw(sim_set);
use AI::CBR::Case;
use AI::CBR::Retrieval;

open(my $learned, "+>>", "./replies/learned.rs") or die "Can't open learned.rs\n";
close($learned);

print "Initializing AI::CBR\n";
my @cases = {};
if (open(my $fh, '<', 'cases')) {
	my $stuff = '';
	while (<$fh>) {
		$stuff .= $_;
	}
	eval {
		@cases = @{Storable::thaw($stuff)};
	};
	close($fh);
}
sub addcase {
	my ($ref1, $ref2) = @_;
	my @words = @$ref1;
	my @replies = @$ref2;
	my $new_case = {
		words	=> [ @words ],
		replies	=> [ @replies ],
	};
	push @cases, $new_case;
}
sub loadfileascases {
	my $file = $_[0];
	open (my $fh, '<', "./replies/" . $file) or die "Can't open " . $file . "\n";
	while (my $line = readline($fh)) {
		chomp($line);
		if ($line =~ m/^\+ [a-z0-9\ ]+$/) {
			my $said = substr($line, 2);
			my @words = split(' ', $said);
			my @replies;

			while (my $line2 = readline($fh)) {
				chomp($line2);
				if ($line2 =~ m/^\- [a-zA-Z0-9\,\'\:\;\ ]+$/) {
					my $reply = substr($line2, 2);
					push @replies, $reply;
				} else {
					last;
				}
			}

			if (scalar(@replies) > 0) {
				addcase(\@words, \@replies);
			}
		}
	}
}
if (scalar(@cases) == 1) {
	opendir(my $dh, './replies') or die "Can't open ./replies directory\n";
	while(my $file = readdir($dh)) {
		next if ($file eq '.' || $file eq '..');

		if (-f './replies/' . $file && $file =~ m/\.rs$/ && $file ne 'learned.rs') {
			loadfileascases($file);
		}
	}
	closedir($dh);
	if (open(my $fh, '>', 'cases')) {
		print $fh Storable::freeze(\@cases);
		close($fh);
	}
}

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
	my @msg_array = split(/[\.\!\?](\s+|$)/, $msg);
	if (open(my $fh, '<', 'sessions/' . $who)) {
		my $stuff = '';
		while (<$fh>) {
			$stuff .= $_;
		}
		eval {
			$rs->{frozen}->{$who} = Storable::thaw($stuff);
		};
		close($fh);
        }
	if (defined($rs->{frozen}->{$who})) {
		$rs->thawUservars($who);
	}
	foreach (@msg_array) {
		if ($_ =~ /[a-zA-Z0-9]/) {
			my $treply = $rs->reply($who, $_);
			if (length($treply) == 0 || $treply =~ 'CBR') {
				my @old_cases = @cases;

				loadfileascases('learned.rs');

				my $said = $rs->{client}->{$who}->{__history__}->{input}->[0];
				my @words = split(/\s+/, $said);

				my $case = AI::CBR::Case->new(
					words	=> {
						sim	=> \&sim_set
					},
				);
				$case->set_values(
					words   => [ @words ],
				);

	                        my $r = AI::CBR::Retrieval->new($case, \@cases);
				$r->compute_sims();
				my $solution = $r->most_similar_candidate();

				if ($solution->{_sim} > 0) {
					my @replies = @{$solution->{replies}};

					if (scalar(@replies) > 0) {
						$treply = $replies[int(rand(scalar(@replies)))];
					}
				} else {
					$treply = $rs->reply($who, 'CBR');
				}

				@cases = @old_cases;
				$rs->{client}->{$who}->{__history__}->{reply}->[0] = $treply;
			}
			$reply .= $treply . ' ';
		}
	}
	$rs->freezeUservars($who);
	if (exists($rs->{frozen}->{$who}) && open(my $fh, '>', 'sessions/' . $who)) {
		print $fh Storable::freeze( $rs->{frozen}->{$who} );
		close($fh);
	}
	$client->send($reply);
	print 'me: ' . $reply . "\n---" . time . "\n";
	$client->close;
}
