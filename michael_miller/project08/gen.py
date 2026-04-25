import random,csv
random.seed(42)
hoods=["Sandtown-Winchester","Harlem Park","Upton","Druid Heights","Belair-Edison","Cherry Hill","Brooklyn","Curtis Bay","Pigtown","Westport","Park Heights","Pimlico","Govans","Greenmount West","Johnston Square","Oliver","Middle East","Berea","Morrell Park","Irvington","Clifton Park","Eastwood","Pen Lucy","Guilford","Charles Village","Waverly","Homeland","Mount Washington","Roland Park","Hampden"]
crimes=["LARCENY","ROBBERY","ASSAULT","COMMON ASSAULT","BURGLARY","AUTO THEFT","AGG. ASSAULT","SHOOTING","HOMICIDE"]
descs=["Pothole","Graffiti","Abandoned Vehicle","Dead Animal","Vacant Building","Illegal Dumping","Parking Complaint","Street Light Out","Rat Rubout","Dirty Street"]
districts=["NORTHEASTERN","NORTHWESTERN","SOUTHERN","EASTERN","WESTERN","CENTRAL","NORTHERN"]
years=[2019,2020,2021,2022,2023]

with open('data/bpd_crime.csv','w',newline='') as f:
  w=csv.writer(f)
  w.writerow(['CrimeDate','CrimeTime','CrimeCode','Location','Description','Weapon','Post','District','Neighborhood','Longitude','Latitude','Premise','Total Incidents'])
  for i in range(3000):
    h=random.choice(hoods)
    d=random.choice(districts)
    y=random.choice(years)
    mo=random.randint(1,12)
    da=random.randint(1,28)
    hr=random.randint(0,23)
    mn=random.randint(0,59)
    w.writerow([str(mo)+"/"+str(da)+"/"+str(y),str(hr).zfill(2)+":"+str(mn).zfill(2)+":00",str(random.randint(1,9)).zfill(2),str(random.randint(100,9999))+" N BALTIMORE ST",random.choice(crimes),random.choice(["FIREARM","KNIFE","HANDS","NONE"]),str(random.randint(100,900)),d,h,str(round(-76.6+random.random()*0.3,6)),str(round(39.2+random.random()*0.3,6)),random.choice(["STREET","ALLEY","HOUSE","STORE"]),"1"])
print("bpd_crime.csv done")

with open('data/311_calls.csv','w',newline='') as f:
  w=csv.writer(f)
  w.writerow(['SRRecordID','SRStatus','Method Received','ServiceRequest','SRType','SRCreatedDate','SRUpdatedDate','SRDueDate','SRClosedDate','Neighborhood','District','CouncilDistrict','PoliceDistrict','PolicePostCode','Latitude','Longitude','Location'])
  for i in range(5000):
    h=random.choice(hoods)
    d=random.choice(districts)
    y=random.choice(years)
    mo=random.randint(1,12)
    da=random.randint(1,28)
    sr_id="SR-"+str(100000+i)
    status=random.choice(["Closed","Open","In Progress"])
    desc=random.choice(descs)
    lat=str(round(39.2+random.random()*0.3,6))
    lon=str(round(-76.6+random.random()*0.3,6))
    dt=str(mo)+"/"+str(da)+"/"+str(y)
    w.writerow([sr_id,status,"Phone",desc,desc+" Request",dt,dt,dt,dt,h,d,str(random.randint(1,14)),d,str(random.randint(100,900)),lat,lon,lat+", "+lon])
print("311_calls.csv done")
