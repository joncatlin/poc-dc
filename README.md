# POC Digital Communications

## Introduction

This repo investigates the potential aspects of the project so we can gain an understanding of the potential issues and solutions.

## Terminology
Below are some terms that are used throughout the project.
| Name | Meaning | 
| ------------- |-------------|
| Message | Some correspondence that is going to be sent via a specific channel |
| Channel | The type of message being sent, such as email, sms, whatsapp |
| Event | Something that happens to a message, such as it is sent, or it is delivered or it fails |


## Components
Each of the components are stored within a directory of the same name. So webhooks is in a dir called webhooks.

| Component        | Description           | 
| ------------- |-------------|
| webhooks      | Receives callbacks from the sending services. For example when an sms is sent via twilio it has a callback so that Twilio calls when there is an update on the status of the sms, such as delivered.|
| channel_sender | This component sends messages using the various service providers. Currently supported are email, sms and whatsapp. |
| event_store | This component receives the events detailing what messages have been sent and their current status. |

# Editor
## TODO
Below is a list of the things that need to be investigated to understand what solutions are available.
1. Investigate the use of an open source HTML editor such as CKEditor or TinyMCE. Can it easily be integrated into a website that would allow us to generate templates with tags, for each of the individual channels of communication.
2. What is the ability of the CKEditor or other to work in a predefined page with static boxes laid out for envelope etc configuration. Use the @page html attributes to define the boxes and the use inline HTML editor to specify the layout within each box.


# Webhooks
## Description
This component listens on a series of endpoints, one for each of the channels supported. This way it knows what type of channel the event was for because the message received may not contain the channel.

Every message received is formated into a standard event and then placed on a Kafka topic for processing by another component in the solution.

## TODO
1. Figure out how to share the common MessageEvent structure beween all of the three collaborating containers
5. Ensure that correct REST principles are implemented
6. Transform all msgs into a standard format before sending them via Kafka so the responses are normalized
7. Determine if the JSON response for email and whatsapp contain an array and if they do then parse the array breaking each one into its own msg
8. Need to add unit tests for each webhook to ensure the correct response occurs

# Building with rdkafka crate
Using the crate rdkafka requires a change to the standard build process according to the documentation at: https://crates.io/crates/rdkafka see the Installation section of the documentation. Also refer to the docs at https://docs.rs/rdkafka/0.23.1/rdkafka/.

## Change to Cargo.toml to build rdkafka
Add the following line to the dependencies section in the cargo.toml file. Apparently there is a problem using the standard Rust build mechanisms and it needs to be built using CMake.
```
rdkafka = { version = "0.23", features = ["cmake-build"] }
```

## Commands used to install the build env on ubuntu
The Dockerfile should install several dependencies for building rdkafka. These installed using the commands below, which are part of the Dockerfile.
```
sudo apt-get install musl-tools build-essential cmake -y
sudo ln -s /usr/bin/g++ /bin/musl-g++
```
## Installing cmake on windows
Go to the cmnake download page and get the distro suitable for the windows installation. https://cmake.org/download/

# Building with Diesel crate when using postgres database
There are several dependencies when using Diesel for Postgres. The command below installs the dependencies on the operating system. These must be installed prior to building the docker images
```
sudo apt-get update
sudo apt-get install -y postgresql postgresql-contrib libpq-dev
```

## Installing Diesel for postgres

### Install Diesel command line
cargo install diesel_cli --no-default-features --features postgres

# TODO
1. Webhooks may drop messages or not get called. Look into a way on some time period, 
querying all of the messages that are outstanding for a period of time. This would 
close the gap for potentially losing status updates.

# Datastructures
## DB Tables
### Events
This table holds the transactional data about the messages that have been sent and the results obtained through the webhook responses.
struct Event {
    id: String,
    status: String,
    datetime_rfc2822: String,
    event_specific_data: String,
}

CREATE TABLE Event (
   message_id       VARCHAR(20) NOT NULL,
   channel          VARCHAR(15) NOT NULL,
   status           VARCHAR(10) NOT NULL,
   timestamp        TIMESTAMP NOT NULL,
   CONSTRAINT pk_msg_id PRIMARY KEY (message_id, channel)
);

### Accounts
This table holds the mapping of the message id that was sent to the account which ultimately gets back to the DPL the account is in.
struct Account {
    account_id: String, // The unique account identifier
    message_id: String,
    channel: String,
}

CREATE TABLE Account (
   message_id       VARCHAR(20) NOT NULL,
   channel          VARCHAR(15) NOT NULL,
   account_id       VARCHAR(30) NOT NULL,
   CONSTRAINT pk_msg_id PRIMARY KEY (message_id, channel),
   CONSTRAINT uk_account_id UNIQUE (account_id)
);

## Docker Secrets
The solution uses several docker secrets. These are:
| Secret Name | Use | 
| ------------- |-------------|
|sms_vendor_account_id| This maps to the Twilio Account SID
|sms_vendor_token| This maps to the Twilio Authtoken
|email_vendor_token| This maps to the Sendgrid API key

To create the secrets, sign in to a docker swarm manager and use the following command to create each secret:
```
printf <secret> | docker secret create <secret name> -
```
Once completed check that each secret exists by using the following command:
```
docker secret ls
```
To update an existing secret you must remove and then recreate


#  TODO for Production
The following are a list of things that need to get done prior to being close to production ready.
1. Sort out error handling throughout the entire solution. Need to readup the best practices and implement.
2. Apply ON DELETE CASCADE correctly to tables to ensure correct tidyup. Then change the code to take advantage of this.
3. Use transactions for all DB communications to ensure it all succeeds or fails and there are no bits left should something fail
4. Locking for the UI? Must consider locking during all the DB updates. Figure out what the default for the DB is.
5. Unit tests for each element of the solution
6. If there is an error in handlebars, such as a bookmark is not found then error out and do not send a message.
7. Each consumer needs to store the last processed offset so that it can start reding from the last offset on wards. This is due to a failure could cause it to miss msgs 
8. Rather than use the send grid message id, use a custom id generated by conocrd code and sent along with the message in the header. The reason for this is that the send grid message id is not included in allw webhook responses


# Networks Used
The following are the networks being used in the solution. 
| Secret Name | Use | 
| ------------- |-------------|
|digital-comms| Used to communicate amongst the services that are sending the communications. Currently only used for the channel_sender to communicated with the make-pdf service.

All the networks need to exist prior to deploying the stacks. Use the following command to create the networks.
```
docker network create -d overlay <network name>
```
