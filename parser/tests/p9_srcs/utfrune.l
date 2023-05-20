
byte*
utfrune(byte *s, int c)
{
	Rune r;
	int n, c1;

	if(c < Runesync)		
		return strchr(s, c);

	for(;;) {
		c1 = *s;
		if(c1 < Runeself) {	
			if(c1 == 0)
				return nil;
			if(c1 == c)
				return s;
			s++;
			continue;
		}
		n = chartorune(&r, s);
		if(r == c)
			return s;
		s += n;
	}
	return nil;
}
