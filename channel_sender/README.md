# Channel Sender
## Description
This service is responsible for taking a digital communications (DC) request and fulfilling it by doing the following steps for each account in the request:
1.  Obtain the template for the communication, depending on the language preference given for the account
2.  Parse the template to determine what bookmarks are required
3.  Obtain the bookmarks for the specified account for which to send the DC
4.  Render the template with the bookmark data
5.  Send the rendered template to the specified vendor channel
6.  Send an event for each DC, containing the account #, DPL, vendor specific unique message id

## Architecture
### Scalability
Kafka is used as the means by which Channel Sender obtains the requests. There is an environment variable that can be configured so that this service could be replicated and each configured to read messages from different topics. This provides a measure of scalability.

The other way to scale this service is to create one replica for each partition in the Kafka topic. As long as the DC requests placed into the topic are spread evenly across the partitions then this will spread the requests evenly across the replicas. So for a 6 partition topic with 6 replicas we get 6 ChannelSender services operating in parallel. 
### Reliability
The Kafka consumer is configured for high reliaility and hence reads and processes a single request at a time. Should the process fail prior during the operation then the request is left on the Kafka topic and is reprocessed when the service resumes.

The service knows where to resume reading from a topic because it saves its offsets to disk periodically and reads them on startup.
### Performance
With only processing a single request per replica the performance of the solution is limited. There are a few ways to remedy this but they all have limitations. For example proessing requests in parallel within a single replica f the Channel Sender service means that should a failure occur, such as a vendors service is down could cause multiple requests to fail.

# CURRENT ACTIONS
1.  Make the Kafka consuming highly resilient to failures




# TODO
1. Save the parition offset so that if there is a failure it knows where to restart from when the process recovers.
2. Check to ensure that the consumer is configured for high availability correctly.
3. Determine a better way to process requests, such as do several in parallel
4. Determine how to do unit/integration testing of the service
5. Determine what to do if a vendors service is down. Should we hold and resend a number of times or should something else occur?

