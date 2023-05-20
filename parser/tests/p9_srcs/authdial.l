

int
authdial()
{
	return dial("net!$auth!ticket", nil, nil, nil);
}
