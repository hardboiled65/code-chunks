#!/usr/bin/perl

use LWP::Simple;
use XML::Simple;
use Data::Dumper;
use XML::LibXML;

$lat = 37.50;
$lon = 127.04;
$zoom = 18;
$url = "http://nominatim.openstreetmap.org/reverse?format=xml&lat=$lat&lon=$lon&zoom=$zoom&addressdetails=1";
$content = get($url);
#$content =~ s/ /%20/g;
#print $content . "\n";

$xml = new XML::Simple;
$data = $xml->XMLin($content);
#print Dumper($data);
print "Position: ". $data->{result}->{content}. "\n";

$parser = XML::LibXML->new();
$tree = $parser->parse_file($content);
$root = $tree->getDocumentElement;
@species = $root->getElementsByTagName('result');

foreach $camelid(@species) {
}
