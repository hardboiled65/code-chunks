#!/usr/bin/env perl

use constant PI => 3.14159265358979323846;

sub toRad {
	return @_[0] * PI / 180;
}

sub toDeg {
	return @_[0] * 180 / PI;
}

sub destVincenty {
	($lat1, $lon1, $brng, $dist) = @_;
	$a = 6378137, $b = 6356752.3142, $f = 1/298.257223563; # WGS-84 ellipsiod
	$s = $dist;
	$alpha1 = toRad($brng);
	$sinAlpha1 = sin($alpha1);
	$cosAlpha1 = cos($alpha1);

	$tanU1 = (1-$f) * tan(toRad($lat1));
	$cosU1 = 1 / sqrt((1 + $tanU1*$tanU1)), $sinU1 = $tanU1*$cosU1;
	$sigma1 = atan2($tanU1, $cosAlpha1);
	$sinAlpha = $cosU1 * $sinAlpha1;
	$cosSqAlpha = 1 - $sinAlpha*$sinAlpha;
	$uSq = $cosSqAlpha * ($a*$a - $b*$b) / ($b*$b);
	$A = 1 + $uSq/16384*(4096+$uSq*(-768+$uSq*(320-175*$uSq)));
	$B = $uSq/1024 * (256+$uSq*(-128+$uSq*(74-47*$uSq)));

	$sigma = $s / ($b*$A), $sigmaP = 2*PI;
	while (abs($sigma-$sigmaP) > 1e-12) {
	      $cos2SigmaM = cos(2*$sigma1 + $sigma);
	      $sinSigma = sin($sigma);
	      $cosSigma = cos($sigma);
	      $deltaSigma = $B*$sinSigma*($cos2SigmaM+$B/4*($cosSigma*(-1+2*$cos2SigmaM*$cos2SigmaM)-$B/6*$cos2SigmaM*(-3+4*$sinSigma*$sinSigma)*(-3+4*$cos2SigmaM*$cos2SigmaM)));
	      $sigmaP = $sigma;
	      $sigma = $s / ($b*$A) + $deltaSigma;
    }

	$tmp = $sinU1*$sinSigma - $cosU1*$cosSigma*$cosAlpha1;
	$lat2 = atan2($sinU1*$cosSigma + $cosU1*$sinSigma*$cosAlpha1, 
	      (1-$f)*sqrt($sinAlpha*$sinAlpha + $tmp*$tmp));
	$lambda = atan2($sinSigma*$sinAlpha1, $cosU1*$cosSigma - $sinU1*$sinSigma*$cosAlpha1);
    $C = $f/16*$cosSqAlpha*(4+$f*(4-3*$cosSqAlpha));
	$L = $lambda - (1-$C) * $f * $sinAlpha *
      ($sigma + $C*$sinSigma*($cos2SigmaM+$C*$cosSigma*(-1+2*$cos2SigmaM*$cos2SigmaM)));
	$lon2 = (toRad($lon1)+$L+3*PI)%(2*PI) - PI; # normalise to -180...+180

	$revAz = atan2($sinAlpha, -$tmp);  # final bearing, if required

	return ( "lat" => toDeg($lat2), "lon" => toDeg($lon2), "finalBearing" => toDeg($revAz) );
}
