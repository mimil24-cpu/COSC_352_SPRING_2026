import csv,collections
def pf(path):
  rows=list(csv.DictReader(open(path)))
  hd=list(rows[0].keys())
  out=["=== csvprof: "+path+" ===","Columns: "+str(len(hd)),""]
  for h in hd:
    v=[r[h].strip() for r in rows if r[h].strip()]
    nu=len(rows)-len(v)
    nm=[]
    for x in v:
      try: nm.append(float(x))
      except: pass
    out.append("Column: "+h)
    if len(nm)==len(v) and nm:
      sn=sorted(nm);n=len(sn);sm=sum(sn)
      out+=["  Type   : Float" if any("." in x for x in v) else "  Type   : Integer","  Count  : "+str(len(rows)),"  Nulls  : "+str(nu),"  Min    : "+str(sn[0]),"  Max    : "+str(sn[-1]),"  Mean   : {:.2f}".format(sm/n),"  Median : "+str(sn[n//2] if n%2 else (sn[n//2-1]+sn[n//2])/2)]
    else:
      tp=collections.Counter(v).most_common(3)
      out+=["  Type   : Text","  Count  : "+str(len(rows)),"  Nulls  : "+str(nu),"  Unique : "+str(len(set(v)))]
      for x,c in tp: out.append("  Top    : '%s' (%d)"%(x,c))
    out.append("")
  return chr(10).join(out)
for p,r in [("data/bpd_crime.csv","reports/bpd_crime_profile.txt"),("data/311_calls.csv","reports/311_calls_profile.txt")]:
  open(r,"w").write(pf(p));print("wrote "+r)
