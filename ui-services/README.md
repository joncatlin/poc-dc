# Services for the Digital Communications User Interface


## Postgres commands
The following command should be executed from the root directory of the project. The one where src is!

To build the schema for the database from scratch
```
 psql -h docker01 -p 55432 -U event_user -d test_db  -f sql/schema.sql

```
To populate the database from scratch. Warning this removes ALL data and resets the database
```
psql -h docker01 -p 55432 -U event_user -d test_db  -f sql/populate.sql
```


## TCPDUMP
Use the following command to monitor the TCP traffic into and out of the UI Service container. This helps in determining what is being sent to the service as it is difficult to get the service to log it
```
sudo tcpdump -A -s 0 'tcp port 50013 and (((ip[2:2] - ((ip[0]&0xf)<<2)) - ((tcp[12]&0xf0)>>2)) != 0)'
```