
intern Resublist sempty;		

/*
 *  return	0 if no match
 *		>0 if a match
 *		<0 if we ran out of _relist space
 */
intern int
regexec1(Reprog *progp,	
	byte *bol,	
	Resub *mp,	
	int ms,		
	byte *starts,
	byte *eol,
	Rune startchar)
{
	int flag;
	Reinst *inst;
	Relist *tlp;
	byte *s;
	int i, checkstart;
	Rune r, *rp, *ep;
	int n;
	Relist* tl;		
	Relist* nl;
	Relist* tle;		
	Relist* nle;
	int match;
	byte *p;

	flag = 0;
	match = 0;
	checkstart = startchar;
	sempty.m[0].sp = nil;
	if(mp!=nil)
		for(i=0; i<ms; i++)
			mp[i].sp = mp[i].ep = nil;
	_relist[0][0].inst = _relist[1][0].inst = nil;

	
	s = starts;
	do{
		
		if(checkstart){
			p = utfrune(s, startchar);
			if(p == nil)
				return match;
			s = p;
		}
		r = *(byte*)s;
		if(r < Runeself)
			n = 1;
		else
			n = chartorune(&r, s);

		
		tl = _relist[flag];
		tle = _reliste[flag];
		nl = _relist[flag^=1];
		nle = _reliste[flag];
		nl->inst = nil;

		
		if(match == 0){
			sempty.m[0].sp = s;
			_renewthread(tl, progp->startinst, &sempty);
		}

		
		for(tlp=tl; tlp->inst; tlp++){	
			if(s == eol)
				break;

			for(inst = tlp->inst; ; inst = inst->next){
				switch(inst->type){
				case RUNE:	
					if(inst->r == r)
						if(_renewthread(nl, inst->next, &tlp->se)==nle)
							return -1;
					break;
				case LBRA:
					tlp->se.m[inst->subid].sp = s;
					continue;
				case RBRA:
					tlp->se.m[inst->subid].ep = s;
					continue;
				case ANY:
					if(r != '\n')
						if(_renewthread(nl, inst->next, &tlp->se)==nle)
							return -1;
					break;
				case ANYNL:
					if(_renewthread(nl, inst->next, &tlp->se)==nle)
							return -1;
					break;
				case BOL:
					if(s == bol || *(s-1) == '\n')
						continue;
					break;
				case EOL:
					if(r == 0 || r == '\n')
						continue;
					break;
				case CCLASS:
					ep = inst->cp->end;
					for(rp = inst->cp->spans; rp < ep; rp += 2)
						if(r >= rp[0] && r <= rp[1]){
							if(_renewthread(nl, inst->next, &tlp->se)==nle)
								return -1;
							break;
						}
					break;
				case NCCLASS:
					ep = inst->cp->end;
					for(rp = inst->cp->spans; rp < ep; rp += 2)
						if(r >= rp[0] && r <= rp[1])
							break;
					if(rp == ep)
						if(_renewthread(nl, inst->next, &tlp->se)==nle)
							return -1;
					break;
				case OR:
					
					if(_renewthread(tlp, inst->right, &tlp->se) == tle)
						return -1;
					
					continue;
				case END:	
					match = 1;
					tlp->se.m[0].ep = s;
					if(mp)
						_renewmatch(mp, ms, &tlp->se);
					break;
				}
				break;
			}
		}
		checkstart = startchar && nl->inst==nil;
		s += n;
	}while(r);
	return match;
}

extern int
regexec(Reprog *progp,	
	byte *bol,	
	Resub *mp,	
	int ms)		
{
	byte *starts;	
	byte *eol;	
	Rune startchar;
	int rv;

	/*
 	 *  use user-specified starting/ending location if specified
	 */
	starts = bol;
	eol = nil;
	if(mp!=nil && ms>0){
		if(mp->sp)
			starts = mp->sp;
		if(mp->ep)
			eol = mp->ep;
	}
	startchar = 0;
	if(progp->startinst->type == RUNE && progp->startinst->r < Runeself)
		startchar = progp->startinst->r;

	
	for(;;){
		if(_relist[0] == nil){
			_relist[0] = malloc(2*_relistsize*sizeof(Relist));
			_relist[1] = _relist[0] + _relistsize;
			_reliste[0] = _relist[0] + _relistsize - 2;
			_reliste[1] = _relist[1] + _relistsize - 2;
			if(_relist[0] == nil)
				regerror("_relist overflow");
		}
		rv = regexec1(progp, bol, mp, ms, starts, eol, startchar);
		if(rv >= 0)
			return rv;
		free(_relist[0]);
		_relist[0] = nil;
		_relistsize += LISTINCREMENT;
	}
	return -1;
}
