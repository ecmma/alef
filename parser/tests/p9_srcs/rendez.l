
void
Rendez.Sleep(Rendez *r, void **bool, int t)
{
	int i;
	void *s;
	Tdb *tdb;
	Task *me, *rh;

	r->lock();
	if(r->t)		
		abort();

	if(bool) {
		i = *(int*)bool;
		if((i&&t) || (!i&&!t)) {
			r->unlock();
			return;
		}
	}

	tdb = PROC.tdb;
	me = tdb->ctask;
	r->t = me;

	tdb->lock();
	r->unlock();

	rh = tdb->runhd;
	if(rh != nil)
		tdb->runhd = rh->link;
	tdb->ctask = rh;
	tdb->sleeper = rh == nil;
	tdb->unlock();

	if(rh == nil)
		rendezvous(tdb, 0);

	s = ALEF_switch(me, tdb->ctask, nil);
	if(s)
		free(s);
}

void
Rendez.Wakeup(Rendez *r)
{
	Task *t;

	if(r == nil)		
		return;

	r->lock();
	t = r->t;
	if(t == nil) {
		r->unlock();
		return;
	}
	r->t = nil;
	r->unlock();

	ALEF_sched(t);
}
