
/*
 *	Machine state
 */
Relist*	_relist[2];
Relist*	_reliste[2];
int	_relistsize = LISTINCREMENT;

/*
 *  save a new match in mp
 */
void
_renewmatch(Resub *mp, int ms, Resublist *sp)
{
	int i;

	if(mp==nil || ms<=0)
		return;
	if(mp[0].sp==nil || sp->m[0].sp<mp[0].sp ||
	   (sp->m[0].sp==mp[0].sp && sp->m[0].ep>mp[0].ep)){
		for(i=0; i<ms && i<NSUBEXP; i++)
			mp[i] = sp->m[i];
		for(; i<ms; i++)
			mp[i].sp = mp[i].ep = nil;
	}
}

/*
 * Note optimization in _renewthread:
 * 	*lp must be pending when _renewthread called; if *l has been looked
 *		at already, the optimization is a bug.
 */
Relist*
_renewthread(Relist *lp,	
	Reinst *ip,	
	Resublist *sep)	
{
	Relist *p;

	for(p=lp; p->inst; p++){
		if(p->inst == ip){
			if((sep)->m[0].sp < p->se.m[0].sp)
				p->se = *sep;
			return nil;
		}
	}
	p->inst = ip;
	p->se = *sep;
	(++p)->inst = nil;
	return p;
}

