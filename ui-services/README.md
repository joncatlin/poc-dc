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