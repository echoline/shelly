#!/usr/bin/perl -w

my @categories;

while(<STDIN>) {
	if ($_ =~ m/^\+ /) {
		my $found = 0;
		my $category = {
			trigger	=> $_,
			replies	=> [],
		};
		foreach my $i (@categories) {
			if ($i->{trigger} eq $_) {
				$category = $i;
				$found = 1;
				last;
			}
		}
		if ($found == 0) {
			push @categories, $category;
		}
		while (<STDIN>) {
			if ($_ =~ m/^\- /) {
				push @{$category->{replies}}, $_;
			} elsif ($_ =~ m/^$/) {
				last;
			}
		}
	}
}

foreach my $i (@categories) {
	print $i->{trigger};

	foreach my $j (@{$i->{replies}}) {
		print $j;
	}

	print "\n";
}
